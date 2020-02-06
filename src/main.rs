extern crate rand;

use rand::Rng;
use std::env;
use std::fs;
use std::io;
use crossterm::cursor;
use crossterm::Terminal;
use std::{thread, time};


fn main() {
    println!("Hello, welcome to game hangman");
    
    
    let mut number_of_mistakes = 0;
    let cursor = cursor();
	let term = crossterm::Terminal::new();
    let mut available_chars = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    let text = fs::read_to_string(
		"C:\\Users\\yeshw\\Desktop\\Umass\\Research\\Rust\\projects\\hang_man_game\\dictionary.txt",
	).expect("Failed to read file");
	let mut random_word = "";
	let random_number = rand::thread_rng().gen_range(0, 263);
	let mut i = 0;
	let words = text.split("\r\n");
	for word in words {
		if i == random_number {
			random_word = word;
		}
		i = i + 1;
	}
	let mut hidden_word = String::from("");
	for _i in 0..random_word.len(){
		hidden_word.push('_');
	}
    loop{
		term.clear(crossterm::ClearType::FromCursorUp).expect("Failed to clear");
		let mut user_input = String::new();

		display_hang_man(number_of_mistakes);
		
		if number_of_mistakes > 5{
			break;
		}
        cursor.goto(25,6).expect("Failed to move cursor");
        print!("{:?}",&available_chars[0..available_chars.len()/2]);
  
        cursor.goto(25,8).expect("Failed to move cursor");
		print!("{:?}",&available_chars[available_chars.len()/2..]);
		
		cursor.goto(25,10).expect("Failed to move cursor");
		print!("{}", hidden_word);

        cursor.goto(0,15).expect("Failed to move cursor");
        println!("Enter a letter (Enter 0 to exit)");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        if user_input.trim() == "0"{
            break;
		} 
		let available = check_and_update_available_chars(&mut available_chars, user_input.trim());
		if !available{
			continue;
		}
		let check = check_in_word(random_word,user_input.trim(),&hidden_word);
		if check.0{
			hidden_word = check.1; 
		}
		if !check.0{
			number_of_mistakes = number_of_mistakes + 1;
		}
		if check.2 {
			break;
		}
		//thread::sleep(time::Duration::from_millis(10000));
    }
}
fn check_in_word(random_word: &str, guessed_letter: &str, hidden_word: &String) -> (bool, String, bool){
	let mut found = false;
	let mut new_hidden_word = String::from("");
	let mut game_done = true;
	for i in 0..random_word.len(){
		let word = &random_word[i..i+1];
		if guessed_letter.eq_ignore_ascii_case(word){
			found = true;
			new_hidden_word.push(guessed_letter.to_uppercase().chars().next().unwrap());
		}
		else{
			if &hidden_word[i..i+1] != "_"{
				new_hidden_word.push(hidden_word[i..i+1].to_uppercase().chars().next().unwrap());
			}
			else{
				game_done = false;
				new_hidden_word.push('_');
			}
		}
	}
	return (found,new_hidden_word,game_done);
}
fn check_and_update_available_chars(available_chars : &mut [&str; 26],  guessed_word: &str ) -> bool{
	if available_chars.contains(&&guessed_word.to_ascii_uppercase()[..]){
		let position = available_chars.iter().position(|&r| r.to_string() == guessed_word.to_ascii_uppercase()).unwrap();
		println!("{:?} postion", position);
		available_chars[position] = "_";
		return true;
	};
	return false;
}


fn display_hang_man(number_of_mistakes: u32,){
    let cursor = cursor();
    if number_of_mistakes == 0 { // if the errors left is 6 the following annimation is drawn
        cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 1 { // if the errors left is 5 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 2 { // if the errors left is 4 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |         |");
		println!( "   |         |");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 3 { // if the errors left is 3 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |        \\|");
		println!( "   |         |");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 4 { // if the errors left is 2 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |        \\|/");
		println!( "   |         |");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 5 { // if the errors left is 1 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |        \\|/");
		println!( "   |         |");
		println!( "   |        /");
		println!( "   |    ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
	else if number_of_mistakes == 6 { // if the errors left is 0 the following annimation is drawn
		cursor.goto(0,4).expect("failed to move");
		println!( "");
		println!( "_________________________");
		println!( "   |/	     |");
		println!( "   |        (_)");
		println!( "   |        \\|/");
		println!( "   |         |");
		println!( "   |        / \\");
		println!( "   |     DEAD!!!!! ");
		println!( "   |    ");
		println!( "   |    ");
		println!( "___|____");
	}
}