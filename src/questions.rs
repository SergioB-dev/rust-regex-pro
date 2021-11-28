use std::io::stdin;
use regex::Regex;

use crate::regex_qa::is_good_regex;
use crate::user::{Ranking, User};

/// Defines what a question consists of.
/// `explanation` - Offers an explanation to how the regex works.
/// `search_string` - The string the user is asked to capture with regex.
/// `filler_string` - Filler text that is placed around or even through the `search_string` to
/// make things a little harder.
/// `points` - How many points a question is worth.
/// `level` - Which tier a question belongs to.
pub struct Question {
    pub explanation: &'static str,
    pub search_string: &'static str,
    pub filler_string: Option<&'static str>,
    pub filler_order: FillerOrder,
    pub points: u32,
    pub ranking: Ranking,
}

impl Question {
    pub fn ask_user_question(&self, user: &mut User) {
        println!("As your first challenge, come up with a clever regex to capture this: \n\n\n\n");
        println!("\t \t--> {} <--", self.search_string);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        let re = Regex::new(&input.trim()).unwrap();

        // Print if the regex is correct or wrong
        match is_good_regex(re, self.search_string) {
            true => {
                println!("Correct");
                user.correct += 1;
                user.score += self.points;
            },
            false => {
                println!("Wrong");
                user.wrong += 1;
            },
        }
    }
}

/// An enum dictating how filler words are spread throughout the final string that is shown
/// to the user.
pub enum FillerOrder {
    Before, After, Throughout, None
}



