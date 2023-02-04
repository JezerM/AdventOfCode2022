use std::fs;

#[derive(PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum EndChoice {
    Lose,
    Draw,
    Win,
}

enum GameState {
    Lose,
    Draw,
    Win,
}

fn char_to_choice(str: char) -> Option<Choice> {
    match str {
        'A' | 'X' => Some(Choice::Rock),
        'B' | 'Y' => Some(Choice::Paper),
        'C' | 'Z' => Some(Choice::Scissors),
        _ => None,
    }
}
fn char_to_end_choice(str: char) -> Option<EndChoice> {
    match str {
        'X' => Some(EndChoice::Lose),
        'Y' => Some(EndChoice::Draw),
        'Z' => Some(EndChoice::Win),
        _ => None,
    }
}

fn calculate_score(my_choice: &Choice, opp_choice: &Choice) -> u32 {
    let mut score: u32 = 0;

    score += match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let mut game_state: GameState;

    game_state = match (my_choice, opp_choice) {
        (Choice::Rock, Choice::Scissors) => GameState::Win,
        (Choice::Paper, Choice::Rock) => GameState::Win,
        (Choice::Scissors, Choice::Paper) => GameState::Win,
        _ => GameState::Lose,
    };
    if my_choice == opp_choice {
        game_state = GameState::Draw;
    }

    score += match game_state {
        GameState::Draw => 3,
        GameState::Win => 6,
        GameState::Lose => 0,
    };

    return score;
}

fn get_winner_choice(choice: &Choice) -> Choice {
    match choice {
        Choice::Scissors => Choice::Rock,
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissors,
    }
}

fn calculate_choice_and_score(how_to_end: &EndChoice, opp_choice: &Choice) -> u32 {
    let my_choice: Choice;

    match how_to_end {
        EndChoice::Draw => {
            my_choice = opp_choice.clone();
        }
        EndChoice::Lose => {
            my_choice = get_winner_choice(&get_winner_choice(&opp_choice));
        }
        EndChoice::Win => {
            my_choice = get_winner_choice(&opp_choice);
        }
    }

    return calculate_score(&my_choice, opp_choice);
}

fn get_score(input: &String) -> u32 {
    let mut my_choice: Option<Choice> = None;
    let mut opp_choice: Option<Choice> = None;

    let mut score: u32 = 0;

    for c in input.chars() {
        if c == '\n' {
            if my_choice.is_none() || opp_choice.is_none() {
                continue;
            }
            score += calculate_score(&my_choice.unwrap(), &opp_choice.unwrap());
            my_choice = None;
            opp_choice = None;
            continue;
        }
        if opp_choice.is_none() {
            opp_choice = char_to_choice(c);
        } else if my_choice.is_none() {
            my_choice = char_to_choice(c);
        }
    }
    return score;
}
fn get_score_choice(input: &String) -> u32 {
    let mut my_choice: Option<Choice> = None;
    let mut opp_choice: Option<Choice> = None;
    let mut end_choice: Option<EndChoice> = None;

    let mut score: u32 = 0;

    for c in input.chars() {
        if c == '\n' {
            if my_choice.is_none() || opp_choice.is_none() {
                continue;
            }
            score += calculate_choice_and_score(&end_choice.unwrap(), &opp_choice.unwrap());
            my_choice = None;
            opp_choice = None;
            end_choice = None;
            continue;
        }
        if opp_choice.is_none() {
            opp_choice = char_to_choice(c);
        } else if my_choice.is_none() {
            end_choice = char_to_end_choice(c);
            my_choice = char_to_choice(c);
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let score = get_score(&input_contents);
        assert_eq!(score, 15);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let score = get_score_choice(&input_contents);
        assert_eq!(score, 12);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let score_first = get_score(&input_contents);
    let score_second = get_score_choice(&input_contents);

    println!("Score (part 1): {}", score_first);
    println!("Score (part 2): {}", score_second);
}
