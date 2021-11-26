mod explanations;
mod user;

/// Basic definitions of objects
pub mod basic {
    pub struct Game {
        pub name: String,
        pub level: Level,
        pub message: Option<String>,
    }

    impl Game {
        pub fn basic() -> Game {
            Game {
                name: "Player 1".to_string(),
                level: Level::Easy,
                message: None,
            }
        }

        pub fn create_level_based_game(chosen_level: Level) -> Game {
            let beginning_message = match chosen_level {
                Level::Easy => "Alright, you better score 100%".to_string(),
                Level::Medium => "Alright, let's see what you can do!".to_string(),
                Level::Hard => "Oh boy, feeling good are we?".to_string(),
            };

            Game {
                name: "Player 1".to_string(),
                level: chosen_level,
                message: Some(beginning_message),
            }
        }

        pub fn print_game_details(&self) {
            println!("Player's name is: {}", self.name);
            println!("Chosen level is: {:?}", self.level);
        }
    }
    #[derive(Debug)]
    pub enum Level {
        Easy,
        Medium,
        Hard,
    }
}

/// Control panel for the flow of the game from start to finish.
pub mod game_flow {
    use crate::basic::Level::Easy;
    use crate::basic::{Game, Level};
    use std::io;

    pub fn show_ascii_art() {
        let asci_art = r#"
        __________                                        __________
\______   \ ____   ____   ____ ___  ___           \______   \_______  ____
 |       _// __ \ / ___\_/ __ \\  \/  /   ______   |     ___/\_  __ \/  _ \
 |    |   \  ___// /_/  >  ___/ >    <   /_____/   |    |     |  | \(  <_> )
 |____|_  /\___  >___  / \___  >__/\_ \            |____|     |__|   \____/
        \/     \/_____/      \/      \/
        "#;

        println!("{}", asci_art);
    }
    pub fn begin_game() -> Game {
        println!("Select a difficulty");
        println!("1: Easy, 2: Medium, 3: Hard");
        let mut difficulty = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        println!("{}", difficulty);
        let chosen_level: Level = match difficulty.trim().parse::<i32>() {
            Ok(2) => Level::Medium,
            Ok(3) => Level::Hard,
            Ok(_) => Level::Easy,
            Err(_err) => {
                // TODO handle error in case the input is not an integer
                Level::Easy
            }
        };
        println!("{:?}", chosen_level);
        Game::create_level_based_game(chosen_level)
    }
}

/// Basic terminal controls
pub mod terminal_controls {
    pub fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();
    }
}

/// Regex based questions and answers
pub mod regex_qa {
    use regex::Regex;

    pub fn is_good_regex(r: Regex, query_string: &str) -> bool {
        r.is_match(query_string)
    }

    #[cfg(test)]
    mod tests {
        use crate::regex_qa::is_good_regex;
        use regex::Regex;

        #[test]
        fn basic_regex() {
            let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
            let search_string = "2014-02-01";
            assert!(is_good_regex(re, search_string))
        }
    }
}
