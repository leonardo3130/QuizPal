pub fn escape_md_v2(text: &str) -> String {
    let specials = r"_*\[\]()~`>#+\-=|{}.!";

    let mut new = String::with_capacity(text.len());

    for c in text.chars() {
        if specials.contains(c) {
            new.push('\\');
        }
        new.push(c);
    }

    new
}
