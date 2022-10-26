use postgres::{Client, Error, NoTls};
use std::io;



fn main() -> Result<(), Error> {
    let mut client:Client = Client::connect(
        "postgres://postgres:root@localhost:5432/postgres",
        NoTls,
    )?;

    println!("++++++++++++++++++++++++++++++++++");
    println!("Welcome to Rust Database CLI CRUD");
    println!("++++++++++++++++++++++++++++++++++");
    println!("");
    while true{
        
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

execution(&client,name,data);

for row in client.query("SELECT id, name, data FROM person", &[])? {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let data: Option<&[u8]> = row.get(2);

    println!("found person: {} {} {:?}", id, name, data);
}
    Ok(())
}