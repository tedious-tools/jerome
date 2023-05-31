use std::env;

use anyhow::{bail, Result};
use clap::{Command, Arg, crate_version, value_parser};

const MESSAGE_NO_ARGS: &str = "command args hould never be 0, something has gone horribly wrong";
const MESSAGE_MISSING_ARGS: &str = "Must provide a string to convert to rot13!";

fn main() -> Result<()> {
    let cmd = build_cli();
    let matches = cmd.get_matches();

    // let cipher = matches.get_one::<String>("cipher").unwrap();
    // let rotation = matches.get_one::<i64>("rotation").unwrap();
    let text: Vec<_> = matches.get_many::<String>("text").unwrap().collect();
    let text = text.iter().fold(text[0].clone(), |accumulator, val| { format!("{} {}", accumulator, val) });

    let translator = jerome::Translator::new();
    // let args: Vec<String> = env::args().collect();
    // let msg = parse_args(args)?;
    let translated = translator.rot13(&text)?;

    println!("Translated gets you: \n{}", translated);

    match cli_clipboard::set_contents(translated) {
        Err(_) => bail!("\n==> Unable to copy to clipboard".to_string()),
        Ok(_) => {
            println!("Successfully copied to clipboard");
            return Ok(());
        }
    };
}

fn build_cli() -> Command {
    let which_cipher = Arg::new("cipher")
        .short('c')
        .long("cipher")
        .help("The kind of cipher to use. Possible values: caesar")
        .value_name("CIPHER")
        .default_value("caesar")
        .value_parser(["caesar"]);

    let caesar_rotation = Arg::new("rotation")
        .required_if_eq("cipher", "caesar")
        .short('r')
        .long("rotation")
        .help("Caesar rotation to use. Valid values are from 1-25 inclusive")
        .default_value("13")
        .value_parser(value_parser!(i64).range(1..=25));

    let text = Arg::new("text")
        .last(true)
        .help("Text to translate")
        .required(true);


    Command::new("jerome")
        .author("Jon Anderson <coding@jpanderson.dev>")
        .version(crate_version!())
        .about("Allows performing translations based on ciphers")
        .arg(which_cipher)
        .arg(caesar_rotation)
        .arg(text)

}

fn parse_args(args: Vec<String>) -> Result<String> {
    let msg = match args.len() {
        0 => bail!(MESSAGE_NO_ARGS),
        1 => bail!(MESSAGE_MISSING_ARGS),
        2 => args[1].clone(),
        _ => {
            let text = &args[1..];

            text[1..].iter().fold(args[1].clone(), |accumulator, val| {
                format!("{} {}", accumulator, val)
            })
        }
    };

    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::{assert_err, assert_ok};

    #[test]
    fn test_arg_values() {
        let args: Vec<String> = vec![];
        let err = assert_err!(parse_args(args));

        assert_eq!(MESSAGE_NO_ARGS, err.to_string());

        let args: Vec<String> = vec!["jerome".to_string()];
        let err = assert_err!(parse_args(args));

        assert_eq!(MESSAGE_MISSING_ARGS, err.to_string());

        let args: Vec<String> = vec!["jerome".to_string(), "doggy".to_string()];
        let text = assert_ok!(parse_args(args));

        assert_eq!("doggy", text);

        let args: Vec<String> = vec![
            "jerome".to_string(),
            "doggy".to_string(),
            "is".to_string(),
            "happy".to_string(),
        ];
        let text = assert_ok!(parse_args(args));

        assert_eq!("doggy is happy", text);
    }
}
