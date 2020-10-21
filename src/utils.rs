use std::borrow::Cow;

// based from https://lise-henry.github.io/articles/optimising_strings.html
// not going to use regex just for that
pub fn clean_name<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();

    fn is_trouble(c: char) -> bool {
        c == ' ' || c == '.' || c == ',' || c == '=' || c == '[' || c == ']' || c == '/' || c == ':'
    }

    let first_trouble_character = input.find(is_trouble);
    if let Some(first_trouble_character) = first_trouble_character {
        let mut output = String::from(&input[0..first_trouble_character]);
        output.reserve(input.len() - first_trouble_character);
        let rest = input[first_trouble_character..].chars();
        for c in rest {
            match c {
                ' ' => output.push_str("_"),
                '-' => output.push_str("_"),
                '/' => output.push_str("_"),
                '=' => {}
                '.' => {}
                ':' => output.push_str("_"),
                ',' => {}
                '[' => {}
                ']' => {}
                '(' => {}
                ')' => {}
                '\'' => {}
                '\"' => {}
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}
