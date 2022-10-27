use postgres::{Client, Error, NoTls};
use std::io;



fn main() -> Result<(), Error> {
    let mut client:Client = Client::connect(
        "postgres://postgres:root@localhost:5432/postgres",
        NoTls,
    )?;
    println!("");
    
    println!("");
    println!("++++++++++++++++++++++++++++++++++");
    println!("Welcome to Rust Database CLI CRUD");
    println!("++++++++++++++++++++++++++++++++++");
    println!("");
    while "true"=="true"{
        println!("1. Insert Data");
        println!("2. Edit Data");
        println!("3. Delete Data");
        println!("4. Display Data");
        println!("5. Query Data");
        println!("6. Exit");
        let mut menuItem =String::new() ;
        io::stdin().read_line(&mut menuItem).expect("failed to readline");
 

       match menuItem.trim().parse::<i32>().unwrap(){
            1=>{
println!("test");
            },
            
            6=>{
                println!("test");
                            },
                            
            _=>println!("Rest of the number"),
            
          }
       break;
    }
//     client.batch_execute("
//     CREATE TABLE personname (
//         id      SERIAL PRIMARY KEY,
//         name    TEXT NOT NULL,
//         data    BYTEA
//     )
// ")?;

let name = "Ferris";
let data = None::<&[u8]>;
client.execute(
    "INSERT INTO person (name, data) VALUES ($1, $2)",
    &[&name, &data],
)?;



for row in client.query("SELECT id, name, data FROM person", &[])? {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let data: Option<&[u8]> = row.get(2);

    println!("found person: {} {} {:?}", id, name, data);
}
    Ok(())
}