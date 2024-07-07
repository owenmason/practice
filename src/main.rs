
use katas;
use practice::rust::playground::*;
use practice::rust::katas::*;
use testo_moddo::call_this_bad_boy;
use device_query::{DeviceQuery, DeviceState, DeviceEvents, Keycode, MouseState};

pub mod testo_moddo {
    pub fn call_this_bad_boy() {
        println!("just a prick");
    }
}

fn run_local() {
    println!("Hello, world!");
    call_this_bad_boy();
}

fn run_playground() {
    playground::testeroni_pizzi();
    playground::closure_val();
    playground::closure_ref();
    playground::closure_mut_ref();
    playground::size_demo();
}

fn run_katas() {
    let bus_stops = [(10, 0), (3, 5), (5, 8)];
    katas::number_0(&bus_stops);
    katas::digital_root_loop(132189);
    katas::string_rev("Friday");
    println!("{}", katas::spin_words("The crazy fox JumP3d ov3r the crazy STRAAAW hay bale!!"));
    println!("{}", katas::spin_words("Just kidding there is still one more"));
}

fn do_loopies() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    let keys: Vec<Keycode> = device_state.get_keys();

    println!("Mouse coordinates {:?}", mouse.coords);
    println!("Is A pressed? {}", keys.contains(&Keycode::A));

    let _guard = device_state.on_mouse_move(|pos| {
        println!("mouse position: {:#?}", pos);
    });

    let _guard = device_state.on_mouse_down(|btn| {
        println!("mouse button down: {:#?}", btn);
    });

    let _guard = device_state.on_mouse_up(|btn| {
        println!("mouse button up: {:#?}", btn);
    });

    let _guard = device_state.on_key_down(|key| {
        println!("key down: {:#?}", key);
        match key {
            Keycode::A | Keycode::S | Keycode::D | Keycode::W => println!("move around..."),
            Keycode::F => println!("{}", playground::play_with_match()),
            _ => (),
        }
    });

    let _guard = device_state.on_key_up(|key| {
        println!("key up: {:#?}", key);
    });

    loop {
        let mouse: MouseState = device_state.get_mouse();
        let keys: Vec<Keycode> = device_state.get_keys();

        for c in keys {
            match c {
                Keycode::Escape | Keycode::Enter => return,
                _ => continue,
            }
        }
    }
}

fn main() {
    // run_local();
    // run_playground();
    // run_katas();

    //println!("{}", '\u{1f577}'.to_string()); //why the fuck can't it display this properly???

    //do_loopies();

    katas::is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']);

    println!("donezo.");
}