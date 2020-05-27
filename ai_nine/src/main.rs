use zzz_print_problem_header::header;

fn main() {
    header::problem(9);

    //let ans = eight::greatest_adjacent_prodect_of_digits(4);
    let ans = nine::do_it();

    println!("{}", ans);
}
/*
<p>A Pythagorean triplet is a set of three natural numbers, <var>a</var> &lt; <var>b</var> &lt; <var>c</var>, for which,</p>
<div style="text-align:center;"> <var>a</var><sup>2</sup> + <var>b</var><sup>2</sup> = <var>c</var><sup>2</sup></div>
<p>For example, 3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup>.</p>
<p>There exists exactly one Pythagorean triplet for which <var>a</var> + <var>b</var> + <var>c</var> = 1000.<br />Find the product <var>abc</var>.</p>
*/

// pythag triplet a<b<c, s.t. a^2 + b^2 = c^2
// e.g. 3, 4, 5
// only one for which a + b + c = 1000,
// find a*b*c
mod nine {
    use std::cmp::Ordering;
    use std::fs;
    use regex::Regex;
    
    struct Trip {
        a: u64,
        b: u64,
        c: u64,
    }
    impl Trip {
        fn is_pythag(&self) -> bool { 
            self.a.pow(2) + self.b.pow(2) == self.c.pow(2)  &&
                self.a < self.b &&
                    self.b < self.c
        }
        fn is_the_answer(&self) -> bool {
            self.is_pythag() && (self.a + self.b + self.c == 1000)
        }
        fn from_tuple(initializer: (u64, u64, u64)) -> Trip {
            Trip {a: initializer.0, b: initializer.1, c: initializer.2}
        }
        fn is_too_big(f: &Trip) -> bool { f.a + f.b + f.c > 1000 }
    }
    impl Iterator for Trip {
        type Item = (u64, u64, u64);
        fn next(&mut self) -> Option<(u64, u64, u64)> {
            self.c = self.c + 1;
            if Trip::is_too_big(self) { self.c = 1 } 
            else { return Some((self.a, self.b, self.c)); }

            self.b = self.b + 1;
            if Trip::is_too_big(self) { self.b = 1 } 
            else { return Some((self.a, self.b, self.c)); }

            self.a = self.a + 1;
            if Trip::is_too_big(self) { self.a = 1 } 
            else { return Some((self.a, self.b, self.c)); }

            None
        }
    }
    
    // Greatest adjacent product of n digits
    pub fn do_it() -> u64 {
        let t: Vec<(u64, u64, u64)> = Trip::from_tuple((1, 1, 1))
            .filter(|&trip: &(u64, u64, u64)| {
                Trip::from_tuple(trip).is_the_answer()
            }).collect();
        println!("The tup is: {} {} {}", t[0].0, t[0].1, t[0].2);
        //assert!(t.len() == 1);
        t[0].0 * t[0].1 * t[0].2
    }

}
