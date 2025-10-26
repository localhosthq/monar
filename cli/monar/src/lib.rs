pub fn generate_greeting(name: Option<&str>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Welcome to monar CLI!".to_string(),
    }
}
