mod common;
mod part1;
mod part2; 
fn p1 (location:&String)  -> i32{
    let (sorted_left,sorted_right) = common::sorted_left_right(&location);
    return part1::part1(sorted_left,sorted_right);
}

fn p2(location: &String) -> i32 {
    let (sorted_left, sorted_right) = common ::sorted_left_right(&location);
    return part2::part2(&sorted_left,&sorted_right);
}

fn main() {
    let test = "test".to_string();
    let input = "input".to_string();
    let test_p1 = p1(&test);
    let input_p1 = p1(&input);
    println!("Test result part 1: {}", test_p1);
    println!("Input result part 1: {}", input_p1);

    println!("Test resutl part 2: {}",p2(&test));
    println!("Input result part 2: {}",p2(&input));
}
