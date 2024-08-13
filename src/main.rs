use  std::env ;
use std::process ;
use minigrep::{Config ,run} ;





fn main () {
    let args : Vec<String > = env::args().collect() ; 
    
    let arg: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("seraching for  {}" , arg.file_path) ;
    println!("in file path {}" , arg.query) ;

    if let Err(e) = run(arg) {
        println!("Application error: {e}");
        process::exit(1);
    }
   
}

