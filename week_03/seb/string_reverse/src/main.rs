use std::io;

fn reverse(word: &String) -> String {
	let reverse_word = word.chars().rev().collect();
	reverse_word
}

fn reverse_manual(word: &String) -> String {
    let size = word.len();
    let mut reverse_chars = vec![' '; size];
    let chars = word.chars();

    for (i, character) in chars.enumerate() {
        reverse_chars[size - 1 - i] = character;
    }

    reverse_chars.into_iter().collect()
}

fn main() {

	loop {
    	println!("Type a word and I'll reverse it!");
	
    	let mut word = String::new();
	
    	io::stdin().read_line(&mut word).expect("Failed to read line");
	
    	let reverse_word = reverse_manual(&word);
	
    	println!("{} backwards is : {}", word.trim(), reverse_word);
    	println!("Try again? y/n");
	
    	let mut try_again = String::new();
	
    	io::stdin().read_line(&mut try_again).expect("Failed to read line");
	
    	match try_again.trim() {
    			"n" => {
    				println!("Goodbye!");
    				break;
    				},
    			"y" => continue,
    			_ => {
                    println!("I don't understand. Goodbye!");
                    break;
                }
    		}
    }
}
