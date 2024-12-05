use crate::common::Report;

fn check_magnitude(delta:i32) -> (bool,String){
    if 0<delta.abs() && delta.abs() < 4 {
        return (true, "None      ".to_string());
    } else if 0<delta.abs(){
        return (false,"Delta high".to_string());
    } else if delta.abs()==0 {
        return (false, "Delta is 0".to_string());
    } else {
        println!("SOMETHING IS WRONG");
        return (false,"No Idea".to_string());
    }

}

fn check_signs(delta1:i32,delta2:i32) -> (bool,String) {
    if delta1.is_positive() && delta2.is_positive() {
        return (true,"None      ".to_string());
    } else if delta1.is_negative() && delta2.is_negative() {
        return (true,"None     ".to_string());
    } else {
        return (false,"Signs wrong".to_string());
    }
}

pub fn saftey(rpt:&mut Report) -> &mut Report{
    let mut safe = true;
    let mut i: usize = 0;
    let mut delta:Vec<i32> = Vec::with_capacity(rpt.data.len());        
    loop {
        if i==rpt.data.len()-1 {break;}
        let diff = rpt.data[i] - rpt.data[i+1];
        delta.push(diff);
        i += 1;
    }
    println!("");
    
    i = 0;
    loop{
        if i == delta.len() {break;}
        let magn_check; let sign_check;
        let magn_reasn; let sign_reasn;
        (magn_check,magn_reasn) = check_magnitude(delta[i]);
        if i< delta.len()-1 {
            (sign_check,sign_reasn) = check_signs (delta[i],delta[i+1]);
            if !sign_check {
        if !sign_check {
            safe = false; 
            rpt.reason = sign_reasn;
            break;
        }
                safe = false; 
                rpt.reason = sign_reasn;
                break;
            }
        }
        if !magn_check {
            safe = false;
            rpt.reason = magn_reasn;
            break;
        }
        i += 1;
    }
    rpt.safe = safe;
    return rpt;
}

