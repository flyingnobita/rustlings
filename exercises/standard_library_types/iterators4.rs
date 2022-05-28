// iterators4.rs

struct Factorial {
    curr: u64,
    next: u64,
}

impl Iterator for Factorial {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr - 1;

        self.curr = self.next;
        self.next = new_next;

        if self.curr == 0 {
            None
        } else {
            Some(self.curr)
        }
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    (2..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
