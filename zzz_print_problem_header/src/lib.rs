#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod header {
    // print the header for problem number
    pub fn problem(number: u64) {
            eprintln!("\
Problem {}
===========", number);
            eprint!("The answer is: ");
    }
}
