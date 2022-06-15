// Project goal: practice-cli foobar "text.txt"
// inspect all of text.txt for any lines that contain foobar
// Very common workflow is to create our program around the specific data
// we're going to be working with - thus we ought to create an abstraction
// to handle this data.
use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};
use log::{info, warn};

/// Command-line input is the data we're orienting our program around
#[derive(Parser)]
struct Cli {
    
    /// Pattern to look for in file
    pattern: String,
    
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> { // changing main return type
    
    // we can also show a progress bar
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
    
        // // example of printing to stderr, or standard error
        // // println!("This is information");
        // // eprintln!("This is an error! :(");

        let stdout = io::stdout(); // get the global stdout entity
        let mut handle = stdout.lock(); // acquire a lock on it
        writeln!(handle, "foo: {}", 42); // add '?' if you care about errors here


        // creating a parser
        let args = Cli::parse();

        // The final way to write it use to use the 'anyhow' crate, which helps with custom
        // error messages
        let content = std::fs::read_to_string(&args.path)
            .with_context(|| format!("Could not read file `{:?}`", &args.path))?;

        println!("Argument 1: {}", args.pattern);
        println!("Argument 2: {:?}", args.path);
        for line in content.lines() {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    } 

    Ok(())

}
