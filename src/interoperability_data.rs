use core::time;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

use op_map;
use op_map::op_pathfinding::get_best_action_to_element;
use op_map::op_pathfinding::OpActionInput;
use op_map::op_pathfinding::OpActionOutput;
use op_map::op_pathfinding::ShoppingList;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::{self};

use recycle_by_ifrustrati::tool::recycle;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, go, teleport, Direction};
use robotics_lib::interface::{put, robot_map};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Tree;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;

use rustici_planner;
use rustici_planner::tool::Destination;
use rustici_planner::tool::PlannerError;
use rustici_planner::tool::PlannerResult;
use rustici_planner::tool::{Action, Planner};

use rip_worldgenerator;
use rip_worldgenerator::MyWorldGen;

use bob_lib::tracker::Goal;
use bob_lib::tracker::GoalTracker;
use bob_lib::tracker::GoalType;

use recycle_by_ifrustrati;

use cargo_commandos_lucky::{self, lucky_function};
pub struct MyRobot {
    robot: Robot,
    tracker: GoalTracker,
    status: usize,
    resting: usize,
    last_move: Direction,
    money_gambled: usize,
    end: Rc<RefCell<bool>>,
    interoperability_data: Arc<Mutex<AllData>>,
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {

        if self.resting > 0 {
            self.resting -= 1;
            println!("I'm resting");
        } else {
            let f_bank = Goal::new(
                "find_bank".to_string(),
                "find a bank".to_string(),
                GoalType::PutOutFire,
                Some(Content::Fire),
                1,
            );

            let job_goal = Goal::new(
                "gather_wood".to_string(),
                "gather at least 10 wood".to_string(),
                GoalType::GetItems,
                Some(Content::Tree(0)),
                11 - *self
                    .get_backpack()
                    .get_contents()
                    .get(&Content::Garbage(0))
                    .unwrap() as u32,
            );

            match self.status {
                0 => {
                    //println!("FINDING A BANK");
                    //FIND A BANK
                    if self.tracker.get_goals().is_empty() {
                        self.tracker.add_goal(f_bank);
                    } else {
                        find_bank(self, world);
                    }
                }
                1 => {
                    //DO A JOB (Gather wood)

                    if self.tracker.get_goals().is_empty() {
                        self.tracker.add_goal(job_goal);
                    } else {
                        //println!("GATHERING WOOD");
                        let obj = *self.tracker.get_goals()[0].get_items_left();
                        get_wood(obj, self, world);
                    }
                }
                2 => {
                    //println!("RECYCLING WOOD");
                    //Recycle wood to get money
                    thread::sleep(time::Duration::from_millis(500));
                    let rec_res = recycle(self, 0);
                    match rec_res {
                        Ok(coins) => println!("Earned {} coins", coins),
                        Err(e) => println!("Error - {:?}", e),
                    }
                    match self.get_backpack().get_contents().get(&Content::Coin(0)) {
                        //check if 10 coin are reached
                        Some(10) => {
                            self.status = 3;
                        }
                        Some(v) => {
                            if v < &10 {
                                self.status = 1;
                            } else {
                                self.status = 3;
                            }
                        }
                        None => {
                            *self.end.borrow_mut() = true;
                            println!("ERROR - NO COIN AFTER RECYCLE")
                        }
                    }
                }
                3 => {
                    //println!("DISPOSING OF TRASH IF NECESSARY");
                    // if some garbage is left, search for a bin and put it there, otherwise put into empty tile
                    match self.get_backpack().get_contents().get(&Content::Garbage(0)) {
                        // if some garbage is left, search for a bin and put it there, otherwise put into empty tile
                        Some(a) => {
                            if *a > 0 {
                                find_bin(self, world, *a);
                            } else {
                                self.status = 4;
                            }
                        }
                        None => {
                            println!("ERROR - NO BACKPACK FOUND");
                            *self.end.borrow_mut() = true;
                        }
                    }
                }

                4 => {
                    //println!("Go Gambling");
                    let mut res = false;
                    if self.tracker.get_goals().is_empty() {
                        self.tracker.add_goal(f_bank);
                    } else {
                        res = find_bank(self, world);
                    }
                    if res {
                        self.status = 5;
                    } else {
                        self.status = 4;
                    }
                }

                5 => {
                    //println!("GAMBLING");
                    //GAMBLE (end when al least 30 money are spend or bank is full/error when try to pay)
                    if self.get_backpack().get_contents().get(&Content::Coin(0)) >= Some(&2) {
                        thread::sleep(time::Duration::from_millis(300));
                        match put(self, world, Content::Coin(0), 2, self.last_move.clone()) {
                            Ok(x) => {
                                //println!("spent {} coins", x);
                                self.money_gambled += 2;
                                if x == 0 {
                                    //println!("Bank is full, I'm done");
                                    //*self.end.borrow_mut() = true;
                                    self.status = 4;
                                }
                            }
                            Err(e) => {
                                println!("ERROR: {:?}", e);
                                *self.end.borrow_mut() = true;
                            }
                        }
                        match lucky_function::lucky_spin(&mut self.robot) {
                            Ok(x) => {
                                println!("{}", x);
                            }
                            Err(_) => {
                                println!("Slot MAchine is Broken");
                                *self.end.borrow_mut() = true;
                            }
                        }
                    } else {
                        //println!("Back to work");
                        self.status = 1;
                    }
                    if self.money_gambled >= 30 {
                        println!("GAMBLED ENOUGH MONEY FOR NOW!");
                        *self.end.borrow_mut() = true;
                    }
                }
                _ => {
                    *self.end.borrow_mut() = true;
                }
            }
        }
        modify_data(self, world);
    }
    fn handle_event(&mut self, _event: Event) {
        // println!();
        // println!("{:?}", event);
        //println!();
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

pub fn find_bank(rob: &mut MyRobot, world: &mut World) -> bool {
    //go to a bank or explore until find one
    let destination = Destination::go_to_content(Content::Bank(0..20));
    let result = Planner::planner(rob, destination, world);
    match result {
        Ok(planner_result) => match planner_result {
            PlannerResult::Path((actions, energy_cost)) => {
                //println!("A bank has been found");
                if energy_cost > rob.get_energy().get_energy_level() {
                    rob.resting = 2;
                    return false;
                } else {
                    let mut last_move = Direction::Up;
                    for a in actions {
                        thread::sleep(time::Duration::from_millis(200));
                        match a {
                            Action::Move(dir) => {
                                last_move = dir.clone();
                                let _ = go(rob, world, dir);
                            }
                            Action::Teleport((x, y)) => {
                                let _ = teleport(rob, world, (x, y));
                            }
                        }
                        modify_data(rob, world);
                    }

                    match last_move {
                        Direction::Up => {
                            match go(rob, world, Direction::Down) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        return false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Up;
                        }
                        Direction::Down => {
                            match go(rob, world, Direction::Up) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        return false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Down;
                        }
                        Direction::Right => {
                            match go(rob, world, Direction::Left) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        return false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Right;
                        }
                        Direction::Left => {
                            match go(rob, world, Direction::Right) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        return false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Left;
                        }
                    }
                    rob.tracker.remove_goal("find_bank");
                    rob.status = 1;
                    //println!("A bank has been reached");
                    rob.last_move = last_move;
                    return true;
                }
            }
            PlannerResult::RadiusExplored => {
                println!("The portion of map with the specified radius has been explored!");
                return false;
            }
            PlannerResult::MapAllExplored => {
                *rob.end.borrow_mut() = true;
                println!("The map has all been explored!");
                return false;
            }
        },
        Err(error) => match error {
            PlannerError::NoContent => {
                //println!("No bank found, searching for one");
                trekking(rob, world);
                return false;
            }
            other_error => {
                panic!("Error!! {:?}", other_error);
            }
        },
    }
}

pub fn trekking(rob: &mut MyRobot, world: &mut World) {
    //println!("Start Trekking");
    //set destination to explore the unknown map
    let max_energy = rob.get_energy().get_energy_level() / 2 ;
    let radius = 50;
    let destination = Destination::Unknown((max_energy, radius));
    let result = Planner::planner(rob, destination, world);

    match result {
        Ok(planner_result) => match planner_result {
            PlannerResult::Path((_actions, _energy_cost)) => {
                //println!("Actions: {:?}", actions);
                //println!("Energy Cost: {}", energy_cost);
            }
            PlannerResult::MapAllExplored => {
                *rob.end.borrow_mut() = true;
                println!("The map has all been explored!");
            }
            PlannerResult::RadiusExplored => {
                println!("The portion of map with the specified radius has been explored!");
            }
            
        },
        Err(error) => {
            println!("Error: {:?}", error);
            match error {
                PlannerError::RoboticLibError(LibError::NotEnoughEnergy) => {
                    rob.resting = 3;
                }
                PlannerError::RestOfMapIsUnreachable => {
                    if rob.status == 3 {
                        //println!("start disposal protocol");
                        dispose_garbage(rob, world)
                    } else {
                        *rob.end.borrow_mut() = true
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn get_wood(obj: u32, rob: &mut MyRobot, world: &mut World) {
    let mut backp: HashMap<Content, usize> = rob.get_backpack().get_contents().clone();
    let mut w_before = 0;
    match backp.get(&Tree(0)) {
        Some(quant) => w_before = *quant,
        None => {}
    }
    if obj > 0 {
        let mut shopping_list = ShoppingList {
            list: vec![(Content::Tree(obj as usize), Some(OpActionInput::Destroy()))],
        };
        match get_best_action_to_element(rob, world, &mut shopping_list) {
            None => {
                trekking(rob, world);
            }
            Some(next_action) => {
                match next_action {
                    OpActionOutput::Move(dir) => {
                        let _ = go(rob, world, dir);
                        modify_data(rob, world);
                        thread::sleep(time::Duration::from_millis(50));
                    }
                    OpActionOutput::Destroy(dir) => {
                        // println!("Destroy");
                        let _ = destroy(rob, world, dir);
                    }
                    OpActionOutput::Put(c, u, d) => {
                        let _ = put(rob, world, c, u, d);
                    }
                }
            }
        }
        let mut w_after = 0;
        backp = rob.get_backpack().get_contents().clone();
        match backp.get(&Tree(0)) {
            Some(quant) => w_after = *quant,
            None => {}
        }
        let gathered: usize = w_after - w_before;
        if gathered > 0 {
            rob.tracker
                .update_manual(GoalType::GetItems, Some(Content::Tree(0)), gathered);
        }
    } else {
        rob.tracker.remove_goal("gather_wood");
        rob.status = 2;
    }
}

pub fn find_bin(rob: &mut MyRobot, world: &mut World, garb: usize) {
    let destination = Destination::go_to_content(Content::Bin(0..20));
    let result = Planner::planner(rob, destination, world);
    match result {
        Ok(planner_result) => match planner_result {
            PlannerResult::Path((actions, energy_cost)) => {
                println!("A bin has been reached");
                if energy_cost > rob.get_energy().get_energy_level() {
                    rob.resting = 3;
                } else {
                    let mut positioning = true;
                    let mut last_move = Direction::Up;
                    for a in actions {
                        thread::sleep(time::Duration::from_millis(200));
                        match a {
                            Action::Move(dir) => {
                                last_move = dir.clone();
                                let _ = go(rob, world, dir);
                            }
                            Action::Teleport((x, y)) => {
                                let _ = teleport(rob, world, (x, y));
                            }
                        }
                        modify_data(rob, world);
                    }

                    match last_move {
                        Direction::Up => {
                            match go(rob, world, Direction::Down) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Up;
                        }
                        Direction::Down => {
                            match go(rob, world, Direction::Up) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Down;
                        }
                        Direction::Right => {
                            match go(rob, world, Direction::Left) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Right;
                        }
                        Direction::Left => {
                            match go(rob, world, Direction::Right) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Left;
                        }
                    }
                    if !positioning {
                        rob.resting = 1;
                    } else {
                        match put(rob, world, Content::Garbage(0), garb, last_move) {
                            Ok(_t) => {
                                //println!("OK {}", t)
                            }
                            Err(e) => match e {
                                LibError::NotEnoughEnergy => {
                                    rob.resting = 2;
                                }
                                _ => {
                                    println!("{:?}", e)
                                }
                            },
                        }
                    }
                }
            }
            _ => {}
        },
        Err(error) => {
            println!("Error: {:?}", error);
            match error {
                PlannerError::RoboticLibError(LibError::NotEnoughEnergy) => {
                    rob.resting = 3;
                }
                PlannerError::RestOfMapIsUnreachable => dispose_garbage(rob, world),
                PlannerError::NoContent => trekking(rob, world),
                PlannerError::Unreachable => dispose_garbage(rob, world),
                _ => {}
            }
        }
    }
}

pub fn dispose_garbage(rob: &mut MyRobot, world: &mut World) {
    let obj = *rob
        .get_backpack()
        .get_contents()
        .get(&Content::Garbage(0))
        .unwrap();
    let destination = Destination::go_to_content(Content::None);
    let result = Planner::planner(rob, destination, world);
    match result {
        Ok(planner_result) => match planner_result {
            PlannerResult::Path((actions, energy_cost)) => {
                if energy_cost > rob.get_energy().get_energy_level() {
                    rob.resting = 3;
                } else {
                    let mut positioning = true;
                    let mut last_move = Direction::Up;
                    for a in actions {
                        thread::sleep(time::Duration::from_millis(200));

                        match a {
                            Action::Move(dir) => {
                                last_move = dir.clone();
                                let _ = go(rob, world, dir);
                            }
                            Action::Teleport((x, y)) => {
                                let _ = teleport(rob, world, (x, y));
                            }
                        }
                        modify_data(rob, world);
                    }

                    match last_move {
                        Direction::Up => match go(rob, world, Direction::Down) {
                            Ok(_t) => {
                                //println!("OK {:?}", t)
                            }
                            Err(e) => match e {
                                LibError::NotEnoughEnergy => {
                                    rob.resting = 2;
                                    positioning = false;
                                }
                                _ => println!("{:?}", e),
                            },
                        },
                        Direction::Down => {
                            match go(rob, world, Direction::Up) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Down;
                        }
                        Direction::Right => {
                            match go(rob, world, Direction::Left) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Right;
                        }
                        Direction::Left => {
                            match go(rob, world, Direction::Right) {
                                Ok(_t) => {
                                    //println!("OK {:?}", t)
                                }
                                Err(e) => match e {
                                    LibError::NotEnoughEnergy => {
                                        rob.resting = 2;
                                        positioning = false;
                                    }
                                    _ => println!("{:?}", e),
                                },
                            }
                            last_move = Direction::Left;
                        }
                    }
                    if !positioning {
                        rob.resting = 1;
                    } else {
                        match put(rob, world, Content::Garbage(0), obj, last_move) {
                            Ok(_t) => {
                                //println!("garbage disposed");
                            }
                            Err(e) => match e {
                                LibError::NotEnoughEnergy => {
                                    rob.resting = 2;
                                }
                                _ => {
                                    println!("{:?}", e)
                                }
                            },
                        }
                    }
                }
            }
            PlannerResult::RadiusExplored => {}
            PlannerResult::MapAllExplored => {
                println!("The map has all been explored!");
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
            match error {
                PlannerError::RoboticLibError(LibError::NotEnoughEnergy) => {
                    rob.resting = 3;
                }
                PlannerError::RestOfMapIsUnreachable => {
                    *rob.end.borrow_mut() = true;
                }
                PlannerError::NoContent => {
                    println!("No bin, leave into nature");
                    dispose_garbage(rob, world)
                }
                PlannerError::Unreachable => {
                    println!("No bin reacheble, leave into nature");
                    dispose_garbage(rob, world)
                }
                _ => {}
            }
        }
    }
}
//------------------------------------------------------------------------
#[derive(Debug)]
pub struct InteroperabilityData {
    pub data_pointer: Arc<Mutex<AllData>>,
}
impl InteroperabilityData {
    pub fn new() -> Self {
        Self {
            data_pointer: Arc::new(Mutex::new(AllData::new())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllData {
    pub world: Vec<Vec<Option<Tile>>>,
    pub energy: u32,
    pub test_string: String,
    pub coordinate: (u32, u32), //type coordiante exist but cannot initialize outside the common crate
    pub backpack: HashMap<Content, usize>,
    pub has_ended: bool,
    pub has_started: bool,
    pub money_gambled: usize, /*
                              pub weather: WeatherType,
                              pub time: DayTime,
                              */
}

impl AllData {
    pub fn new() -> Self {
        Self {
            world: Vec::new(),
            energy: 0u32,
            test_string: "Mi chiamano Trinità".to_string(),
            coordinate: (0, 0),
            backpack: HashMap::new(),
            has_ended: false,
            has_started: false,
            money_gambled: 0,
        }
    }

    fn update_ui(&mut self, world: &mut World) {
        self.world = robot_map(world).unwrap();
    }
}

//---------------------------------------------

pub fn init(
    interoperability_data: Arc<Mutex<AllData>>,
    dim: usize,
    sea: i32,
    garden: i32,
    desert: i32,
    connect: bool,
    remove_s: bool,
    city: i32,
    flat: bool,
    seed: Option<u64>,
    end: Rc<RefCell<bool>>,
) -> (MyRobot, MyWorldGen) {
    //initialize world generator and robot
    let generator = rip_worldgenerator::MyWorldGen::new_param(
        dim, sea, garden, desert, connect, remove_s, city, flat, seed,
    );
    let rob = MyRobot {
        robot: Robot::new(),
        tracker: GoalTracker::new(),
        status: 0, // 0 => find bank, 1 => get wood 2 => go gambling
        resting: 0,
        interoperability_data,
        end: end,
        last_move: Direction::Up,
        money_gambled: 0,
    };
    return (rob, generator);
}

//---------------------------------------------------

pub fn game_loop(mut run: Runner, end: Rc<RefCell<bool>>) -> Runner {
    'running: loop {
        match run.game_tick() {
            Ok(_x) => {
                if *end.borrow() == true {
                    //if End.as_ptr() == true {
                    break 'running;
                }
            }
            Err(e) => {
                println!("Error during game_ tick{:?}", e);
                break 'running;
            }
        }
    }
    return run;
}
//-------------------------------------------------------------
fn modify_data(rob: &mut MyRobot, world: &mut World) {
    let mut data = rob.interoperability_data.lock().unwrap();
    if !data.has_started {
        data.has_started = true;
    }
    data.energy = rob.get_energy().get_energy_level() as u32;
    data.update_ui(world);
    data.backpack = rob.get_backpack().get_contents().clone();
    data.coordinate = (
        rob.get_coordinate().get_row() as u32,
        rob.get_coordinate().get_col() as u32,
    );
    data.has_ended = *rob.end.borrow();
    data.money_gambled = rob.money_gambled;
    match rob.status {
        0 => {
            data.test_string = "FINDING A BANK".to_string();
        }
        1 => data.test_string = "GATHERING WOOD".to_string(),
        2 => data.test_string = "RECYCLING WOOD".to_string(),
        3 => data.test_string = "DISPOSING EXTRA GARBAGE".to_string(),
        4 => data.test_string = "APPROACHING THE SLOT".to_string(),
        5 => data.test_string = "GAMBLING".to_string(),
        _ => data.test_string = "UNNATURAL STATUS".to_string(),
    }
    if data.has_ended {
        data.test_string = "FULFILLED MY PURPOSE".to_string();
        data.has_started = false;
    }

    //data.weather = world.environmental_conditions;
}
