use std::fs;

fn main() {
    let data_as_string: String = fs::read_to_string("./actual.txt").expect("Could not read file");
    let split_data = data_as_string.split("\n");
    let mut total_score: u32 = 0;
    for data in split_data {
        let trim_data = data.trim();
        let data_len = trim_data.len();
        //println!("{}", &trim_data[..data_len/2]);
        //println!("{}", &trim_data[data_len/2..]);
        
        let mut encountered_numerics : Vec<u32> = Vec::new();
        let first_half_numeric = string_to_prio(&trim_data[..data_len/2]);
        let second_half_numeric = string_to_prio(&trim_data[data_len/2..]);

        for number in first_half_numeric {
            if encountered_numerics.contains(&number) {
                continue;
            }
            else if second_half_numeric.contains(&number){
                encountered_numerics.push(number);
                total_score = total_score + number;
            }
        }

    }
    println!("Total score is {}", total_score);

}

fn string_to_prio(input_str : &str) -> Vec<u32> {
    /*
        This function takes a &str string as input and converts each of the
        chars represented in it a numeric form represented by the following scheme:
        a-z are converted into numeric u32 1-26
        A-Z are converted into numeric u32 27-52
    */

    let char_vec : Vec<char> = input_str.chars().collect();
    let mut converted_vec : Vec<u32> = Vec::new();

    for char_ele in char_vec {
        let dec_conversion = char_ele as u32;
        if dec_conversion < 91 {
            converted_vec.push(dec_conversion -  64 + 26);
        }
        else {
            converted_vec.push(dec_conversion - 96);
        }
    }

    return converted_vec;


}