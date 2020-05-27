fn main() {
    let a = "\
Problem 1
=========";

    eprintln!("{}", a);
    eprint!("The sum is: ");
    println!("{}", one::get_the_sum());
}

mod one {
    // lots of iters and collects because idk the type annotation for iterator
    //pub fn get_the_sum() -> u64 { get_multiples(numbers_below(10))
    pub fn get_the_sum() -> u64 { get_multiples(numbers_below(1000))
        .iter().sum() }
    fn numbers_below(below_me: u64) -> Vec<u64> { (0..below_me).collect() }
    fn get_multiples(a: Vec<u64>) -> Vec<u64> { a.into_iter()
        .filter(|&num: &u64| (num % 3 == 0) || (num % 5 == 0)).collect() }
}

//<p>If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.</p>
//<p>Find the sum of all the multiples of 3 or 5 below 1000.</p>
