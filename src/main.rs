mod api;
mod keyboard;

use api::fetch_word_from_api;
use keyboard::map_key_to_char;
use raylib::{prelude::*, ffi::IsFileDropped};

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init().size(200, 400).title("Wordle Clone").build();

    rl.set_target_fps(60);
    let mut target_word;
    target_word = match fetch_word_from_api().await {
        Ok(word) => word.to_uppercase(),
        Err(err) => {
            eprintln!("Failed to fetch the word from the API: {:?}", err);
            return;
        }
    };
    let mut target_chars: Vec<char> = target_word.chars().collect();
    let mut guessed_letters = Vec::new();
    let mut attempts = Vec::new();
    let max_attempts = 5;
    let mut attempts_left = max_attempts;
    let mut game_running = false;

    while !rl.window_should_close() {
        if attempts_left > 0 && game_running {
            if let Some(key) = rl.get_key_pressed() {
                if let Some(c) = map_key_to_char(key) {
                    if c == '/' {
                        if guessed_letters.len() != 0 {
                            guessed_letters.pop();
                        }
                    } else {
                        guessed_letters.push(c);

                        if guessed_letters.len() == target_word.len() {
                            if guessed_letters.iter().collect::<String>() == target_word {
                                println!(
                                    "You've used all attempts. The target word was: {}",
                                    target_word
                                );
                                break;
                            }
                            let feedback = generate_feedback(&guessed_letters, &target_chars);

                            println!(
                                "Attempt {}/{}: {}",
                                max_attempts - attempts_left + 1,
                                max_attempts,
                                guessed_letters.iter().collect::<String>(),
                            );

                            attempts.push((guessed_letters.clone(), feedback));
                            guessed_letters.clear();
                            attempts_left -= 1;
                        }
                    }
                }
            }
        } else {
            game_running = false;
            if let Some(key) = rl.get_key_pressed() {
                if key == KeyboardKey::KEY_ENTER {
                    attempts_left = max_attempts;
                    attempts.clear();
                    // target_word = "hello".to_uppercase();
                    target_word = match fetch_word_from_api().await {
                        Ok(word) => word.to_uppercase(),
                        Err(err) => {
                            eprintln!("Failed to fetch the word from the API: {:?}", err);
                            return;
                        }
                    };
                    println!("word is {}",target_word);
                    target_chars = target_word.chars().collect();
                    game_running = true;
                }
            }
            // println!(
            //     "You've used all attempts. The target word was: {}",
            //     target_word
            // );
            // break;
        }

        let mut y = 50;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        if game_running {
            d.draw_text("Enter your guess:", 10, 10, 20, Color::WHITE);
            let mut x = 10;
            for (_i, &ch) in guessed_letters.iter().enumerate() {
                d.draw_text(&ch.to_string(), x, y, 20, Color::WHITE);
                x += 15;
            }
            y += 30;
            for (attempt, feedback) in &attempts {
                let mut x = 10;
                for (i, &ch) in attempt.iter().enumerate() {
                    // println!("{feedback:s?}");
                    d.draw_text(&ch.to_string(), x, y, 20, feedback[i]);
                    x += 15;
                }
                y += 30;
            }

            d.draw_text(
                &format!("Attempts left: {}", attempts_left),
                10,
                y,
                20,
                Color::WHITE,
            );
        } else {
            d.draw_text(&format!("Attempts Over"), 10, y, 20, Color::WHITE);
            y += 30;
            d.draw_text(&format!("Word is {}", target_word), 10, y, 20, Color::WHITE);
            y += 30;
            d.draw_text(&format!("Click Enter"), 10, y, 20, Color::WHITE);
            y += 30;
            d.draw_text(&format!("to play again"), 10, y, 20, Color::WHITE);
        }
    }

    drop(thread); // Close the Raylib thread cleanly
}

fn generate_feedback(guess: &[char], target: &[char]) -> Vec<Color> {
    let mut feedback = Vec::new();
    
    for (i, &g) in guess.iter().enumerate() {
        if target[i] == g {
            feedback.push(Color::GREEN); // Correct letter in correct position (green)
        } else if target.contains(&g) {
            feedback.push(Color::YELLOW); // Correct letter in wrong position (yellow)
        } else {
            feedback.push(Color::GRAY); // Letter not found in the target
        }
    }
    
    feedback
}

