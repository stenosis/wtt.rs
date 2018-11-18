use std::env;

// city location
const ARG_CITY_FULL: &str = "--City";
const ARG_CITY_SHORT: &str = "-c";

// print help information
const ARG_HELP_FULL: &str = "--Help";
const ARG_HELP_SHORT: &str = "-h";

// show moonphase informatin
const ARG_MOON_FULL: &str = "--Moon";
const ARG_MOON_SHORT: &str = "-m";

// save the city location to disk
const ARG_SAVE_FULL: &str = "--Save";
const ARG_SAVE_SHORT: &str = "-s";

/// The config is a representation of the program options
pub struct Config {
    pub city: String,
    pub show_moon: bool,
    pub show_help: bool,
    pub save: bool
}

impl Config {

    pub fn new() -> Config {
        Config {
            city: String::from(""),
            show_moon: false,
            show_help: false,
            save: false
        }
    }

    /// parse the application arguments into the config struct
    pub fn parse_arguemnts(&mut self) {

        let args: Vec<String> = env::args().collect();
        for i in 0 .. args.len() {

            // parse arguments with value
            if i+1 < args.len() {
                match args[i].as_str() {
                    
                    ARG_CITY_FULL => self.city = args[i+1].clone(),
                    ARG_CITY_SHORT => self.city = args[i+1].clone(),
                    _ => (),
                }
            }
            // check for flags only
            match args[i].as_str() {
                
                ARG_HELP_FULL => self.show_help = true,
                ARG_HELP_SHORT => self.show_help = true,

                ARG_MOON_FULL => self.show_moon = true,
                ARG_MOON_SHORT => self.show_moon = true,

                ARG_SAVE_FULL => self.save = true,
                ARG_SAVE_SHORT => self.save = true,

                _ => (),
            }
        }
    }
}
