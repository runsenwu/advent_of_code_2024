use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. get a list of inputs
    let contents = fs::read_to_string("input.txt")?;
    let mut sum = 0;

    // We can use the iterator function here to iterate through this effectively
    for line in contents.lines() {
        let list: Vec<&str> = line.split(" ").collect();

        if list.len() == 1 {
            sum += 1;
            continue;
        }

        let mut curr: isize = list[0].parse::<isize>()?;
        let mut next: isize = list[1].parse::<isize>()?;

        let mut diff: isize = next - curr;
        let mut increase = false;

        if diff.abs() > 3 || diff == 0 {
            continue;
        } else {
            if diff > 0 {
                increase = true;
            } else if diff < 0 {
                increase = false;
            }
        }

        let mut increment = true;

        for i in 2..list.len() {
            curr = next;
            next = list[i].parse::<isize>()?;

            diff = next - curr;

            if diff.abs() > 3 || diff == 0 {
                increment = false;
                break;
            } else {
                if diff > 0 && !increase {
                    increment = false;
                    break;
                } else if diff < 0 && increase {
                    increment = false;
                    break;
                }
            }
        }

        if increment {
            sum += 1;
        }
    }

    println!("{sum}");

    Ok(())
}
