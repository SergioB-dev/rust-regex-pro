pub mod constants;
mod explanations;
mod questions;

pub mod user;

/// Basic definitions of objects, traits, etc.
pub mod basic {
    use crate::questions;
    use crate::user::User;

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

    use crossterm::style::{Color, Colored, SetForegroundColor, Stylize};

    use crate::basic::{Game, Level};
    use crate::questions::Question;
    use crate::terminal_controls::clear_screen;
    use std::io;

    use crate::user::User;

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

    pub fn show_game_header(user: &User) {
        show_ascii_art();
        let user_win_pct = user.pct();
        let user_score = user.calculate_score();
        let user_ranking = user.get_user_ranking();
        println!(
            "\t%: {} \tScore: {} \tRank: {}",
            user_win_pct, user_score, user_ranking
        );
        println!("***********************************************\n\n")
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

    pub fn present_questions(questions: [Question; 3], user: &mut User) {
        for question in questions.into_iter() {
            question.ask_user_question(user);
            clear_screen();
            show_game_header(&user);
        }
    }
}

/// Basic terminal controls
pub mod terminal_controls {
    use std::io;

    use crossterm::{
        execute,
        terminal::{Clear, ClearType},
    };

    pub fn clear_screen() {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    }
}

/// Regex based questions and answers
pub mod regex_qa {
    use regex::Regex;

    pub fn is_good_regex(r: Regex, query_string: &str) -> bool {
        //FIXME: Buggy. User can enter erroneous regex and still be valid.
        if let Some(re) = r.captures(query_string) {
            re.get(0).map_or("", |m| m.as_str()) == query_string
        } else {
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::regex_qa::is_good_regex;
        use regex::Regex;

        #[test]
        fn assert_working_re() {
            let good_re = Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap();
            let search_string = "2014-02-01";
            assert!(is_good_regex(good_re, search_string))
        }

        #[test]
        fn spot_faulty_re() {
            let bad_re = Regex::new(r"\d{4}").unwrap();
            let search_string = "2004-12-12";
            assert_ne!(is_good_regex(bad_re, search_string), true)
        }
    }
}
