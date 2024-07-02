// src/rust/katas.rs

pub mod katas {
    pub fn number_0(bus_stops: &[(i32, i32)]) -> i32 {
        let mut ppl = 0;
        for stop in bus_stops {
            ppl += stop.0 - stop.1;
        }
        return ppl;
    }

    pub fn number_1(bus_stops: &[(i32, i32)]) -> i32 {
        bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)
    }
}