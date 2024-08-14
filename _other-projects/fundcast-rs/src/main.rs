use clap::{Arg, ArgAction, Command};

fn main() {
    let arg_matches = Command::new("fundcast")
        .version("0.1.0")
        .author("Alec Hampton")
        .about("Simple utility to forecast your finances")
        .get_matches();

        
    println!("Accounts");
    println!("--------------");

}
