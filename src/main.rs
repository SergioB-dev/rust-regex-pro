use clap::{App, Arg};
use cli_regex::game_flow::{begin_game, show_ascii_art};
use cli_regex::terminal_controls::clear_screen;
// mod questions;

fn main() {
    clear_screen();
    show_ascii_art();
    #[allow(unused_mut)]
    let mut running_game = begin_game();
    clear_screen();
    show_ascii_art();
    running_game.ask_question();

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
    running_game.print_game_details();
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config is: {}", config);
}
