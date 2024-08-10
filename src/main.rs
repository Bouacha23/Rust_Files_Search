use  std::env ;
use std::fs ; 




fn main () {
    let arg : Vec<String > = env::args().collect() ; 
    
    let arg  = parse_config(&arg);
        println!("seraching for  {}" , arg.file_path) ;
    println!("in file path {}" , arg.query) ;
    let contents = fs::read_to_string(tuple.1)
    .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

struct args {
    query  : String ,
    file_path  : String ,
}


fn parse_config (arg : &[String]) -> args {
    let  query :String = arg[1].clone() ;
    let  file_path : String  =  arg[2].clone() ;
    args {
        query  , 
        file_path  ,
    }
}