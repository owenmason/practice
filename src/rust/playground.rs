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

    pub fn closure_val() {
        let x = 5;
        let capture_by_value = move |y| x + y;
    
        // x is moved into the closure
        dbg!("The result is: {}", capture_by_value(3));
    }

    pub fn closure_ref() {
        let x = 5;
        let capture_by_reference = |y| x + y;
    
        // x is borrowed by the closure
        dbg!("The result is: {}", capture_by_reference(3));
        dbg!("x is still accessible: {}", x);
    }

    pub fn closure_mut_ref() {
        let mut x = 5;
        {
            let mut capture_by_mutable_reference = |y| x += y;
            capture_by_mutable_reference(3);
        }
    
        // x is modified by the closure
        dbg!("x is now: {}", x);
    }

    pub fn size_demo() {    
        enum Foo1 { A, B }
        dbg!("{}", std::mem::size_of::<Foo1>());
        enum Foo2 { A(u8), B(u8) }
        dbg!("{}", std::mem::size_of::<Foo2>());
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
