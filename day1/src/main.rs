use std::{fs, env};
use std::path::Path;

fn main() {

    compute_number("input.txt".to_owned());

}

fn compute_number(file_path : String) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut current : String = "".to_owned();
    let mut total_score = 0;
    
    for c in contents.chars() {
        if c == '\n' {
            current = double_letter_if_needed(current);
            println!("Adding {current}");
            total_score += current.parse::<i32>().unwrap();
            current.clear();
            continue;
        }
        if c.is_digit(10) {
            if current.len() >= 2 {
                current.replace_range(1..2, c.to_string().as_str());
            }else{
                current.push(c);
            }
        }
    }
    //last line doesn't end with '\n'
    current = double_letter_if_needed(current);
    println!("Adding {current}");
    total_score += current.parse::<i32>().unwrap();

    println!("Total: {total_score}");
    return total_score;
}

fn double_letter_if_needed(current : String) -> String{
    let mut tmp : String = current.to_string();
    if tmp.len() == 1{
        tmp.push_str(&current);
    }
    return tmp;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use crate::compute_number;

    #[test]
    fn test_intro() {
        assert_eq!(compute_number("input.txt".to_owned()), 142);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(compute_number("input_puzzle.txt".to_owned()), 54159);
    }
}
