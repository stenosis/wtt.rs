extern crate curl;
extern crate directories;
extern crate ansi_term;
extern crate crossterm;

use std::io::{Write};
use std::io::prelude::*;
use std::fs::File;
use curl::easy::{Easy};
use directories::{UserDirs};
use ansi_term::Colour::*;
use crossterm::terminal::{terminal,ClearType};
use crossterm::Screen;

mod config;

fn main() {

    // load config from parsed arguments
    let mut config = config::Config::new();
    config.parse_arguemnts();

    if config.show_help {
        print_help();

    } else {
        if config.save {
            write_city_to_disk(&config.city).unwrap();

        } else if config.city.len() == 0 {

            match load_city_from_disk() {
                Ok(c) => config.city = c,
                Err(_) => ()
            }
        }

        // check if we want to print the moonphase insteadt of weather
        if config.show_moon {
            config.city = String::from("moon");
        }

        // make sure we got some data to look for
        if config.city.len() > 0 {
            // determine the weather information from wttr.in and print it
            get_wttr_information(&config);

        } else {
            println!("Please specify a city with -c");
        }
    }
}

/// Writes a given city location name to a config file inside the users home folder.
fn write_city_to_disk(city: &str) -> std::io::Result<()> {

    if let Some(user_dirs) = UserDirs::new() {
        let path = format!("{}{}", user_dirs.home_dir().display(), "/.wttr");
        let mut buffer = File::create(path)?;
        buffer.write_all(city.as_bytes())?;
    }
     Ok(())
}

/// Loads a saved city name from the config file inside the users home folder.
fn load_city_from_disk() -> Result<String, String> {

    let mut city = String::from("");
    let user_dirs = UserDirs::new().unwrap();
    let file_path = format!("{}{}", user_dirs.home_dir().display(), "/.wttr");

    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            file.read_to_string(&mut city)
                .map_err(|err| err.to_string())
                .map(|_| city)
        })
}

/// Gets the weather information for the given city location
/// from wttr.in and prints the result into the std out.
fn get_wttr_information(config: &config::Config) {

    let url = format!("wttr.in/{}", &config.city);
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.useragent("curl/7.37.0").unwrap();
    let mut dst = Vec::new();
    
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    };

    // transfered data and std output
    let data = String::from_utf8(dst).unwrap();
    let mut output = String::from("");

    // show only the current weather data
    if config.show_now && !config.show_moon {

        for (i, line) in data.lines().enumerate() {
            if i < 7 {
                output = format!("{}\n{}", output, line)
            } else {
                output += "\n";
                break;
            }
        }

    // show the entire weather data
    } else {
        output = format!("\n{}", data);
    }
    println!("{}", output);
}

/// Prints some simple program instructions
fn print_help() {

    clear_screen();
    println!("{}", White.bold().paint("Wtt.rs"));
    println!("A simple wttr.in wrapper for Unix terminals, written in Rust. Has the possibility to save the used city location.\n");
    println!("\t-c or --City to specify the located city");
    println!("\t-s or --Save for saving the located city into a config file");
    println!("\t-m or --Moon for getting information about the current mondphase");
    println!("\t-n or --Now for displaying only the current weather data");
    println!("\t-h or --Help for printing the command list\n");
}

/// Clears the terminal screen
fn clear_screen() {

    // println!("{}[2J", 27 as char);
    let screen = Screen::default();
    terminal(&screen).clear(ClearType::All);
}