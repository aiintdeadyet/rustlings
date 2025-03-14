
fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // let mut first: usize = 0;
    // for (i, c) in input.char_indices() {
    //     first = i;
    //     if c != ' ' {
    //         break;
    //     }
    // }    
    // let mut last: usize = 0;
    // for (i, c) in input.char_indices() {
    //     last = i;
    //     if c != ' ' {
    //         break;
    //     }
    // }
    // output = input
    // input
    let chars_to_trim: &[char] = &[' '];
    let output: &str = input.trim_matches(chars_to_trim);
    output
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut output = String::from(input);
    output.push_str(" world!");
    output
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let output = String::from(input);
    output.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
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
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
