use zzz_print_problem_header::header;

fn main() {
    header::problem(3);

    let answer = three::get_largest_prime_factor(600851475143);
    println!("{}", answer);
}

// Very inefficient
mod three {
    pub fn get_largest_prime_factor(num: u64) -> u64 {
        // all potential factors: [1,num)
        //let a: Vec<u64> = (1..num)
        let a: Option<u64> = (1..num)
            // get factors of num
            .filter(|&b: &u64|{ is_factor_of(b, num) }) 
            // filter out the nonprime(composite) factors
            .filter(|&b: &u64|{ is_prime(b) })
            .last();
            //.collect();

        match a {
            Some(b) => b,
            None => 0
        }
    }
    // is a a factor of b?
    fn is_factor_of(a: u64, b: u64) -> bool { (b % a) == 0 }
    fn is_prime(n: u64) -> bool {
        //  definition of a prime number
        //  only factors 1 and itself
        let factors: Vec<u64> = (1..n)
            .filter(|&b: &u64|{ is_factor_of(b, n) }).collect();

        factors.len() <= 1
    }
}
