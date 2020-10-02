// 1st consonant of each word is moved to end with "ay" added; "first" -> "irst-fay"
// if word starts with vowel "hay" added instead; "apple" -> "apple-hay"
// verify UTF encoding
pub mod translate {
    pub fn to_pig_latin(text: &str) -> String {
        let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
        let mut result = String::new();
        for word in text.split_whitespace() {
            // consonant: first -> irst-fay
            // consonant: t -> -tay
            // vowel: oski -> oksi-hay
            // vowel: i -> i-hay
            // punctuation: hey! -> ey-hay!
            // punctuation: hey!! -> ey-hay!!

            result.push(' ');

            let mut chars = word.chars().peekable();

            // if word is single character push and finish
            if chars.peek().is_some() {
                let char = chars.next().unwrap();
                result.push(char);
                continue;
            }

            let mut word_terminated = false;

            // build suffix from first character
            let mut suffix = String::new();
            let char = chars.next().unwrap();
            if vowels.contains(&char) {
                suffix.push_str("-hay")
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
                    result.push(char);
                    continue
                }

                if !char.is_alphabetic() {
                    word_terminated = true;
                }

                result.push(char);
            }

            result.push_str(&suffix);
        }

        result
    }
}