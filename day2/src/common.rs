use std::fs;
use std::fmt;

pub struct Report {
    pub safe: bool,
    pub size: usize,
    pub data: Vec<i32>,
    pub reason: String,
}

impl Report {
    pub fn new (data:Vec<i32>) ->Report {
        Report {
            safe: false,
            size: data.len(),
            data,
            reason: "None".to_string(),
        }
        
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"Report: ")?;
        write!(f," Safe: {}",self.safe)?;
        write!(f," Reason: {} \t",self.reason)?;
        let mut i:usize = 0;
        while i<self.data.len(){
            write!(f,"{} ",self.data[i])?;
            i += 1;
        }
        Ok(())
    }
}

pub fn construct_reports(location:&String) -> Vec<Report> {
    let contents = read(&location);
    let splitted = contents
                    .split("\n")
                    .collect::<Vec<_>>();
    let mut reports : Vec<Report> = Vec::with_capacity(splitted.len());
    let mut i: usize = 0;
    loop {
        if i == splitted.len()-1 {break;}
        let data:Vec<i32> = splitted[i as usize]
                                .split(" ")
                                .map(|x| x.parse::<i32>().unwrap() )
                                .collect::<Vec<_>>();

        reports.push(Report::new(data));
        i += 1;
    }
    return reports;
}

pub fn read(file_path:&String) -> String {
    let contents:String = fs::read_to_string(file_path)
                            .expect("Should have been able to read");
    return contents;
}

