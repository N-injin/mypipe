extern crate clap;
use clap::{App, Arg};
use std::process::{Command, Stdio};
use std::io::Result;

fn main() -> Result<()> {
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

        let input_command_string = matches.value_of("in").unwrap();
        let output_command_string = matches.value_of("out").unwrap();

        let input_command = Command::new("sh")
                                    .arg("-c")
                                    .arg(input_command_string)
                                    .stdout(Stdio::piped())
                                    .spawn()?;

        let output_command = Command::new("sh")
                                     .arg("-c")
                                     .arg(output_command_string)
                                     .stdin(input_command.stdout.unwrap())
                                     .output()
                                     .expect("Failed to execute process");

        if !output_command.status.success() {
            println!("{}{}", String::from_utf8_lossy(&output_command.stderr), output_command.status);
            std::process::exit(output_command.status.code().unwrap());
        }

        println!("{}", String::from_utf8_lossy(&output_command.stdout));
        Ok(())
}
