mod common;
use regex::Regex;

fn parse(d:String) ->i32 {
    d.parse().unwrap()
}


fn find_pattern_in_line(pattern:regex::Regex,line:String) -> Vec<Vec<i32>>{
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for matched in pattern.find_iter(&line) {
        if let Some(capture) = pattern.captures(matched.as_str()) {
            let matched_nums = Vec::from([parse(capture[1].to_string()),
                                          parse(capture[2].to_string())]);
            numbers.push(matched_nums);
        }
    }
    return numbers
}

fn find_pattern_in_line2(pattern:regex::Regex,line:String) -> Vec<Vec<i32>> {
    let mut is_on:bool = true;
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for matched in pattern.find_iter(&line) {
        if let Some(capture) = pattern.captures(matched.as_str()) {
            if capture[0] == "do()".to_string() {
                is_on = true;
            } else if capture[0] == "don't()".to_string() {
                is_on = false;
            } else {
                if is_on {
                    let matched_nums = Vec::from([parse(capture[1].to_string()),
                                                  parse(capture[2].to_string())]);
                    numbers.push(matched_nums);
                }
            }
        }
    }
    return numbers
}


fn assemble_numbers(pattern:regex::Regex,splitted:String) -> i32 {
    let mut numbers:Vec<Vec<i32>> = Vec::new();
    let mut mul_sum:i32 = 0;
    let mut n = find_pattern_in_line(pattern.clone(),splitted);
    numbers.append(&mut n);
    for number_pair in numbers {
        mul_sum += number_pair[0]*number_pair[1];
    }
    mul_sum
}

fn assemble_numbers2(pattern:regex::Regex,splitted:String) -> i32{
    let mut numbers:Vec<Vec<i32>> = Vec::new();
    let mut mul_sum:i32 = 0;
    let mut n = find_pattern_in_line2(pattern.clone(),splitted);
    numbers.append(&mut n);
    for number_pair in numbers {
        mul_sum += number_pair[0]*number_pair[1];
    }
    mul_sum
}

fn part1(location:&String) -> i32{
    let contents = common::read(location);
    let mull_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    assemble_numbers(mull_pattern,contents)
}

fn part2(location:&String) -> i32{
    let contents = common::read(location);
    let mull_pattern = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    assemble_numbers2(mull_pattern,contents)
}
fn main() {
    let input:String = "input".to_string();
    let test:String = "test".to_string();
    let test2:String = "test2".to_string();
    println!("Test: {}, Test1: {}, Input: {}",part1(&test),part1(&test2),part1(&input));
    println!("Test: {}, Test1: {}, Input: {}",part2(&test),part2(&test2),part2(&input));

}
