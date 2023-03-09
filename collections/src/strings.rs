/*
Convert strings to pig latin. The first consonant of each word is moved to the 
end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that 
start with a vowel have “hay” added to the end instead (“apple” becomes 
“apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn pig_latin(s: &str) -> String {
    let mut pig_latin = String::new();
    for word in s.split_whitespace() {
        let mut pig_word = String::new();
        let first_char = word.chars().next().unwrap();
        if !is_char(first_char) {
            pig_word.push(first_char);
        } else {
            if is_vowel(first_char) {
                pig_word = word.get(0..word.len()).unwrap().to_string();
                pig_word.push_str("-hay");
            } else { // is consonant
                pig_word = word.get(1..word.len()).unwrap().to_string();
                pig_word.push('-');
                pig_word.push(first_char);
                pig_word.push_str("ay");
            }
        }
        pig_latin.push_str(&pig_word);
        pig_latin.push(' ');
    };
    pig_latin.pop();
    return pig_latin
}

fn is_char(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn example() {
    let sentence = "be quiet ! Do not let them know what we are saying";
    let new_sentence = pig_latin(sentence);
    println!("{}", sentence);
    println!("{}", new_sentence);
}
