use std::cell::RefCell;
use std::rc::Rc;
use robotics_lib::runner::Runner;
mod interoperability_data;
use crate::interoperability_data::init;

use interoperability_data::game_loop;


fn main() {
    let  end: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let operability = interoperability_data::InteroperabilityData::new();
    let mut initializer = init(
        operability.data_pointer,
        50,
        2,
        1,
        0,
        true,
        true,
        10,
        false,
        Some(1),
        end.clone(),
    );
    let run = Runner::new(Box::new(initializer.0), &mut initializer.1).unwrap();

    game_loop(run, end.clone());
}
