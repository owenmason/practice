pub mod katas {
    #[cfg(test)]
    #[test]
    pub fn run_tests() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
        assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
        // assert_eq!(spin_words("no Seriously!! this is the last one!!"), "no ylsuoireS!! this is the last one!!");
        // assert_eq!(spin_words("no no Seriously!! this is the last oneee!!"), "no no ylsuoireS!! this is the last oneee!!");
        // assert_eq!(spin_words("no no nooooo Seriously!! this is the last one!!"), "no no nooooo ylsuoireS!! this is the last one!!");

        assert!(  is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        assert!(! is_valid_walk(&['w']));
        assert!(! is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }

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

    pub fn string_rev(phrase: &str) -> String {
        let p_vec: Vec<char> = phrase.chars().rev().collect();
        let mut res: String = String::from("");
        for c in p_vec {
            res.push_str(&c.to_string());
        }
        return res;
        //return String::from(phrase.chars().rev()); //WHY DOESN'T THIS WORK??
    }

    pub fn string_rev_revisit(phrase: &str) -> String {
        phrase.chars().rev().collect()
    }

    pub fn spin_words(words: &str) -> String {
        let mut result: Vec<String> = Vec::new();
        let w_split: Vec<&str> = words.split(' ').collect();
        for word in w_split {
            if word.trim().len() >= 5 {
                result.push(word.chars().rev().collect());
            }
            else {
                result.push(word.to_string());
            }
        }
        return result.join(" ");
    }

    // todo: this verison works with punctuation!!
    // pub fn spin_words_adv(words: &str) -> String {
    //     let mut result: Vec<String> = Vec::new();
    //     let w_split: Vec<&str> = words.split(' ').collect();
    //     for word in w_split {
    //         let chars_vec: Vec<char> = word.chars().collect();
    //         let num_alpha_chars: i32 = word.chars().map(|x| x.is_alphabetic()).collect::Vec<char>().len();
    //         let mut result_word: Vec<char> = Vec::new();
    //         if num_alpha_chars >= 5 {
    //             let word_len: usize = chars_vec.len();
    //             let mut i: usize = 0;
    //             for c in chars_vec.iter() {
    //                 if c.is_alphabetic() {
    //                     result_word.push(chars_vec[word_len-i]);
    //                 }
    //                 else {
    //                     result_word.push(*c);
    //                 }
    //                 i += 1;
    //             }
    //         }
    //     }
    //     return result.join(" ");
    // }

    //🌟
    pub fn is_valid_walk(walk: &[char]) -> bool {
        if walk.len() != 10 { return false; }
        let (mut x, mut y): (i32, i32) = (0, 0);
        for dir in walk {
            match dir {
                'n' => y += 1,
                's' => y -= 1,
                'e' => x += 1,
                'w' => x -= 1,
                _ => ()
            };
        }
        (x, y) == (0, 0)
    }
}
