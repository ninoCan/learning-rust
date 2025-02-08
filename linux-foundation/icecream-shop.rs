use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Icecream {
    flavour: String,
    price: f32,
}

fn icecream_flavours() -> HashMap<String, Icecream> {
    let mut menu = HashMap::new();
    menu.insert(
        "Strawberry".to_string(),
        Icecream {
            flavour: "Strawberry".to_string(),
            price: 1.00,
        },
    );
    menu.insert(
        "Lemon".to_string(),
        Icecream {
            flavour: "Lemon".to_string(),
            price: 1.00,
        },
    );
    menu.insert(
        "Chocolate".to_string(),
        Icecream {
            flavour: "Chocolate".to_string(),
            price: 2.00,
        },
    );
    menu
}

#[derive(Debug)]
struct Order {
    icecream: String,
    quantity: u32,
}

impl Order {
    fn total_price(&self, icecream_menu: &HashMap<String, Icecream>) -> Option<f32> {
        match icecream_menu.get(&self.icecream) {
            Some(icecream) => Some(icecream.price * self.quantity as f32),
            None => None,
        }
    }
}

fn main() {
    let icecreams = icecream_flavours();
    let mut orders: Vec<Order> = Vec::new();

    loop {
        println!("Available Icecreams:");
        for (_, icecream) in &icecreams {
            println!("{} - ${:.2}", Icecream.flavour, icecream.price);
        }

        println!("Enter your icecream choice (or 'q' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let icecream_choice = input.trim().to_string();
        input.clear();

        if icecream_choice == "q" {
            break;
        }

        println!("Enter quantity:");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let quantity: u16 = match input.trim().parse() {
            Ok(qty) => qty,
            Err(_) => {
                println!("Invalid quantity. Please enter a valid number.");
                continue;
            }
        };
        input.clear();

        if !icecream_menu.contains_key(&icecream_choice) {
            println!("Invalid icecream choice. Please select a icecream from the menu.");
            continue;
        }

        let order = Order {
            icecream: icecream_choice.clone(),
            quantity,
        };
        orders.push(order);
    }

    println!("Your order:");
    let mut total_cost = 0.0;
    for order in &orders {
        println!(
            "{} - {} x ${:.2} each = ${:.2}",
            order.icecream,
            order.quantity,
            icecream_menu[&order.icecream].price,
            order.total_price(&icecream_menu).unwrap()
        );
        total_cost += order.total_price(&icecream_menu).unwrap();
    }
    println!("Total Cost: ${:.2}", total_cost);
}
