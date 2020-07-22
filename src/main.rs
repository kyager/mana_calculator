fn main() {
	calculate_mana();

	fn get_deck_size() -> f32 {
		let mut buffer = String::new();
		println!("How many cards are in your deck?");
		std::io::stdin().read_line(&mut buffer).expect("Failed.");

		buffer.trim().parse::<f32>().unwrap()
	}

	fn get_spell_count() -> f32 {
		let mut buffer = String::new();
		println!("How many spells are in your deck?");
		std::io::stdin().read_line(&mut buffer).expect("Failed");

		buffer.trim().parse::<f32>().unwrap()
	}

	fn calculate_mana() {
		let colors = ["white", "blue", "green", "red", "black", "colorless"];
		let deck_size = get_deck_size();
		let spells = get_spell_count();
		let mut symbol_count = Vec::new();

		for i in &colors {
			let mut buffer = String::new();
			println!("How many {} symbols are in the deck?", i);
			std::io::stdin().read_line(&mut buffer).expect("Failed.");
			let color = (buffer.trim().parse::<f32>().unwrap(), i);
			symbol_count.push(color);
		}

		let total_lands = deck_size - spells;

		println!("You should have {} total land", total_lands);

		for i in symbol_count {
			if i.0 > 0.0 {
				let x = ((total_lands / spells) * i.0).round();
				println!("{} of those lands should be {}", x, i.1);
			}
		}
	}
}