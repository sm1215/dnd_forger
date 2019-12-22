use std::io;
use std::string::String;
use std::fmt;
use std::fs;
use std::num;

struct Item {
    name: String,
    value: u32
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}gp", self.name, self.value)
    }
}

struct Command {
    name: String,
    description: String,
}

// 1: create a list of weapons / armors
// 2: user inputs materials to break down
// 3: search for item, add to list to break down
// 4: total up materials value list
// 5: filter armory list based on what user can afford
fn main() {
    let mut armory: Vec<Item> = Vec::new();
    let contents = fs::read_to_string("armory.csv")
        .expect("Something went wrong loading the armory.");

    for mut line in contents.lines() {
        line = line.trim();
        let entry: Vec<&str> = line.split(",").collect();

        let name = entry[0].to_string();

        let value: Result<u32, num::ParseIntError> = entry[1].parse();
        let value: u32 = match value {
            Ok(val) => val,
            Err(err) => { continue },
        };

        let item = Item {
            name,
            value,
        };

        armory.push(item);
    }

    // todo: add more commands
    // let commands = [
    //     Command {
    //         name: String::from("list armory"),
    //         description: String::from("List the contents of the armory"),
    //     },
    //     Command {
    //         name: String::from("search"),
    //         description: String::from("Search for an item name in the armory"),
    //     },
    //     Command {
    //         name: String::from("add <item_name>"),
    //         description: String::from("Adds an item from the armory to your components list"),
    //     },
    //     Command {
    //         name: String::from(""),
    //         description: String::from(""),
    //     },
    // ];

    let mut materials: Vec<&Item> = Vec::new();
    let mut total_value: u32 = 0;

    loop {
        let mut input = String::new();
        println!("Please input an item name or type 'forge' to see what you can craft...");

        io::stdin().read_line(&mut input)
            .expect("error: expected input but didn't receive");
        
        input = input.trim().to_lowercase();

        match input.as_ref() {
            "list" => print_list(&armory),
            "forge" => break,
            _ => println!("try again"),
        }

        if input == "forge" {
            break;
        }

        let mut found = false;
        for item in &armory {
            if input == item.name.to_lowercase() {
                found = true;
                println!("Adding {}", item.name);
                materials.push(item);
            }
        }

        if found == false {
            println!("Item {} not found", input);
        }

        total_value = materials.iter().fold(0, |acc, item| acc + item.value);
        println!("Total Value of materials: {}", total_value);
    }

    println!("Ok, forging, you have {}gp to spend. Here are your options...", total_value);
    for item in armory.iter().filter(|item| item.value <= total_value) {
        println!("{}", item);
    }
}

fn print_list(list: &[Item]) -> () {
    for item in list {
        println!("{}", item);
    }
    ()
}