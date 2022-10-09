use rand::Rng;
use std::io;
fn main() {
    while "start"== "start" {
        println!("");
        println!("");
        println!("Enter a number from 1 - 10");
        println!("###########################");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        
        let num = rand::thread_rng().gen_range(0..10);
        if input.trim().parse::<u32>().unwrap() == num {
            println!("Great well done the predicted number was {}", num);
        } else {
            println!("Sorry try again the predicted number was {}", num);
        }
    }
}
