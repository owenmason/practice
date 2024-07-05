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

    //one iter
    pub fn digital_root_single(n: i64) -> i64 {
        let mut n_vec = Vec::new();
        for c in n.to_string().chars() {
            n_vec.push(c);
        }
        return n_vec.iter().map(|&c| c.to_digit(10).unwrap() as i64).sum();
    }

    //high score solution. wow.
    pub fn digital_root_optimized(n: i64) -> i64 {
        (n - 1) % 9 + 1
    }

    pub fn digital_root(n: i64) -> i64 {
        let sum_digits = n.to_string().chars().map(|x| x.to_digit(10).unwrap() as i64).sum();
        if sum_digits > 9 {
            digital_root(sum_digits);
        }
        return sum_digits;
    }

    pub fn digital_root_loop(n: i64) -> i64 {
        let mut sum_digits: i64 = n;
        loop {
            sum_digits = sum_digits.to_string().chars().map(|x| x.to_digit(10).unwrap() as i64).sum();
            if sum_digits < 10 {
                return sum_digits;
            }
        }
    }
}
