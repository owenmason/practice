
use practice::rust::playground::*;
use practice::rust::katas::*;
use testo_moddo::call_this_bad_boy;

pub mod testo_moddo {
    pub fn call_this_bad_boy() {
        println!("just a prick");
    }
}

fn main() {
    // println!("Hello, world!");
    playground::testeroni_pizzi();
    let bus_stops = [(10, 0), (3, 5), (5, 8)];
    katas::number_0(&bus_stops);
    call_this_bad_boy();
    playground::closure_val();
    playground::closure_ref();
    playground::closure_mut_ref();
    playground::size_demo();

    katas::digital_root_loop(132189);

    println!("donezo.");
}