use std::thread;
use rand::seq::SliceRandom;
use rand::seq::index::sample;
use std::io;
use regex::Regex;

pub mod playground {
    pub fn testeroni_pizzi() {
        println!("Hello, world2222222!");
        let strval = String::from("juxtaposed phalanges");
        println!("{}", strval);
        //let djinn = !vec[1,2,3,4,5,6,7,8,9,10];

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

    pub fn is_fibonacci_subset(n: i32) -> bool {
        match n {
            1 | 2 | 3 | 5 | 8 => return true,
            _ => return false,
        }
    }

    struct fibonacci_subset {
        vals: i32,
    }

    pub fn get_user_input(s: &str) -> String {
        println!("{}-> ", s);
        let mut buffer = String::new();
        //let stdin = io::stdin().read_line(&mut buffer)?;
        return buffer;
    }

    pub fn play_with_match() -> String {
        let n: i32 = 6;
        let funky: Vec<char> = "😀😇😎🤪🕷️🧙🎲🐍".chars().collect(); //'\u{1f577}''\u{1f600}''\u{1f607}''\u{1f60e}''\u{1f92a}''\u{1f577}''\u{fe0f}''\u{1f9d9}''\u{1f3b2}''\u{1f40d}'        

        let rand_indexes = rand::seq::index::sample(&mut rand::thread_rng(), funky.len(), 1);
        let mut funky_rand: Vec<char> = Vec::new();
        for index in rand_indexes {
            funky_rand.push(funky.get(index).copied().unwrap());
        }
        match n {
            i32::MIN..=-1 => return String::from(format!("{} is pretty low", n)),
            0 => return String::from(format!("unsurprisingly, the number of bits in i32 is {}", i32::BITS)),
            1 | 2 | 3 | 5 | 8 => return String::from("it's a fibonacci subset!"),
            8..=i32::MAX => return String::from(format!("probably pretty high {}", n)),
            _ => {
                // let unicode_value = u32::from_str_radix(&funky_rand[0].to_string(), 16).unwrap();
                // let character = std::char::from_u32(unicode_value).unwrap();
                //return String::from(format!("I bet you missed something hehe JACKPOT!! {}", funky_rand[0]));
                //println!("{}", snailquote::unescape(&funky_rand[0].to_string()).unwrap());
                return String::from(format!("I bet you missed something hehe JACKPOT!! {}", funky_rand[0])); //Windows console does NOT support UTF-8!!!
            },
        }
    }


    // Unicode is a character set. UTF-8, UTF-16, and UTF-32 are different encodings for the same character set.

    //todo
    pub fn output_rand_unicode() {
        loop {
            println!("{}", rand::random::<char>());
            //if ()
        }
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
