fn main() {
	let deck_size = get_deck_size();
	let spells = get_spell_count(deck_size);

	calculate_mana(deck_size, spells);

	fn get_deck_size() -> f32 {
		loop {
			let mut buffer = String::new();
			let mut input = 0.0;
			println!("How many cards are in your deck?");
			std::io::stdin().read_line(&mut buffer).expect("Failed to read input.");
			input = buffer.trim().parse::<f32>().expect("Please enter a valid number.");

			if input >= 40.0 {
				break input
			}

			println!("Your deck is too small, the minimum amount of cards in a deck is 40, please enter a new deck size.")
		}
	}

	fn get_spell_count(deck_size: f32) -> f32 {
		loop {
			let mut buffer = String::new();
			let mut input = 0.0;
			println!("How many spells are in your deck?");
			std::io::stdin().read_line(&mut buffer).expect("Failed to read input.");
			input = buffer.trim().parse::<f32>().expect("Please enter a valid number.");

			if input <= deck_size {
				break input
			}

			println!("You have more spells in your deck than the amount of cards in your deck, please enter a new number of spells.")
		}
	}

	fn calculate_mana(deck_size: f32, spells: f32) {
		let mut symbol_count = Vec::new();
		let colors = ["white", "blue", "green", "red", "black", "colorless"];
		let total_lands = deck_size - spells;

		for i in colors.iter() {
			let mut buffer = String::new();
			println!("How many {} symbols are in the deck?", i);
			std::io::stdin().read_line(&mut buffer).expect("Failed to read input.");
			let color = (buffer.trim().parse::<f32>().expect("Please enter a valid number."), i);
			symbol_count.push(color);
		}

		println!("You should have {} total land", total_lands);

		for i in symbol_count {
			if i.0 > 0.0 {
				let x = ((total_lands / spells) * i.0).round();
				println!("{} of those lands should be {}", x, i.1);
			}
		}
	}
}