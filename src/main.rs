use std::env::args;
use std::process;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn print_help(){
    println!("\
pgen: A Rust based Utility to generate random passwords.
Takes a single unsigned 16 bit integer as argument.
Usage: 
    pgen [INT]
")
}

struct Config {
    length: u16
}

impl Config{
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            eprintln!("Invalid number of args.(Expected:1, Got:{})",args.len()-1);
            print_help();
            process::exit(1);
        }
        let length = args[1].parse::<u16>().unwrap();
        Ok(Config{length})
    }
}

fn gen_alphanum(n: u16){
    let randstring: String=thread_rng().sample_iter(&Alphanumeric).take(n.into()).map(char::from).collect();
    println!("{}",randstring);
}
fn main() {
    let arg: Vec<String> = args().collect();
    let conf = Config::new(&arg).unwrap_or_else(|err|{
        eprintln!("Some error occured. Error message: \n{}",err);
        process::exit(1)
    });
    gen_alphanum(conf.length);

    
}


