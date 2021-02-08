extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

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
}

impl Lorem {
    pub fn new() -> Lorem {
        let list = include_str!("./resources/lorem-list").lines()
        .map(|l| l)
        .collect();
        Lorem {
            words: list,
        }
    }

    pub fn get_phrase(self, min: u32, max: u32) -> String {
        let mut rng = rand::thread_rng();
        let count:u32;
        if  min >= max {
            count = min;
        } else {
            count = rng.gen_range(min..max);
        }
        let sentences = Uniform::from(2..6);
        let quantity = Uniform::from(10..20);
        let mut phrase = String::new();
        for _ in 0..count {
            for _ in 0..sentences.sample(&mut rng) {
                let mut first_word = self.get_words(1);
                first_word = first_word.first_to_uppper_case();
                phrase.push_str(&first_word);
                phrase.push_str(&self.get_words(quantity.sample(&mut rng)));
                phrase.pop();
                phrase = phrase.trim_matches(',').to_string();
                phrase.push_str(". ");
            }
            phrase.push_str("\n\n")
        }
        phrase.trim().to_string()
    }

    fn get_words(&self, count: u32) -> String {
        let mut rng = rand::thread_rng();
        let size = self.words.len();
        let words_range = Uniform::from(0..size);
        let mut word_count = 0;
        let mut words = String::new();
        while word_count < count {
            let word = &self.words[words_range.sample(&mut rng)].to_string();
            words.push_str(&word);
            if rng.gen_ratio(20,100) && word.len() > 3 {
                words.push_str(", ")
            } else {
                words.push_str(" ");
            }
            word_count += 1;
        }
        words
    }
}