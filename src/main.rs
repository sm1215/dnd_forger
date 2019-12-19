use std::io;
use std::string::String;
use std::fmt;

struct Item {
    name: String,
    value: u32
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}gp", self.name, self.value)
    }
}

// 1: create a list of weapons / armors
// 2: user inputs materials to break down
// 3: search for item, add to list to break down
// 4: total up materials value list
// 5: filter armory list based on what user can afford
fn main() {
    let armory = [
        Item { name: String::from("Padded"), value: 5 },
        Item { name: String::from("Leather"), value: 10 },
        Item { name: String::from("Studded Leather"), value: 45 },
        Item { name: String::from("Hide"), value: 10 },
        Item { name: String::from("Chain Shirt"), value: 50 },
        Item { name: String::from("Scale Mail"), value: 50 },
        Item { name: String::from("Breastplate"), value: 400 },
        Item { name: String::from("Half plate"), value: 750 },
        Item { name: String::from("Ring mail"), value: 30 },
        Item { name: String::from("Chain mail"), value: 75 },
        Item { name: String::from("Splint"), value: 200 },
        Item { name: String::from("Plate"), value: 1500 },
        Item { name: String::from("Shield"), value: 10 },
    ];
    let mut materials: Vec<&Item> = Vec::new();
    let mut total_value: u32 = 0;

    loop {
        let mut input = String::new();
        println!("Please input an item name or type 'forge' to see what you can craft...");

        io::stdin().read_line(&mut input)
            .expect("error: expected input but didn't receive");
        
        input = input.trim().to_lowercase();

        println!("{}", input);
        if input == "forge" {
            break;
        }

        let mut found = false;
        for item in &armory {
            if command == item.name.to_lowercase() {
                found = true;
                println!("Adding {}", item.name);
                materials.push(item);
            }
        }

        if found == false {
            println!("Item {} not found", command);
        }

        total_value = materials.iter().fold(0, |acc, item| acc + item.value);
        println!("Total Value of materials: {}", total_value);
    }

    println!("Ok, forging, you have {}gp to spend. Here are your options...", total_value);
    for item in armory.iter().filter(|item| item.value <= total_value) {
        println!("{}", item);
    }
}
