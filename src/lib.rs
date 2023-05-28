use std::vec::Vec;

use anyhow::{bail, Result};

pub struct Translator {
    alphabet: Vec<char>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator{
            alphabet: ('a'..='z').collect(),
        }
    }

    /// Encodes/decodes a given text using a ROT13 cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// let original = "Zapper man".to_string();
    /// assert_eq!("Mnccre zna".to_string(), jerome::Translator::new().rot13(&original).unwrap());
    /// ```
    pub fn rot13(&self, plaintext: &String) -> Result<String> {
        self.caesar(plaintext, 13)
    }

    /// Encodes/decodes a given text using any Caesar rotation cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// let original = "Faces".to_string();
    /// assert_eq!("Idfhv".to_string(), jerome::Translator::new().caesar(&original, 3).unwrap());
    /// ```
    pub fn caesar(&self, plaintext: &String, shift: usize) -> Result<String> {
        if shift > 25 {
            bail!("Must pass shift between 0 and 25 inclusive");
        }

        let mut translated = String::from("");

        for c in plaintext.chars() {
            if !c.is_ascii_alphabetic() {
                translated.push(c);
                continue;
            }

            let is_uppercase = c.is_ascii_uppercase();
            let needle = c.to_ascii_lowercase();

            // This should return true, so something has probably gone wrong if
            // this doesn't find the index.
            if let Some(index) = self.alphabet.iter().position(|&r| r == needle) {
                let rotation = (index + shift) % 26;
                let new_letter = self.alphabet[rotation].clone();

                let new_letter = match is_uppercase {
                    true => new_letter.to_ascii_uppercase(),
                    false => new_letter,
                };

                translated.push(new_letter);
            } else {
                bail!("Found a character we didn't expect or know how to handle: {}", c);
            }
        }

        Ok(translated)
    }
}

/// Encodes/decodes a given text using a ROT13 cipher.
///
/// # Examples
///
/// ```
/// let original = "Zapper man".to_string();
/// assert_eq!("Mnccre zna".to_string(), jerome::rot13(&original).unwrap());
/// ```
pub fn rot13(plaintext: &String) -> Result<String> {
    Translator::new().rot13(plaintext)
}

/// Encodes/decodes a given text using any Caesar rotation cipher.
    ///
    /// # Examples
    ///
    /// ```
    /// let original = "Faces".to_string();
    /// assert_eq!("Idfhv".to_string(), jerome::caesar(&original, 3).unwrap());
    /// ```
pub fn caesar(plaintext: &String, shift: usize) -> Result<String> {
    Translator::new().caesar(plaintext, shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_rot13() {
        let original = "Sphinx of black quartz, judge my vow.".to_owned();
        let expected = "Fcuvak bs oynpx dhnegm, whqtr zl ibj.".to_owned();

        assert_eq!(expected, rot13(&original).unwrap());

        let translator = Translator::new();

        assert_eq!(expected, translator.rot13(&original).unwrap());
    }

    #[test]
    fn caesar_cipher() {
        let original = "abrAcadaBya".to_owned();
        let expected = "bcsBdbebCzb".to_owned();

        assert_eq!(expected, caesar(&original, 1).unwrap());
    }
}
