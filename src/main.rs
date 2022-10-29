use postgres::{Client, Error, NoTls};

use std::io;

fn main() -> Result<(), Error> {
    let mut client: Client =
        Client::connect("postgres://postgres:root@localhost:5432/postgres", NoTls)?;
    println!("");
    println!("");
    println!("++++++++++++++++++++++++++++++++++");
    println!("Welcome to Rust Database CLI CRUD");
    println!("++++++++++++++++++++++++++++++++++");
    println!("");
    while "true" == "true" {
        println!("1. Insert Data");
        println!("2. Edit Data");
        println!("3. Delete Data");
        println!("4. Display Data");
        println!("5. Query Data");
        println!("6. Exit");
        let mut menu_item = String::new();
        io::stdin()
            .read_line(&mut menu_item)
            .expect("failed to readline");

        match menu_item.trim().parse::<i32>().unwrap() {
            1 => {
                println!("");
                println!("*************");
                println!("Input Data");
                println!("*************");
                let mut username = String::new();
                println!("Enter the person name below :");
                io::stdin()
                    .read_line(&mut username)
                    .expect("failed to readline");

                let data = None::<&[u8]>;
                client.execute(
                    "INSERT INTO person (name, data) VALUES ($1, $2)",
                    &[&username, &data],
                )?;
                println!("");
                println!("{} - Inserted succesful", username);
            }
            2 => {
                println!("");
                println!("***********");
                println!("Edit Data");
                println!("***********");
                for row in client.query("SELECT id, name, data FROM person", &[])? {
                    let id: i32 = row.get(0);
                    let name: &str = row.get(1);
                    let data: Option<&[u8]> = row.get(2);

                    println!(" {} {} {:?}", id, name, data);
                }
                println!("");
                println!("Enter person Id:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("failed to readline");
                let mut newusername = String::new();
                io::stdin()
                    .read_line(&mut newusername)
                    .expect("error input");
                let  index = index.trim().parse::<i32>().unwrap();
                client.execute(
                    "UPDATE person SET
             name=$1
            WHERE id=$2",
                    &[&newusername, &index],
                )?;
                println!("Row successfully update");
            }
            3 => {
                println!("");
                println!("***********");
                println!("Delete Data");
                println!("***********");

                println!("Enter an id:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Please enter a number");
                let index = index.trim().parse::<i32>().unwrap();
                client.execute(
                    "DELETE FROM person
            WHERE id = $1",
                    &[&index]
                )?;
                println!("Delete was successfully");
            }
            4 => {
                println!("");
                println!("***********");
                println!("Display All Data");
                println!("***********");
                for row in client.query("SELECT id, name, data FROM person", &[])? {
                    let id: i32 = row.get(0);
                    let name: &str = row.get(1);
                    let data: Option<&[u8]> = row.get(2);

                    println!("{} {} {:?}", id, name, data);
                }
            }
            5 => {
                println!("");
                println!("***********");
                println!("Query a row");
                println!("***********");
                println!("Enter an id:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Please enter a number");
                let index = index.trim().parse::<i32>().unwrap();
                for row in client.query("SELECT  name, data FROM person  WHERE id =$1", &[&index])?
                {
                    let name: &str = row.get(0);
                    let data: Option<&[u8]> = row.get(1);

                    println!("Selected row has {} {:?} ",name, data);
                }
            }
            6 => {
                break;
            }
            _ => println!("Rest of the number"),
        }
        //    break;
    }
    
    Ok(())
}
