use std::sync::Arc;
use robotics_lib::world::tile::{Content, TileType};
use serde::Serialize;
use tokio::sync::Mutex;

// Data structure that represents the initial configuration of the game
#[derive(Clone)]
pub struct _InitData {
    pub grid_ui_dim:usize,
    pub dimension:usize,
    pub n_mari:i32,
    pub n_prati:i32,
    pub n_deserti:i32,
    pub fully_connected:bool,
    pub remove_street:bool,
    pub n_citta:i32,
    pub flat:bool,
    pub seed: Option<u64>
}
impl _InitData {
    pub fn new() -> _InitData {
        _InitData {
            grid_ui_dim: 26,
            dimension: 100,
            n_mari: 1,
            n_prati: 1,
            n_deserti: 1,
            fully_connected: true,
            remove_street: false,
            n_citta: 10,
            flat: false,
            seed: Some(rand::random::<u64>())
        }
    }
}

// Data structure that wraps the initial configuration of the game so that
// it can be shared between threads and in the state of the web server
#[derive(Clone)]
pub struct InitData {
    pub data: Arc<Mutex<_InitData>>
}
impl InitData{
    pub fn new() -> InitData {
        InitData {
            data: Arc::new(Mutex::new(_InitData::new()))
        }
    }
}

//-----------------------------------------------------------------------------------------------

// Data structure that represents a cell in the grid
// It is used to send the grid to the frontend in a more readable way
#[derive(Clone, Serialize)]
pub struct SmartCell
{
    pub id: usize,
    pub tile_type: TileType,
    pub elevation: usize,
    pub content: Content,
}

//-----------------------------------------------------------------------------------------------