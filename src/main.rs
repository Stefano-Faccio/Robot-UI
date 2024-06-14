
mod interoperability_data;
mod data_type;

use crate::data_type::{InitData, SmartCell};
use crate::interoperability_data::{InteroperabilityData, init, game_loop};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket_dyn_templates::{context, Template};
use rocket::State;
use robotics_lib::runner::{Runner};
use rocket::response::status;
use serde_json::json;
use tokio::time::{interval, Duration};
use rocket::futures::{SinkExt, StreamExt};
use rand::{SeedableRng, RngCore, rngs};
use tokio::sync::Mutex;
use crate::rngs::StdRng;

//This is the main function that starts the server
#[launch]
fn rocket() -> _ {
    //Seed init: this is a presentation, so we use a fixed seed
    let seed: u64 = 2u64;
    //RNG
    let rng:Arc<Mutex<StdRng>> = Arc::new(Mutex::new(StdRng::seed_from_u64(seed)));
    //Init world configuration data
    let init_conf:InitData = InitData::new();
    //Interoperability data between the robot and the ui server
    let run_data:InteroperabilityData = InteroperabilityData::new();

    rocket::build()
        // put the data in the server state
        .manage(init_conf)
        .manage(run_data)
        .manage(rng)
        // add templating system
        .attach(Template::fairing())
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![index_page, start_robot, websocket])
}


#[get("/")]
//Genero la pagina iniziale
async fn index_page(init_conf: &State<InitData>) -> Template {
    //Return the only page that we have
    // This data is used to generate the page
    let grid_ui_dim = init_conf.data.lock().await.grid_ui_dim;
    let cells: Vec<usize> = (0..(grid_ui_dim * grid_ui_dim)).collect();
    let backpack = ["Tree", "Bush", "Rock", "Water", "Fish", "Crate", "Scarecrow", "Bin", "Garbage", "Fire","JollyBlock" ,"Coin", "Bank", "Market", "Building"];
    //Render the page
    Template::render("root", context! {grid_size: grid_ui_dim, cells: cells, backpack: backpack, backpack_size: backpack.len()})
}

#[post("/start")]
//Start the robot: this is a POST request
async fn start_robot(_init_conf: &State<InitData>, _run_data: &State<InteroperabilityData>, rng: &State<Arc<Mutex<StdRng>>>) -> status::Accepted<String> {

    //Check if a game is already started
    return if !_run_data.data_pointer.lock().unwrap().has_started
    {
        //Generate the next seed
        let new_seed = rng.lock().await.next_u64();

        //Generate the new configuration
        _init_conf.data.lock().await.grid_ui_dim = 26;
        _init_conf.data.lock().await.dimension = 100;
        _init_conf.data.lock().await.n_mari = 1;
        _init_conf.data.lock().await.n_prati = 1;
        _init_conf.data.lock().await.n_deserti = 1;
        _init_conf.data.lock().await.fully_connected = true;
        _init_conf.data.lock().await.remove_street = false;
        _init_conf.data.lock().await.n_citta = 10;
        _init_conf.data.lock().await.flat = false;
        _init_conf.data.lock().await.seed = Some(new_seed);

        //Print the seed
        //println!("Seed: {:?}", _init_conf.data.lock().await.seed);
        eprintln!("Seed: {:?}", _init_conf.data.lock().await.seed);

        //Copy the configurations to pass it to the thread
        let init_conf = _init_conf.data.lock().await.clone();
        let arc_clone = _run_data.data_pointer.clone();

        //Create a new thread to run the robot
        let _handle = thread::spawn(move || {
            let end: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
            let mut initializer = init(arc_clone, init_conf.dimension,
                                       init_conf.n_mari,init_conf.n_prati,init_conf.n_deserti,
                                       init_conf.fully_connected,init_conf.remove_street,
                                       init_conf.n_citta,init_conf.flat,init_conf.seed, end.clone());
            let run = Runner::new( Box::new(initializer.0), &mut initializer.1).unwrap();
            game_loop(run, end.clone());
        });

        //JSON response
        status::Accepted(json!({
            "Success": true,
            "Message": format!("New seed: {}", new_seed)
        }).to_string())
    }
    else
    {
        status::Accepted(json!({
            "Success": false,
            "Message": "Game already started"
        }).to_string())
    }
}

