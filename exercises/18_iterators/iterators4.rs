// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

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

    // I really wanted to use recursion with tail call optimization (should have read the comment
    // better, but I got excited), but couldn't figure out how to build in an accumulator without
    // changing the function signature, 'cause I'd rather not require the caller to have to know
    // what number to pass as second argument. That should be an implementation detail. I came
    // across this in a blog: https://101wiki.softlang.org/Concept:Accumulator_parameter

    fn acc(current: u64, fact: u64) -> u64 {
        match current {
            ..=1 => fact,
            _ => acc(current - 1, current * fact),
        }
    }

    acc(num, 1)
    // Or just do this, but I didn't come up with it:
    // (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

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
