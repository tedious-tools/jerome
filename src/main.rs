use std::env;
use std::collections::HashMap;

const MAPPING: [(char, char); 26] = [
    ('a', 'n'),
    ('b', 'o'),
    ('c', 'p'),
    ('d', 'q'),
    ('e', 'r'),
    ('f', 's'),
    ('g', 't'),
    ('h', 'u'),
    ('i', 'v'),
    ('j', 'w'),
    ('k', 'x'),
    ('l', 'y'),
    ('m', 'z'),
    ('n', 'a'),
    ('o', 'b'),
    ('p', 'c'),
    ('q', 'd'),
    ('r', 'e'),
    ('s', 'f'),
    ('t', 'g'),
    ('u', 'h'),
    ('v', 'i'),
    ('w', 'j'),
    ('x', 'k'),
    ('y', 'l'),
    ('z', 'm'),
];

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => (),
        1 => return Err("Must provide a string to convert to rot13!".to_string()),
        _ => {
            return Err(
                "Found more than one argument. Please wrap your string in \"'s if it has spaces"
                    .to_string(),
            )
        }
    };

    let mapped = HashMap::from(MAPPING);
    let msg = args[1].to_ascii_lowercase();
    let mut translated = String::from("");

    for c in msg.chars() {
        if let Some(found_val) = mapped.get(&c) {
            translated.push(found_val.clone());
        } else {
            translated.push(c);
        }
    }

    println!("Translated gets you: \n{}", translated);

    Ok(())
}
