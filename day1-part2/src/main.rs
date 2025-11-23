use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. get a list of inputs
    let contents = fs::read_to_string("input.txt")?;

    // 2. put them into two lists
    let mut list1 = Vec::<usize>::new();
    let mut list2 = Vec::<usize>::new();

    // We can use the iterator function here to iterate through this effectively
    for line in contents.lines() {
        // This here can techinically be done using a sorted tree
        let (left, right) = line.split_once("   ").unwrap();
        list1.push(left.parse::<usize>()?);
        list2.push(right.parse::<usize>()?);
    }

    // some d&a here,
    list1.sort();
    list2.sort();

    let mut sum = 0;

    // 3. find number of times each left occour in right
    for index in 0..list1.len() {
        let lower = list2.partition_point(|x| x < &list1[index]);
        let upper = list2.partition_point(|x| x <= &list1[index]);

        if lower != upper {
            sum += list1[index] * (upper - lower);
        }
    }
    
    println!("{sum}");

    Ok(())
}
