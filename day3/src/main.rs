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

fn assemble_numbers(pattern:regex::Regex,splitted:String) {
    let mut numbers:Vec<Vec<i32>> = Vec::new();
    let mut mul_sum:i32 = 0;
        let mut n = find_pattern_in_line(pattern.clone(),splitted);
        numbers.append(&mut n);
    for number_pair in numbers {
        mul_sum += number_pair[0]*number_pair[1];
        println!("{},{}",number_pair[0],number_pair[1])
    }
    println!("Sum of muls: {}",mul_sum);
}

fn main() {
    let location:String = "input".to_string();
    let contents = common::read(&location);
    let mull_pattern = Regex::new(r"mul\((\d|\d\d|\d\d\d),(\d|\d\d|\d\d\d)\)").unwrap();

    let dont_splitted = contents
                            .split("don't")
                            .collect::<Vec<_>>();
    let mut do_dont_split:Vec<Vec<&str>> = Vec::new();
    for line in dont_splitted {
        let do_splitted = line
                            .split("do")
                            .collect::<Vec<_>>();
        do_dont_split.push(do_splitted);

    }

    for dont in do_dont_split {
        for doit in dont{
            println!("{}",doit)
        }
    }
    //assemble_numbers(pattern,contents);
}
