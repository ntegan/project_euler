use zzz_print_problem_header::header;

fn main() {
    header::problem(8);

    //let ans = eight::greatest_adjacent_prodect_of_digits(4);
    let ans = eight::greatest_adjacent_prodect_of_digits(13);

    println!("{}", ans);
}

mod eight {
    use std::cmp::Ordering;
    use std::fs;
    use regex::Regex;
    
    //use super::*;
    
    // Greatest adjacent product of n digits
    pub fn greatest_adjacent_prodect_of_digits(n: u64) -> u64 {
        let the_num_vec = get_the_num();
        let the_full_num = the_num_vec.join("");

        // turbofish operator ::<>
        //let the_full_int: u64 = the_full_num.parse::<u64>().unwrap();
        // ^^ nope

        // do stuff widdit
        do_the_stuff(the_full_num, n)
    }

    fn do_the_stuff(numstring: String, n: u64) -> u64 {
        let mut bestprod: u64 = 0;
        let mut product: u64;
        let bytes = numstring.as_bytes();
        assert!(n < bytes.len() as u64);

        // i will "point" to all possible groups of n chars
        for i in 0..bytes.len() as u64 - n - 1 {
            product = 1;
            for j in i..i + n as u64 {
                product = product * 
                    (numstring.chars().nth(j as usize)
                     .unwrap().to_digit(10).unwrap() as u64);
            }
            bestprod = if product > bestprod { product } else { bestprod };
        }
        bestprod
    }

    fn get_the_num() -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        let raw_string = fs::read_to_string("ah_eight/info.html")
            .expect("Couldn't find the info.html. need it for the num");
        let re = Regex::new(r"([0-9]*)<br").unwrap();
        assert!(re.is_match(&raw_string));
        for cap in re.captures_iter(&raw_string) {
            v.push(String::from(&cap[1]));
        }

        assert!(v.len() > 1);

        v
    }
}
