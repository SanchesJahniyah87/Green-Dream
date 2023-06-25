// Importing necessary packages and modules
use std::io::{stdin, stdout, Write};

// Definition of struct representing an item
// in the store.
struct Item {
	name: String,
	price: f32,
	organic: bool,
	sustainably_sourced: bool

}

fn main() {
	let mut items = vec![
		Item { 
			name: "apple".to_string(), 
			price: 2.99, 
			organic: true, 
			sustainably_sourced: true 
		},
		Item { 
			name: "broccoli".to_string(), 
			price: 4.99, 
			organic: true, 
			sustainably_sourced: false 
		},
		Item { 
			name: "salmon".to_string(), 
			price: 8.99, 
			organic: false, 
			sustainably_sourced: true 
		},
		Item { 
			name: "lettuce".to_string(), 
			price: 4.99, 
			organic: true, 
			sustainably_sourced: false 
		}
	];

	println!("Welcome to the eco-friendly grocery store!");

	loop {
		println!("Please enter the name of the item you would like to purchase or 'quit' to exit.");
	
		let mut input = String::new();
		stdin().read_line(&mut input).expect("Failed to read line");
		let item_name = input.trim();

		if item_name == "quit" {
			break;
		}

		let mut item_found = false;
		let mut item_index = 0;
		let mut total_price = 0.0;
		while item_index < items.len() {
			let current_item = &items[item_index];
			if current_item.name == item_name {
				item_found = true;
				total_price += current_item.price;
				if current_item.organic {
					println!("This item is organic.");
				}
				if current_item.sustainably_sourced {
					println!("This item is sustainably sourced.");
				}
				break;
			}
			item_index += 1;
		}

		if item_found {
			println!("The total price is {}.", total_price);
		} else {
			println!("Sorry, we don't have that item.");
		}
	}

	println!("Thanks for shopping in our eco-friendly grocery store!");

}