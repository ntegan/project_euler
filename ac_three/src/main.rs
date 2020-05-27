use zzz_print_problem_header::header;

fn main() {
    header::problem(3);

    let answer = three::get_largest_prime_factor(15);
    println!("{}", answer);
}
//<p>The prime factors of 13195 are 5, 7, 13 and 29.</p>
//<p>What is the largest prime factor of the number 600851475143 ?</p>

mod three {
    pub fn get_largest_prime_factor(num: u64) -> u64 {
        3
    }
}
