use clipboard::{ClipboardContext, ClipboardProvider};
use convert_case::{Case, Casing};
use std::env;

fn to_snake_case(text: &str) -> String {
    text.to_case(Case::Snake)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- \"Your Input Text Here\"");
        std::process::exit(1);
    }

    let input_text = &args[1];

    let snake_case_result = to_snake_case(input_text);

    println!("Input text: {}", input_text);
    println!("Snake case: {}", snake_case_result);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(snake_case_result).unwrap();
    println!("Snake case copied to clipboard.");
}
