extern crate rand;

use rand::Rng;

trait FirstLetterToUppperCase {
    fn first_to_uppper_case(self) -> String;
}
  
impl FirstLetterToUppperCase for String {
    fn first_to_uppper_case(self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

pub struct Lorem {
    words: Vec<&'static str>,
    rng: rand::prelude::ThreadRng,
}

impl Lorem {
    pub fn new() -> Lorem {
        let list = include_str!("./resources/lorem-list").lines()
        .map(|l| l)
        .collect();
        Lorem {
            words: list,
            rng: rand::thread_rng()
        }
    }

    fn get_count(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            min
        } else {
            self.rng.gen_range(min, max)
        }
    }

    pub fn get_phrase(mut self, min: u32, max: u32) -> String {
        let count = self.get_count(min, max);
        let mut phrase = String::new();
        for _ in 0..count {
            let sentences = self.get_count(2, 6);
            for _ in 0..sentences {
                let mut first_word = self.get_words(1, false);
                first_word = first_word.first_to_uppper_case();
                phrase.push_str(&first_word);
                let quantity = self.get_count(10, 20);
                phrase.push_str(&self.get_words(quantity, false));
                phrase.pop();
                phrase = phrase.trim_matches(',').to_string();
                phrase.push_str(". ");
            }
            phrase.push_str("\n\n")
        }
        phrase
    }

    fn get_words(&mut self, count: u32, title: bool) -> String {
        let size = self.words.len();
        let mut word_count = 0;
        let mut words = String::new();
        while word_count < count {
            let mut word = self.words[self.rng.gen_range(0, size)].to_string();
            if title && (word_count == 0 || word.len() > 3) {
                word = word.first_to_uppper_case();
            }
            words.push_str(&word);
            let comma_chan = 20;
            let rand = self.rng.gen_range(0,100);
            if comma_chan >= rand && word.len() > 3 {
                words.push_str(", ")
            } else {
                words.push_str(" ");
            }
            word_count += 1;
        }
        words
    }
}