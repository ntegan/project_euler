use zzz_print_problem_header::header;

fn main() {
    header::problem(5);
    //let ans = five::get_the_palindrome();
    let ans = five::get_the_num();

    println!("{}", ans);
}

// 2520 smallest num divisible by all of [1..10], no remainder
// what smallest pos num is same by all nums [1..20]?
mod five {
    use super::*;

    pub fn get_the_num() -> u64 {
        smallest_num_divisible_by_one_through(20)
    }

    fn smallest_num_divisible_by_one_through(upto: u64) -> u64 {
        // divisible by 1 through upto (inclusive)
        let d_by: Vec<u64> = (1..upto + 1).collect();
        let mut i=1;

        loop {
            match is_divisible_by_all(i, &d_by) {
                true    => { break; },
                false   => { },
            }

            i = i + 1;
        }

        i
    }

    // is a divisible by b?
    fn is_divisible_by(a: u64, b: u64) -> bool { a % b == 0 }
    fn is_divisible_by_all(a: u64, b: &Vec<u64>) -> bool {
        for i in b.iter() {
            match is_divisible_by(a, *i) {
                true => {},
                false => { return false },
            }
        }
        true
    }
}
