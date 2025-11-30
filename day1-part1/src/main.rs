use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. get a list of inputs
    let contents = fs::read_to_string("input.txt")?;

    // 2. put them into two lists
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    // We can use the iterator function here to iterate through this effectively
    for line in contents.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        list1.push(left.parse::<i32>()?);
        list2.push(right.parse::<i32>()?);
    }

    // 3. sort the lists
    list1.sort();
    list2.sort();

    let mut sum = 0;

    // 4. find the differences between those two lists
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    // 5. sum up the differences
    println!("{sum}");

    Ok(())
}
