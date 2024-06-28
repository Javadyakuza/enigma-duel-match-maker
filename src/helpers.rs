use rand::seq::{index, SliceRandom};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    body: String,
    opt1: String,
    opt2: String,
    opt3: String,
    opt4: String,
    correct: String,
}

pub fn get_rand_questions() -> String {
    // Sample JSON data
    let data = r#"
    {
        "question_1": {
          "body": "What year did the Cold War end?",
          "opt1": "1978",
          "opt2": "1923",
          "opt3": "1989",
          "opt4": "1929",
          "correct": "1989"
        },
        "question_2": {
          "body": "Who developed the theory of relativity?",
          "opt1": "Isaac Newton",
          "opt2": "Albert Einstein",
          "opt3": "Galileo Galilei",
          "opt4": "Niels Bohr",
          "correct": "Albert Einstein"
        },
        "question_3": {
          "body": "What is the capital of Australia?",
          "opt1": "Sydney",
          "opt2": "Melbourne",
          "opt3": "Canberra",
          "opt4": "Brisbane",
          "correct": "Canberra"
        },
        "question_4": {
          "body": "Which planet is known as the Red Planet?",
          "opt1": "Earth",
          "opt2": "Mars",
          "opt3": "Jupiter",
          "opt4": "Saturn",
          "correct": "Mars"
        },
        "question_5": {
          "body": "What is the largest ocean on Earth?",
          "opt1": "Atlantic Ocean",
          "opt2": "Indian Ocean",
          "opt3": "Arctic Ocean",
          "opt4": "Pacific Ocean",
          "correct": "Pacific Ocean"
        },
        "question_6": {
          "body": "Who wrote 'To Kill a Mockingbird'?",
          "opt1": "Harper Lee",
          "opt2": "J.K. Rowling",
          "opt3": "Jane Austen",
          "opt4": "Ernest Hemingway",
          "correct": "Harper Lee"
        },
        "question_7": {
          "body": "What is the smallest prime number?",
          "opt1": "0",
          "opt2": "1",
          "opt3": "2",
          "opt4": "3",
          "correct": "2"
        },
        "question_8": {
          "body": "Which element has the chemical symbol 'O'?",
          "opt1": "Osmium",
          "opt2": "Oxygen",
          "opt3": "Gold",
          "opt4": "Iron",
          "correct": "Oxygen"
        },
        "question_9": {
          "body": "In which year did the Titanic sink?",
          "opt1": "1905",
          "opt2": "1912",
          "opt3": "1923",
          "opt4": "1931",
          "correct": "1912"
        },
        "question_10": {
          "body": "Who painted the Mona Lisa?",
          "opt1": "Vincent van Gogh",
          "opt2": "Pablo Picasso",
          "opt3": "Leonardo da Vinci",
          "opt4": "Claude Monet",
          "correct": "Leonardo da Vinci"
        }
    }"#;

    // Parse the JSON data
    let questions_map: HashMap<String, Question> = serde_json::from_str(data).unwrap();

    // Collect questions into a vector
    let mut rng = rand::thread_rng();

    // Generate three random numbers between 1 and 10 (inclusive)
    let random_numbers: Vec<u32> = generate_unique_random_numbers(3, 1..=10);

    let indexes: Vec<String> = random_numbers
        .into_iter()
        .map(|num| format!("question_{}", num))
        .collect();
    // Select three random questions
    let selected_questions: Vec<&Question> = vec![
        questions_map.get(&indexes[0]).unwrap(),
        questions_map.get(&indexes[1]).unwrap(),
        questions_map.get(&indexes[2]).unwrap(),
    ];

    serde_json::to_string(&selected_questions).unwrap()
}

fn generate_unique_random_numbers(count: usize, range: std::ops::RangeInclusive<u32>) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut unique_numbers = HashSet::new();

    while unique_numbers.len() < count {
        let num = rng.gen_range(range.clone());
        unique_numbers.insert(num);
    }

    unique_numbers.into_iter().collect()
}
use cosmwasm_std::to_json_binary;

pub fn create_key_hash(con_1: &str, con_2: &str) -> String {
    to_json_binary(&format!("{}{}", con_1, con_2))
        .unwrap()
        .to_string()
}

use std::process::Command;

pub fn create_game_room(
    contestant1: String,
    contestant2: String,
    prize_pool: i32,
) -> Result<(), String> {
    // Define the path to your Node.js script
    let node_script_path =
        "/home/javadyakuza/Desktop/enigma_duel_match_maker/js_scripts/create_game_room.js"; // Update this with the actual path

    // Execute the Node.js script with arguments
    let output = Command::new("node")
        .arg(node_script_path)
        .arg(contestant1)
        .arg(contestant2)
        .arg(prize_pool.to_string())
        .output()
        .expect("Failed to execute Node.js script");

    // Print the output from the script
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stdout.contains("Error") || stderr.contains("Error") {
        Err(format!("{}{}", stdout, stderr))
    } else {
        Ok(())
    }
}

pub fn finish_game_room(game_room_key: String, winner: String) -> Result<(), String> {
    // Define the path to your Node.js script
    let node_script_path =
        "/home/javadyakuza/Desktop/enigma_duel_match_maker/js_scripts/finish_game_room.js"; // Update this with the actual path

    // Execute the Node.js script with arguments
    let output = Command::new("node")
        .arg(node_script_path)
        .arg(game_room_key)
        .arg(winner)
        .output()
        .expect("Failed to execute Node.js script");

    // Print the output from the script
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stdout.contains("Error") || stderr.contains("Error") {
        Err(format!("{}{}", stdout, stderr))
    } else {
        Ok(())
    }
}
fn count_trues(s: &str) -> usize {
    s.chars().filter(|&c| c == '1').count()
}
pub fn determine_winner(user1: &str, user2: &str) -> String {
    // Count the number of '1's in each string
    let count1 = count_trues(user1);
    let count2 = count_trues(user2);

    // Determine the winner
    if count1 > count2 {
        return user1.to_string();
    } else if count2 > count1 {
        return user2.to_string();
    } else {
        return "\"\"".to_string(); // It's a tie
    }
}
