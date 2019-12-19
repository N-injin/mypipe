extern crate clap;
use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("MyPipe")
        .version("1.0")
        .about("Pipe operator implementation")
        .author("Jim Bienvenu")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("input")
                .help("Input command")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("ouput")
                .help("Ouput command")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

        let input_command = matches.value_of("in").unwrap();
        let output_command = matches.value_of("out").unwrap();


        println!("Input : {} \nOutput : {}", input_command, output_command);
}
