use std::fs;

fn main(){

    // Create mutable (modifiable?) variables of datatype int64 
    let mut tmpSum : i64;
    let mut maxVal: i64 = 0;
    let mut tmpInt: i64;

    /*  
        Read the information from the text file and split it into substrings
        for each distinct set of values
    */
    let data: String = fs::read_to_string("./actual.txt").expect("Unable to read file");
    let initial_split = data.split("\n\r"); // \n for new line and \r for line break

    // Create a dynamic array (a vector) to store the calculated substring sums
    let mut calorie_array : Vec<i64> = Vec::new();
    let mut max_array : [i64; 3] = [1,2,3];
     
    for string_collection in initial_split {
        tmpSum = 0; // Set the tmpSum variable to 0 for each substring
        // Now each section is split into substrings from which integers can be extracted
        let internal_split = string_collection.split("\n"); 

        for value in internal_split {
            // Skip empty strings, as "\n\r" strings are still inlcluded
            // TODO: Look into if the split operation has exclusion clause.
            if value.is_empty() {
                continue;
            }
            /*  
                Each integer substring is trimed of ending "\n", is then parsed into i64 datatype
                the result is then unwrapped, granting access to the i64 representation
            */
            tmpInt = value.trim().parse::<i64>().unwrap();
            // Rust does not have increment (+=) or decrement (-=) operators for clarity reasons
            tmpSum = tmpInt + tmpSum;
        }
        // Update the maxValue variable if new max is encountered
        if tmpSum > maxVal {
            maxVal = tmpSum;
        }
        
        update_max_value_array(3, &mut max_array, tmpSum);

        // Add the value to the vector
        calorie_array.push(tmpSum);

    }
    for max in max_array {
        println!("{}", max);
    }
    println!("{}", max_array.iter().sum::<i64>());
    println!("The biggest amount of calories from a single elf is {}", maxVal); 
    
}

fn update_max_value_array(array_len : usize, array: &mut [i64], check_value : i64) {
    let mut tmpVal;
    let mut curr_index = 0;
    let mut previous_val = 0;
    for x in (0..array_len).rev() {
        if check_value > array[x] {
            previous_val = array[x];
            curr_index = x;
            array[x] = check_value;
            break;
        }
    }
 
    while curr_index > 0 {
        if curr_index == 1 {
            array[curr_index-1] = previous_val;
            break
        }
        tmpVal = array[curr_index - 1];
        array[curr_index - 1] = previous_val;
        previous_val = tmpVal;
        curr_index = curr_index - 1;
    }
}

