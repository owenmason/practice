use std::thread;

pub mod playground {
    pub fn testeroni_pizzi() {
        println!("Hello, world2222222!");
        let strval = String::from("juxtaposed phalanges");
        println!("{}", strval);
        //let djinn = !vec[1,2,3,4,5,6,7,8,9,10];

        // let closure = |x| x;
        // let s = closure(String::from("tadpoles"));
        // let n = closure(33);
        // let t1 = thread::spawn();
    }

    pub fn stubby() {
        println!("can we do this??");
    }

    enum Colors {
        Red,
        Green,
        Blue
    }

    struct Task {
        task_id: u32,
        color: Colors
    }
}
