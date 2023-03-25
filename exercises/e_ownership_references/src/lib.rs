pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("The word \"{}\" is plural", s);
    } else {
        println!("The word \"{}\" is singular", s);
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    // if s.starts_with("b") && s.contains("a") {
    //     true
    // } else {
    //    false
    // }
    // The above us quite wordy
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
    // *s = String::from("sparkly") // Also works
}
