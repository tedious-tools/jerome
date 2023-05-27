use std::collections::HashMap;
use anyhow::Result;

const ROT13_MAPPING: [(char, char); 26] = [
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

pub struct Translator {
    rot13_mapping: HashMap<char, char>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator{
            rot13_mapping: HashMap::from(ROT13_MAPPING),
        }
    }

    /// Encodes/decodes a given text using a ROT13 cipher. At the moment, this
    /// loses any case by default. Will likely take na option later of some kind.
    /// Basically just stores the HashMap so the creation only happens once.
    ///
    /// # Examples
    /// ```
    /// let original = "Zapper man".to_string();
    /// assert_eq!("mnccre zna".to_string(), jerome::Translator::new().rot13(&original).unwrap());
    /// ```
    pub fn rot13(&self, plaintext: &String) -> Result<String> {
        let plaintext = plaintext.to_ascii_lowercase();

        let mut translated = String::from("");

        for c in plaintext.chars() {
            if let Some(found_val) = self.rot13_mapping.get(&c) {
                translated.push(found_val.clone());
            } else {
                translated.push(c);
            }
        }

        Ok(translated)
    }
}

/// Encodes/decodes a given text using a ROT13 cipher. At the moment, this
/// loses any case by default. Will likely take na option later of some kind.
///
/// # Examples
/// ```
/// let original = "Zapper man".to_string();
/// assert_eq!("mnccre zna".to_string(), jerome::rot13(&original).unwrap());
/// ```
pub fn rot13(plaintext: &String) -> Result<String> {
    Translator::new().rot13(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_rot13() {
        let original = "Sphinx of black quartz, judge my vow.".to_owned();
        let expected = "fcuvak bs oynpx dhnegm, whqtr zl ibj.".to_owned();

        assert_eq!(expected, rot13(&original).unwrap());

        let translator = Translator::new();

        assert_eq!(expected, translator.rot13(&original).unwrap());
    }
}
