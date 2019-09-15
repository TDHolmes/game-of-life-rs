use std::time::Duration;
use std::path::Path;
use clap::{App, Arg, value_t};

use gameoflife::app;


fn main() {
    let matches = App::new("game-of-life")
        .about("An implementation of Conway's Game of Life.")
        .arg(Arg::with_name("rows")
            .short("r")
            .value_name("ROWS")
            .help("Number of rows in the grid")
            .takes_value(true))
        .arg(Arg::with_name("cols")
            .short("c")
            .value_name("COLS")
            .help("Number of columns in the grid")
            .takes_value(true))
        .arg(Arg::with_name("rand-density")
            .short("p")
            .help("Probability that a spot is alive at the beginning - [0,1]")
            .takes_value(true))
        .arg(Arg::with_name("rate")
            .long("rate")
            .help("Speed of the refresh cycles in miliseconds")
            .takes_value(true))
        .arg(Arg::with_name("config-filepath")
            .long("config-filepath")
            .short("f")
            .help("Board configuration file. Supports custom JSON or standard RLE. See http://www.conwaylife.com/wiki/Run_Length_Encoded for more info.")
            .takes_value(true)
    ).get_matches();

    // argument unwrapping / parsing
    let rows = value_t!(matches, "rows", usize).unwrap_or(40);
    let cols = value_t!(matches, "cols", usize).unwrap_or(80);
    let dur = value_t!(matches, "rate", u64).unwrap_or(250);
    let rand_prob = value_t!(matches, "rand-density", f32).unwrap_or(0.25);
    let path_str_opt = matches.value_of("config-filepath");

    // coax some types
    let duration = Duration::from_millis(dur);

    let mut path: Option<&Path> = None;
    if let Some(path_string) = path_str_opt {
        let p: &Path = Path::new(path_string);
        path = Some(p);
    }

    app::app(rows, cols, Some(rand_prob), path, duration);
}
