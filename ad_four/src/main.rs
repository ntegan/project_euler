use zzz_print_problem_header::header;

fn main() {
    header::problem(4);
    let ans = four::get_the_palindrome();

    println!("{}", ans);
}
//<p>A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.</p>
//<p>Find the largest palindrome made from the product of two 3-digit numbers.</p>

mod four {
    use super::*;
    use std::cmp::Ordering;
    pub fn get_the_palindrome() -> u64 {
        let start_num = 999;

        let mut first_found: u64 = 0; // j when find first palindrome
        let mut first_pal: Option<u64> = None; // first pal
        let mut best_pal: u64 = 0;

        // 100 first 3 digit num (potential palindrome)
        for i in (100..start_num).rev() {
            for j in (100..start_num).rev() {
                // only check for palindrome if product greater than curbest
                match best_pal.partial_cmp(&(i * j))
                    .expect("Uh oh") {
                    Ordering::Greater => { break; },
                    _ => {},
                }

                if is_palindrome(i * j) {
                    match first_pal {
                        // set the first found palindrome
                        None => { 
                            let first_found = j; 
                            first_pal = Some(i * j); 
                            best_pal = first_pal.unwrap();
                            println!("First: i {}, j {}, {}", i, j, best_pal);
                            break; },
                        // set subsequent palindromes
                        Some(_) => {
                            println!("New: i {}, j {}, {}", i, j, best_pal);
                            best_pal = i * j;
                        },
                    }
                }
            }

            // don't do any repeats
            match i.partial_cmp(&first_found).unwrap() {
                Ordering::Less => { break; },
                _ => {},
            }
        }

        best_pal
    }
    fn is_palindrome(test: u64) -> bool {
        let s = test.to_string();
        let slen = s.len();
        let front: String;
        let back: String;

        // make sure number is a reasonable number
        match slen.partial_cmp(&2).expect("Uncomparable") {
            Ordering::Less => panic!("Low number"),
            Ordering::Equal => panic!("Low number"),
            _ => {},
        }

        // even or odd number of characters
        match slen % 2 {
            0 => {
                front = String::from(&s[0..slen/2]);
                back = String::from(&s[slen/2..slen])
                    // reverse it
                    .chars().rev().collect();
            },
            _ => {
                front = String::from(&s[0..slen/2]);
                back = String::from(&s[slen/2 + 1..slen])
                    // reverse it
                    .chars().rev().collect();
            },
        }

        front == back
    }
}
