use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone(); 
    let name = "Deepak";
    let status =  "100%";
    // println!("Args: {:?}", command);   // Args: "hello"
    if command == "hello" {
        println!("Hi, How are you {}?", name);
    }
    else if command == "status"{
        println!("Status: {}", status);
    }
    else {
        println!("This is not a valid command!");
    }

}