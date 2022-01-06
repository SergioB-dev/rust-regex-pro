use clap::{App, Arg};

use lib::constants::preset_questions::NOOB_QUESTIONS;
use lib::lib::game_flow::{begin_game, present_questions, show_game_header};
use lib::lib::terminal_controls::clear_screen;
use lib::user::User;

// mod questions;

fn main() {
    let mut user = User::default();
    clear_screen();
    show_game_header(&user);

    // let this = "hello".on_dark_red();
    // println!("{}", this);

    #[allow(unused_mut)]
    let mut running_game = begin_game();
    let level = running_game.level;

    clear_screen();
    show_game_header(&user);
    present_questions(NOOB_QUESTIONS, &mut user, &level);

    let matches = App::new("First Test")
        .version("1.0")
        .author("Sergio Bost <mail@sergio.dev>")
        .about("Do something awesome")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        // .arg(Arg::with_name("Level")
        //     .short("l")
        //     .long("level")
        //     .help("Help")
        //     .required(true)
        //     //.index(1)
        //     .takes_value(true)
        .get_matches();

    //println!("{}", running_game.message.unwrap());

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config is: {}", config);
}
