mod common;
use crate::common::Report;
mod part1;
mod part2;


fn p2(mut reports:Vec<Report>) -> u32{
    let mut i:usize = 0;
    let mut safe:u32 = 0;
    loop{
        if i == reports.len() {break;}
        part2::saftey(&mut reports[i],true);
        if reports[i].safe { safe += 1;}
        println!("Line: {} {}",i, reports[i]);
        i += 1;
    }
    println!("Safe: {}",safe);
    return safe;
}

fn p1(mut reports:Vec<Report>) -> u32{
    let mut i:usize = 0;
    let mut safe:u32 = 0;
    loop{
        if i == reports.len() {break;}
        part1::saftey(&mut reports[i]);
        if reports[i].safe { safe += 1;}
        println!("Line: {} {}",i, reports[i]);
        i += 1;
    }
    println!("Safe: {}",safe);
    return safe;
}

fn main() {
    let test:String = "test".to_string();
    let test_reports = common::construct_reports(&test);
    let input:String = "input".to_string();
    let input_reports = common::construct_reports(&input);
    let p1_test = p1(test_reports);
    let p1_input = p1(input_reports);

    let test_reports = common::construct_reports(&test);
    let input_reports = common::construct_reports(&input);
    let p2_test = p2(test_reports);
    let p2_input = p2(input_reports);
}

