fn main() {
    let pig_latin = to_pig_latin("first");
    println!("{}", pig_latin);
}

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word
// and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!
fn to_pig_latin(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let first_char = s.chars().next().unwrap();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let pig_latin = if vowels.contains(&first_char) {
        format!("{s}-hay")
    } else {
        let rest_of_word = s.chars().skip(1).collect::<String>();

        format!("{rest_of_word}-{first_char}ay")
    };
    pig_latin
}
