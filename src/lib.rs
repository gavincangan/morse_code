use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use console_error_panic_hook;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
}

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
    pub fn convert(&self, input: &str) -> String {
        let is_morse = self.is_morse_code(input);
        console_log!("DEBUG Detected {}: {}", if is_morse { "morse code" } else { "plain text" }, input);
        if is_morse {
            self.from_morse(input)
        } else {
            self.to_morse(input)
        }
    }

    fn is_morse_code(&self, input: &str) -> bool {
        input.split_whitespace().all(|word| {
            word.chars().all(|c| c == '.' || c == '-')
        })
    }

    fn to_morse(&self, text: &str) -> String {
        let morse = text.split_whitespace()
            .map(|word| word.to_uppercase().chars()
                .filter_map(|c| self.to_morse.get(&c))
                .map(|&s| s)
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("  ");
        console_log!("DEBUG output - Morse code: {:?}", morse);
        morse
    }

    fn from_morse(&self, morse: &str) -> String {
        let text = morse.split("  ")
        .map(|word| word.split_whitespace()
            .map(|code| *self.from_morse.get(code).unwrap_or(&' '))
            .collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");
        console_log!("DEBUG output - text: {:?}", text);
        text
    }
}