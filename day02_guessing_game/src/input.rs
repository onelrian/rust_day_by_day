use std::io;

pub fn input(prompt: &str, attempt: usize) -> i32 {
    let prompt = format!("{} Attempt {}", prompt, attempt + 1);
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        eprintln!("No input provided");
        return 0;
    }

    match trimmed_input.parse::<i32>() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Invalid input: {}", e);
            0
        }
    }
}
