pub fn abbreviate(phrase: &str) -> String {
    let mut s = String::from("");
    for i in phrase.split(" ") {
        s.push_str(i);
    }
    s
}
