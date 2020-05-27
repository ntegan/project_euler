use zzz_print_problem_header::header;

fn main() {
    header::problem(6);

    let ans = six::get_the_diff(100);

    println!("{}", ans);
}

// sum of squares first 10 nat nums = 1^2 + ... + 10^2 = 385
// square of sum first 10 nat nums = (1 + 2 + ...10)^2 = 3025
// diff is 3025 - 385 = 2640
// find diff b/t sum sq of first 100 nat and the square of their sum

mod six {
    //use super::*;

    pub fn get_the_diff(upto: u64) -> u64 {
        ((squares_summed(upto) as i64) - (sum_squared(upto) as i64))
            .abs() as u64
    }

    fn squares_summed(upto: u64) -> u64{
        (1..upto + 1)
            .map(|gg| { gg.pow(2) })
            .sum()
    }

    fn sum_squared(upto: u64) -> u64{
        //std::ops::Range<u64> { start: 3, end: 5 }
        (1..upto + 1)
            .sum::<u64>()
            .pow(2)
    }


}
