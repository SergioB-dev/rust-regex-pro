

pub mod basic {
    pub struct Game {
        pub name: String,
        pub level: Level,
        pub message: Option<String>,
    }

    impl Game {
        pub fn basic() -> Game {
            Game { name: "Player 1".to_string(), level: Level::Easy, message: None }
        }

        pub fn create_level_based_game(chosen_level: Level) -> Game {
            let mut beginning_message = "".to_string();
            match chosen_level {
                Level::Easy => beginning_message = "Alright, you better score 100%".to_string(),
                Level::Medium => beginning_message = "Alright, let's see what you can do!".to_string(),
                Level::Hard => beginning_message = "Oh boy, feeling good are we?".to_string()
            }

            Game { name: "Player 1".to_string(), level: chosen_level, message: Some(beginning_message) }
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
        Hard
    }
}

pub mod game_flow {
    use std::error::Error;
    use std::io;
    use std::io::{stdout, Write};
    use crossterm::cursor::Hide;
    use crossterm::{ExecutableCommand, execute, QueueableCommand};
    use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen};
    use crate::basic::{Game, Level};
    use crate::basic::Level::Easy;


    pub fn show_ascii_art() -> Result<(), Box<dyn Error>>{

        let asci_art = r#"
        __________                                        __________
\______   \ ____   ____   ____ ___  ___           \______   \_______  ____
 |       _// __ \ / ___\_/ __ \\  \/  /   ______   |     ___/\_  __ \/  _ \
 |    |   \  ___// /_/  >  ___/ >    <   /_____/   |    |     |  | \(  <_> )
 |____|_  /\___  >___  / \___  >__/\_ \            |____|     |__|   \____/
        \/     \/_____/      \/      \/
        "#;

        let mut stdout = io::stdout();
        println!("{}", asci_art);
        Ok(())
    }
    pub fn begin_game() -> Game {
        let mut s = String::new();
        println!("Select a difficulty");
        println!("1: Easy, 2: Medium, 3: Hard");
        let mut difficulty = String::new();

        io::stdin()
            .read_line( &mut difficulty)
            .expect("Failed to read line");

        println!("{}", difficulty);
        let mut chosen_level: Level = Easy;
        match difficulty.trim().parse::<i32>().unwrap() {

                2     => {chosen_level = Level::Medium; println!("Choosing Medium")},
                3     => {chosen_level = Level::Hard},
                _       => (),
        };
        println!("{:?}", chosen_level);
        Game::create_level_based_game(chosen_level)
    }
}

pub mod terminal_controls {
    pub fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();

    }
}