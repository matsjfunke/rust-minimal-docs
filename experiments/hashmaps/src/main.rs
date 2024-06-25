use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team_name: {team_name}, score: {score}");
    
    // iterating
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
