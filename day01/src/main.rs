use std::env;
use std::fs;

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let mut top_calories: [i32; 4] = [0; 4];

    //let mut max_calories: i32 = 0;
    let mut curr_calories: i32 = 0;
    let mut is_word: bool = true;

    let mut word: String = String::new();
    for c in input_contents.chars() {
        if c == '\n' && is_word {
            is_word = false;
            println!("Word: {word}");
            let calories = word.parse::<i32>().unwrap();
            word.clear();

            curr_calories += calories;
        } else if c == '\n' && !is_word {
            println!("Read elf: {curr_calories}");
            top_calories[3] = curr_calories;
            top_calories.sort_by(|a, b| b.cmp(a));
            //if curr_calories >= max_calories {
            //max_calories = curr_calories;
            //}
            curr_calories = 0;
        } else {
            is_word = true;
            word.push(c);
        }
    }
    println!("Read elf: {curr_calories}");
    top_calories[3] = curr_calories;
    top_calories.sort_by(|a, b| b.cmp(a));

    println!("Top calories: {:?}", top_calories);

    let mut final_calories = 0;
    for n in 0..3 {
        final_calories += top_calories[n];
    }

    println!("Final calories: {final_calories}");

    //if curr_calories >= max_calories {
    //max_calories = curr_calories;
    //}
    //println!("Max calories: {max_calories}");
}
