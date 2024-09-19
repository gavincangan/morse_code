use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct MorseConverter {
    to_morse: HashMap<char, &'static str>,
    from_morse: HashMap<&'static str, char>,
}

#[wasm_bindgen]
impl MorseConverter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut to_morse = HashMap::new();
        let mut from_morse = HashMap::new();

        let morse_codes = [
            ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."),
            ('E', "."), ('F', "..-."), ('G', "--."), ('H', "...."),
            ('I', ".."), ('J', ".---"), ('K', "-.-"), ('L', ".-.."),
            ('M', "--"), ('N', "-."), ('O', "---"), ('P', ".--."),
            ('Q', "--.-"), ('R', ".-."), ('S', "..."), ('T', "-"),
            ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"),
            ('Y', "-.--"), ('Z', "--.."), ('0', "-----"), ('1', ".----"),
            ('2', "..---"), ('3', "...--"), ('4', "....-"), ('5', "....."),
            ('6', "-...."), ('7', "--..."), ('8', "---.."), ('9', "----.")
        ];

        for &(c, code) in morse_codes.iter() {
            to_morse.insert(c, code);
            from_morse.insert(code, c);
        }

        MorseConverter { to_morse, from_morse }
    }

    #[wasm_bindgen]
    pub fn to_morse(&self, text: &str) -> String {
        text.to_uppercase()
            .split_whitespace()
            .map(|word| word.chars()
                .map(|c| *self.to_morse.get(&c).unwrap_or(&" "))
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("/")
    }

    #[wasm_bindgen]
    pub fn from_morse(&self, morse: &str) -> String {
        morse.split_whitespace()
            .flat_map(|code| if code == "/" { vec![' '] } else { vec![*self.from_morse.get(code).unwrap_or(&' ')] })
            .collect()
    }
}