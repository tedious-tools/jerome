use std::env;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let translator = jerome::Translator::new();

    translator.caesar(&String::from("doggy man"), 13)?;

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        bail!("Must provide a string to convert to rot13!")
    }

    let msg = match args.len() {
        0 => bail!("command args hould never be 0, something has gone horribly wrong"),
        1 => bail!("Must provide a string to convert to rot13!".to_string()),
        2 => args[1].clone(),
        _ => {
            let text = &args[1..];

            text[1..].iter().fold(args[1].clone(), |accumulator, val| format!("{} {}", accumulator, val))
        },
    };

    println!("Message is: {}", msg);

    let translated = jerome::rot13(&msg)?;

    println!("Translated gets you: \n{}", translated);

    match cli_clipboard::set_contents(translated) {
        Err(_) => bail!("\n==> Unable to copy to clipboard".to_string()),
        Ok(_) => {
            println!("Successfully copied to clipboard");
            return Ok(())
        },
    };
}
