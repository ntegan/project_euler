use zzz_print_problem_header::header;

fn main() {
    header::problem(7);

    //let ans = seven::get_the_umpth_prime_numer(6);
    let ans = seven::get_the_umpth_prime_numer(10_001);

    println!("{}", ans);
}
// first six prime numbers: 2,3,5,7,11,13
// sixth is 13
//
// what is 10_001 st prime num

mod seven {
    use std::cmp::Ordering;
    //use super::*;
    
    // get the nth prime number
    pub fn get_the_umpth_prime_numer(n: u64) -> u64 {
        // start at the 1st prime number
        let mut i = 2u64;   // iterator, test if this is prime
        let mut j = 1u64;   // we are on the jth prime

        loop {
            // check if is prime
            match is_prime(i) {
                true => {},
                false => { i = i + 1; continue; },
            }

            // if we are on the prime that we are asked for
            //      then break and return
            if let Some(o) = j.partial_cmp(&n) {
                match o { Ordering::Equal => { break; }, _ => {} }
            }

            // otherwise keep looking
            i = i + 1;

            // if we get here, i is prime
            j = j + 1;
        }

        // return j
        i
    }

    fn is_factor_of(a: u64, b: u64) -> bool { (b % a) == 0 }
    fn is_prime(n: u64) -> bool {
        //  definition of a prime number
        //  only factors 1 and itself
        let factors: Vec<u64> = (1..n)
            .filter(|&b: &u64|{ is_factor_of(b, n) }).collect();

        factors.len() <= 1
    }

}
