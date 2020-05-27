fn main() {
    let a = "\
Problem 2
=========";

    eprintln!("{}", a);
    eprint!("The sum is: ");
    println!("{}", two::get_the_sum(4_000_000));
}

mod two {
    use std::cmp::Ordering;

    // fibonacci struct starting at curr and next, ending at no_exceed-1
    struct Fibonacci {
        curr:       u64,
        next:       u64,
        no_exceed:  u64,
    }
    impl Iterator for Fibonacci {
        type Item = u64;
        fn next(&mut self) -> Option<u64> {
            let nextt = self.curr + self.next;
            match self.curr.partial_cmp(&self.no_exceed).expect("Why no compare") {
                Ordering::Less => {
                    let ret = self.curr;
                    self.curr = self.next;
                    self.next = nextt;
                    Some(ret)
                },
                _ => None,
            }
        }
    }

    // getting the solution
    pub fn get_the_sum(no_exceed: u64) -> u64 {
        Fibonacci { curr: 1, next: 2, no_exceed: no_exceed }
            .filter(|&num: &u64|{ (num % 2) == 0 })            
            .sum()
    }
}
