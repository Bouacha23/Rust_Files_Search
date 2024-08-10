use  std::env ;
use std::fs ; 


fn main () {
    let arg : Vec<String > = env::args().collect() ; 
    
   
    let  query :&String = &arg[1] ;
    let  file_path : &String  = & arg[2] ;

    println!("seraching for  {}" , query) ;
    println!("in file path {}" , file_path) ;

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

