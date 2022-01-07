use crate::lib::basic::Level;
use crate::lib::regex_qa::is_good_regex;
use crate::user::{Ranking, User};
use regex::Regex;
use std::io::stdin;
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
    pub fn ask_user_question(&self, user: &mut User, level: &Level, i: &i8) -> bool {
        println!(
            "For challenge number {}, come up with a clever regex to capture this: \n\n\n",
            i
        );
        let string_to_display = self.produce_user_facing_string();
        println!("{}", string_to_display);
        println!("[-] Extract --> {} <--", self.search_string);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        let regex_is_correct = Regex::new(input.trim())
            .map(|re| is_good_regex(re, self.search_string))
            .unwrap_or(false);

        if regex_is_correct {
            println!("\n\nCorrect regex, well done :)\n\n");
            user.correct += 1;
            user.increment(level, true);
            true
        } else {
            println!("\n\nIncorrect regex, try again :/\n\n");
            user.wrong += 1;
            user.increment(level, false);
            false
        }
    }

    pub fn produce_user_facing_string(&self) -> String {
        let filler_words = self.filler_string.unwrap_or("");
        let search_string = self.search_string;

        match self.filler_order {
            FillerOrder::Before => format!("'{} {}'", filler_words, search_string),
            FillerOrder::After => format!("'{} {}'", search_string, filler_words),
            FillerOrder::Throughout => format!("'{}'", search_string),
            FillerOrder::Void => format!("'{}'", search_string),
        }
    }
}

/// An enum dictating how filler words are spread throughout the final string that is shown
/// to the user.
pub enum FillerOrder {
    Before,
    After,
    Throughout,
    Void,
}
