pub fn to_pig_latin(text: &str) -> String {
    let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
    let mut result = String::new();
    for word in text.split_whitespace() {
        result.push(' ');

        let mut chars = word.chars();

        // if word is single character push and finish
        if word.len() == 1 {
            let char = chars.next().unwrap();
            result.push(char);
            continue;
        }

        let mut word_terminated = false;

        // build suffix from first character
        let mut suffix = String::new();
        let char = chars.next().unwrap();
        if vowels.contains(&char) {
            suffix.push_str("-hay");
            result.push(char);
        } else if char.is_alphabetic() {
            suffix.push_str(&format!("-{}ay", char));
        } else {
            word_terminated = true;
            result.push(char);
            continue
        }

        // iteration remaining characters
        while let Some(char) = chars.next() {
            if word_terminated {
                suffix.push(char);
                continue
            }

            // non-alpha chars indicate word terminated
            if !char.is_alphabetic() && char != '-' {
                word_terminated = true;
                suffix.push(char);
                continue
            }

            result.push(char);
        }

        result.push_str(&suffix);
    }

    result
}