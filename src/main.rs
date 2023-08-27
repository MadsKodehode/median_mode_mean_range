#![warn(clippy::unwrap_used)]
use std::{collections::HashMap, io, num::ParseIntError};
fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //Parse each comma separated number to i32 and return a Result containing a vec of those i32 nums
        let result: Result<Vec<i32>, ParseIntError> =
            input.trim().split(',').map(str::parse).collect();

        //If result was successful
        let mut nums: Vec<i32> = if let Ok(vec) = result {
            vec
        } else {
            println!("Invalid input, please type numbers seperated by commas");
            continue;
        };

        let median = median(&mut nums);

        let mode = mode(&nums);
        let mean = mean(&nums);

        println!("Median: {median:?}, Mode: {mode:?}, Mean: {mean:?}");
    }
}

fn mean(nums: &[i32]) -> f64 {
    let sum = f64::from(nums.iter().sum::<i32>()); //Sum of adding all nums together
    let len = nums.len() as f64; //Cast as f64 to get floating point

    sum / len //We need floating point
}

fn mode(nums: &[i32]) -> &i32 {
    //Create new empty HashMap for storing occurences of each n in nums
    let mut occurence_map: HashMap<&i32, u8> = HashMap::new();

    //Iterate each n in nums vector and call max_by_key with a function to get the n that occurs the most
    nums.iter()
        .max_by_key(|&n| {
            let count = occurence_map.entry(n).or_insert(0); //Use n from nums as keys in occurence_map

            *count += 1; //Increment value of current key
            *count //Return the one with highest count
        })
        .expect("Unable to find mode of nums")
}

fn median(nums: &mut [i32]) -> f64 {
    nums.sort_unstable();

    let nums_len = nums.len();

    //If vector is even
    if nums_len % 2 == 0 {
        let middle_left = nums
            .get((nums_len / 2) - 1)
            .expect("Could not get left middle value of nums");

        let middle_right = nums
            .get(nums_len / 2)
            .expect("Could not get right middle value of nums");

        f64::from(middle_left + middle_right) / 2.
    } else {
        let middle = nums
            .get(nums_len / 2)
            .expect("Could not get median of nums");

        f64::from(*middle)
    }
}
