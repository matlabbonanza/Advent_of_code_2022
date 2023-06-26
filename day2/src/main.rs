use std::fs;

fn main() {
    
    let data_as_string: String = fs::read_to_string("./actual.txt").expect("Could not read file");
    let mut first_task_score : i32 = 0;
    let mut second_task_score : i32 = 0;
    
    let data_split = data_as_string.split("\n");
    for move_set in data_split {
        let trimmed = move_set.trim();
        let vec: Vec<&str> = trimmed.split_ascii_whitespace().collect();
        let second_vec : Vec<&str> = vec![vec[0].clone(), map_needed_play(vec.clone())];

        first_task_score = first_task_score + played_object_score(vec[1]);
        first_task_score = first_task_score + resolve_played_result(vec);

        second_task_score = second_task_score + played_object_score(second_vec[1]);
        second_task_score = second_task_score + resolve_played_result(second_vec);
    }
    println!("First task score: {}", first_task_score);
    println!("Second task score: {}", second_task_score);
}

fn played_object_score(input : &str) -> i32 {
    match input {
        "X" => return 1,
        "Y" => return 2,
        "Z" => return 3,
        _ => println!("Input does not match expected values")
    }

    return -1;
}

fn map_needed_play(input_vec : Vec<&str>) -> &str {
    let mut play_string : &str = "";
    // X - means lose here
    if input_vec[1] == "X" {
        if input_vec[0] == "A" {
            play_string = "Z";
        }
        else if input_vec[0] == "B" {
            play_string = "X";
        }
        else {
            play_string = "Y";
        }
    }
    // Y - means draw
    if input_vec[1] == "Y" {
        if input_vec[0] == "A" {
            play_string = "X";
        }
        else if input_vec[0] == "B" {
            play_string = "Y";
        }
        else {
            play_string = "Z";
        }
    }
    // Z - means win
    if input_vec[1] == "Z" {
        if input_vec[0] == "A" {
            play_string = "Y";
        }
        else if input_vec[0] == "B" {
            play_string = "Z";
        }
        else {
            play_string = "X";
        }
    }

    return play_string;
}

fn resolve_played_result(input_vec : Vec<&str>) -> i32 {
    let mut round_score : i32 = 0;

    if input_vec[0] == "A" {
        if input_vec[1] == "X" {
            round_score = 3;
        }
        else if input_vec[1] == "Y" {
            round_score = 6;
        }
    }

    if input_vec[0] == "B" {
        if input_vec[1] == "Y" {
            round_score = 3;
        }
        else if input_vec[1] == "Z" {
            round_score = 6;
        }
    }

    if input_vec[0] == "C" {
        if input_vec[1] == "Z" {
            round_score = 3;
        }
        else if input_vec[1] == "X" {
            round_score = 6;
        }
    }

    return round_score;

}