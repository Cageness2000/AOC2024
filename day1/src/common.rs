use std::fs;
use std::str::FromStr;


fn read(file_path:&String) -> String {
    let contents:String = fs::read_to_string(file_path)
                            .expect("Should have been able to read");
    return contents;
}

fn parse_left_right(location:&String) -> (Vec<i32>,Vec<i32>){
    let contents = read(&location);
    let splitted = contents
                    .split("\n")
                    .collect::<Vec<_>>();
    let splitted_length:usize = splitted.len()-1;
    let mut left_side   :Vec<i32> = Vec::with_capacity(splitted_length);    
    let mut right_side  :Vec<i32> = Vec::with_capacity(splitted_length);    

    let mut i:usize = 0;
    loop {
        if i == splitted_length {break;}
        let parsed_line = splitted[i as usize]
                            .split("   ")
                            .collect::<Vec<_>>();
        left_side .push( FromStr::from_str(parsed_line[0]).unwrap());
        right_side.push( FromStr::from_str(parsed_line[1]).unwrap());
        i += 1;
    }
    return (left_side,right_side); 
}

fn merge(v1:&Vec<i32>, v2:&Vec<i32>) -> Vec<i32> {
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut merged: Vec<i32> = Vec::with_capacity(v1.len()+v2.len());

    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            merged.push(v1[i]);
            i += 1;
        } else {
            merged.push(v2[j]);
            j += 1;
        }
    }
    
    if i < v1.len() {
        while i < v1.len() {
            merged.push(v1[i]);
            i += 1;
        }
    }
    if j < v2.len() {
        while j < v2.len() {
            merged.push(v2[j]);
            j += 1;
        }
    }
    return merged;
}

fn merge_sort(to_sort:&Vec<i32>) -> Vec<i32> {
    if to_sort.len() < 2{
        return to_sort.to_vec();
    } else {
        let size = to_sort.len() /2;
        let left = merge_sort(&to_sort[0..size].to_vec());
        let right = merge_sort(&to_sort[size..].to_vec());
        let merged = merge(&left, &right);
        return merged;
    }
}


pub fn sorted_left_right(location:&String) -> (Vec<i32>,Vec<i32>) { 
    let (left,right) = parse_left_right(&location);
    let sorted_left : Vec<i32>  = merge_sort(&left);
    let sorted_right: Vec<i32> = merge_sort(&right);
    return (sorted_left,sorted_right)
}
