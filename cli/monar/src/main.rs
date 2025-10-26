use clap::Parser;
use monar::generate_greeting;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{}", generate_greeting(cli.name.as_deref()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_greeting() {
        assert_eq!(generate_greeting(None), "Welcome to monar CLI!");
    }

    #[test]
    fn test_generate_greeting_with_name() {
        assert_eq!(generate_greeting(Some("test")), "Hello, test!");
    }
}