#[get("/ws")]
//Websocket connection
async fn websocket(ws: ws::WebSocket, init_conf: &State<InitData>, _run_data: &State<InteroperabilityData>) -> ws::Channel<'static> {
    //Create a clone of the data to pass to the thread
    let arc_clone = _run_data.inner().data_pointer.clone();
    //Get the grid dimension and the grid ui dimension
    let grid_ui_dim = init_conf.data.lock().await.grid_ui_dim;
    let grid_dim = init_conf.data.lock().await.dimension;

    ws.channel(move |mut stream: ws::stream::DuplexStream| {
        Box::pin(async move {
            //Spawn a new task to send data to the client
            tokio::spawn(async move {
                //Wait 500millisec
                tokio::time::sleep(Duration::from_millis(500)).await;
                //Every 250ms send the data to the client
                let mut interval = interval(Duration::from_millis(250));
                let mut close_frame = ws::frame::CloseFrame {
                    code: ws::frame::CloseCode::Normal,
                    reason: "Robot finished".to_string().into(),
                };
                //Select the first task that completes between the interval and the stream
                loop {
                    tokio::select! {
                        //If the interval ticks
                        //Send the data to the client
                        _ = interval.tick() => {
                            //Clone the data to send to the client
                            //The world, the robot position, the backpack and the energy
                            let run_data = {
                                let my_data = arc_clone.lock().unwrap();
                                my_data.clone()
                            };

                            //The world is much bigger than the grid dimension
                            //I have to send to the client only the cells that it can see
                            //Get the robot position
                            let mut robot_x = run_data.coordinate.0 as i32;
                            let mut robot_y = run_data.coordinate.1 as i32;
                            //Compute the start and end of the ui grid
                            let mut start_x = robot_x - grid_ui_dim as i32/2;
                            let mut start_y = robot_y - grid_ui_dim as i32/2;
                            let mut end_x = robot_x + grid_ui_dim as i32/2;
                            let mut end_y = robot_y + grid_ui_dim as i32/2;

                            //Check if the start and end are inside the grid
                            if start_x < 0 {
                                start_x = 0;
                                end_x = grid_ui_dim as i32;
                            }
                            if start_y < 0 {
                                start_y = 0;
                                end_y = grid_ui_dim as i32;
                            }
                            if end_x > grid_dim as i32 {
                                end_x = grid_dim as i32;
                                start_x = grid_dim as i32 - grid_ui_dim as i32;
                            }
                            if end_y > grid_dim as i32 {
                                end_y = grid_dim as i32;
                                start_y = grid_dim as i32 - grid_ui_dim as i32;
                            }
                            //Compute the robot position in the ui grid
                            robot_x = robot_x - start_x;
                            robot_y = robot_y - start_y;

                            //Create a vector of SmartCell to send to the client only the cells that it can see
                            let mut world:Vec<SmartCell> = Vec::new();
                            if run_data.world.len() != 0 {
                                for i in start_x as usize .. end_x as usize {
                                    for j in start_y as usize .. end_y as usize {
                                        if !run_data.world[i][j].is_none() {
                                            let tile = run_data.world[i][j].clone().unwrap();
                                            let cell_id = (i-start_x as usize) * grid_ui_dim + (j-start_y as usize);
                                            let cell = SmartCell {
                                                id: cell_id,
                                                tile_type: tile.tile_type,
                                                elevation: tile.elevation,
                                                content: tile.content};
                                            world.push(cell);
                            }}}}
                            //Clone the backpack to send to the client
                            let mut backpack:HashMap<String, usize> = HashMap::new();
                            for (key, value) in run_data.backpack.iter() {
                                backpack.insert(key.to_string(), *value);
                            }
                            //Create the json data to send to the client
                            let my_json_data = json!({
                                    "world" : world,
                                    "energy" : run_data.energy,
                                    "test_string" : run_data.test_string,
                                    "ui_coordinate" : (robot_x, robot_y),
                                    "grid_coordinate" : (run_data.coordinate.0, run_data.coordinate.1),
                                    "backpack" : backpack,
                                }).to_string();
                            //Send the data to the client
                            let _ = stream.send(ws::Message::Text(my_json_data)).await;
                            //Check if the robot has finished (and started) -> end the connection
                            //println!("Started: {}, Ended: {}", run_data.has_started, run_data.has_ended);
                            if (!run_data.has_started) && (run_data.has_ended) {
                                println!("Robot finish!");
                                //Close connection
                                break;
                            }
                        }
                        // If the client sends a message
                        Some(Ok(message)) = stream.next() => {
                            match message {
                                // text message: print it
                                ws::Message::Text(text) => {
                                    println!("Received Text message: {}", text);
                                }
                                // close message: close the connection
                                ws::Message::Close(close_mex) => {
                                    println!("Received Close message: {:?}", close_mex);
                                    close_frame.reason = "Client ask to close connection".to_string().into();
                                    close_frame.code = ws::frame::CloseCode::Normal;
                                    break;
                                }
                                // other message: print it
                                _ => {
                                    println!("Received other message: {:?}", message);
                                }
                            }
                        }
                        // If the client disconnects
                        else => {
                            close_frame.reason = "Client disconnected".to_string().into();
                            close_frame.code = ws::frame::CloseCode::Error;
                            break;
                        }
                    }
                }

                // Close the connection
                let _ = stream.close(Some(close_frame)).await;
                println!("Connection closed");
            });

            tokio::signal::ctrl_c().await.unwrap();
            Ok(())
        })
    })
}