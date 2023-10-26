use std::fs;

const BLANKLINE: &'static str = "\n\n";

fn main() {
    //read file
    let input = fs::read_to_string("day1.input").expect("Shoulda been able to read the file");
    //split by blankline to elfs
    let mut cals: Vec<i32> = input
        .split(BLANKLINE)
        .map(|parag| {
            //split the elfs to cals by newline
            parag
                .split("\n")
                // parse numbers
                .map(|cal| cal.parse::<i32>().unwrap_or(0))
                //sum the numbers in the lines
                .sum()
        })
        .collect();

    cals.sort();
    cals.reverse();

    //get the max and return it
    let max = cals.iter().max().unwrap();
    println!("max is {max}");

    // PART2
    let top3 = &cals[..3];
    let answer: i32 = top3.iter().sum();
    println!("sum of top3 is {answer}");
}
