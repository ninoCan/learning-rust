use std::io;

struct House {
    house_type: String,
    year: u8,
    color: String,
    value: u32,
}

impl House {
    // Method to display car details
    fn display(&self) {
        println!(
            "house_type: {}\nYear: {}\nColor: {}\nValue: {} euros",
            self.make, self.model, self.year, self.color, self.mileage
        );
    }
}

fn main() {
    let house1 = get_house_details();
    let house2 = get_house_details();

    println!("House 1:");
    house1.display();

    println!("House 2:");
    house2.display();
}

fn get_house_details() -> House {
    println!("Enter house details:");
    let mut user_input = String::new();

    println!("House type:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user_input");
    let house_type = user_input.trim().to_string();
    user_input.clear();

    println!("Year:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user_input");
    let year: u8 = user_input.trim().parse().expect("Invalid input");
    user_input.clear();

    println!("Color:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user_input");
    let color = user_input.trim().to_string();
    user_input.clear();

    println!("Value (in euros):");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user_input");
    let value: u32 = user_input.trim().parse().expect("Invalid input");

    House {
        house_type,
        year,
        color,
        value,
    }
}
