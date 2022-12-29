pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",

        m if m.ends_with("?")
            && m.contains(char::is_alphabetic)
            && !m.contains(char::is_lowercase) =>
        {
            "Calm down, I know what I'm doing!"
        }

        m if m.ends_with("?") => "Sure.",
        m if m.contains(char::is_alphabetic) && !m.contains(char::is_lowercase) => {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}
