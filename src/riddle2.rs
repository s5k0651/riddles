// Riddle: What is 3/7 chicken, 2/3 cat and 2/4 goat?
// Write a rust code that is generic for the inputs and that ALWAYS solves the riddle.

use std::io;

#[derive(Debug)]
pub struct RiddleInput {
    value: String,
    proportion: String,
}

fn solve(input: Vec<RiddleInput>) -> String {
    let mut result: Vec<&str> = vec![];

    for x in input.iter() {
      let mut proportion_parts: Vec<usize> = vec![];

      // transform the ["3","7"] into the [3,7]
      for part in x.proportion.split('/') {
        proportion_parts.push(part.parse().unwrap());
      }

      // if the second item is not the same as the length of the string we should not continue
      if proportion_parts[1] != x.value.len() {
        panic!("Cannot be computed, denominator doesn't match the value length");
      }

      // take the amount of the characters from the start of the value string
      let value_part = &x.value[..proportion_parts[0]];
      result.push(value_part);
    }

    println!("Answer is: {}", result.join(""));
    result.join("")
  }

pub fn solve_riddle_2() {
    println!("-----------------------------------------------------------");
    println!("--| Riddle: What is 3/7 chicken, 2/3 cat and 2/4 goat? |--");
    println!("-----------------------------------------------------------");
    // input should be a sentence containing all the words separacted by a space,
    // like "chicken cat goat"
    println!("Enter a sentence containing all the words separated by a space");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    // The input string is separated to individual words vector
    // like "chicken" "cat" "goat"
    let input: Vec<&str> = input.split_whitespace().collect();
    println!("Word Vector: {:?}", input);

    println!("Similarly, Enter a sentence containing all the proportions respectively");
    let mut proportion_parts = String::new();
    io::stdin().read_line(&mut proportion_parts).expect("Error reading input");

    // The proportion_parts is separated to individual fraction vector
    // like "3/7" "2/3" "2/4"
    let proportion_parts: Vec<&str> = proportion_parts.split_whitespace().collect();
    println!("Fraction Vector: {:?}", proportion_parts);

    if input.len() != proportion_parts.len() {
        panic!("Words, Proportions don't match");
    }
    else {
        let mut i = 0;
        let length = input.len();
        let mut data: Vec<RiddleInput> = Vec::new();
        while i < length {
            data.push(RiddleInput {
                value: String::from(input[i]),
                proportion: String::from(proportion_parts[i]),
              });
            i = i + 1;
        }
        solve(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_riddle2() {
        assert_eq!(solve(vec![
            RiddleInput {
              value: String::from("chicken"),
              proportion: String::from("3/7"),
            },
            RiddleInput {
              value: String::from("cat"),
              proportion: String::from("2/3"),
            },
            RiddleInput {
              value: String::from("goat"),
              proportion: String::from("2/4"),
            },
          ]), "chicago");
    }

    #[test]
    fn test2_riddle2() {
        assert_eq!(solve(vec![
            RiddleInput {
              value: String::from("height"),
              proportion: String::from("2/6"),
            },
            RiddleInput {
              value: String::from("lloyd"),
              proportion: String::from("3/5"),
            }
          ]), "hello");
    }
}