// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut boundary: (usize, usize) = (0, input.len() - 1);
    let whitespace = " ".to_string();
    while boundary.0 < boundary.1 {
        if input[boundary.0..boundary.0 + 1] == whitespace {
            boundary.0 += 1;
        }
        if input[boundary.1..boundary.1 + 1] == whitespace {
            boundary.1 -= 1;
        }
        if input[boundary.0..boundary.0 + 1] != whitespace && input[boundary.1..boundary.1 + 1] != whitespace {
            return input[boundary.0..(boundary.1 + 1)].to_string();
        }
    }
    return String::from("");
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut res = input.to_string();
    res.push_str(" world!");
    return res;
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let res = input.replace("cars", "balloons");
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
