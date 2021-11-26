use regex::Regex;
use std::io::stdin;

use crate::regex_qa::is_good_regex;

/// A function that takes a search string such as: '2003-10-20' and does something. The function is incomplete at this point.
/// `Result` indicates whether the user's answer was right or wrong, we should do something with that.
pub fn ask_user_question(search_string: &str) {
    println!("As your first challenge, come up with a clever regex to capture this: \n\n\n\n");
    println!("\t \t--> {} <--", search_string);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    let re = Regex::new(&input.trim()).unwrap();

    // Print if the regex is correct or wrong
    match is_good_regex(re, search_string) {
        true => println!("Correct"),
        false => println!("Wrong"),
    }

    //TODO: Do something with our result, preferably increment the User's score / ranking
}
