use regex::Regex;
use std::collections::HashMap;
use std::fs;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BYEAR_RE: Regex = Regex::new(r"^(byr):(\d{4})$").unwrap();
    static ref EYEAR_RE: Regex = Regex::new(r"^(eyr):(\d{4})$").unwrap();
    static ref IYEAR_RE: Regex = Regex::new(r"^(iyr):(\d{4})$").unwrap();
    static ref HTG_RE: Regex = Regex::new(r"^(hgt):(((\d{2})(in))|((\d{3})(cm)))$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^(hcl):\#([0-9a-f]{6})$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(ecl):(amb|blu|brn|grn|gry|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^(pid):([0-9]{9})$").unwrap();
}

fn main() {
    let filename = "/home/caleb/codestuff/advent2020/fourth_day/src/input.txt";
    let data = fs::read_to_string(filename).expect("error");
    let data_split_on_empty_lines: Vec<&str> = data.split("\n\n").collect();
    let mut valid_num = 0;

    for data_block in data_split_on_empty_lines {
        let data_split_whitespace: Vec<&str> = data_block.split_ascii_whitespace().collect();
        let mut passport_map: HashMap<String, String> = HashMap::new();
        for feild in data_split_whitespace {
            if BYEAR_RE.is_match(feild) {
                let cap = BYEAR_RE.captures(feild).unwrap();
                let num = cap[2].parse::<i64>().unwrap();
                if num >= 1920 && num <= 2002 {
                    passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
                } else {
                    continue;
                }
            } else if IYEAR_RE.is_match(feild) {
                let cap = IYEAR_RE.captures(feild).unwrap();
                let num = cap[2].parse::<i64>().unwrap();
                if num >= 2010 && num <= 2020 {
                    passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
                } else {
                    continue;
                }
            } else if EYEAR_RE.is_match(feild) {
                let cap = EYEAR_RE.captures(feild).unwrap();
                let num = cap[2].parse::<i64>().unwrap();
                if num >= 2020 && num <= 2030 {
                    passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
                } else {
                    continue;
                }
            } else if HTG_RE.is_match(feild) {
                let cap = HTG_RE.captures(feild).unwrap();
                if cap.get(5) != None && &cap[5] == "in" {
                    let num = cap[4].parse::<i64>().unwrap();
                    if num >= 59 && num <= 76 {
                        passport_map.insert(String::from(&cap[1]), String::from(&cap[3]));
                    }else {
                        continue;
                    }
                }else if cap.get(8) != None && &cap[8] == "cm" {
                    let num = cap[7].parse::<i64>().unwrap();
                    if num >= 150 && num <= 193 {
                        passport_map.insert(String::from(&cap[1]), String::from(&cap[6]));
                    }else {
                        continue;
                    }
                }
            
            } else if HCL_RE.is_match(feild) {
                let cap = HCL_RE.captures(feild).unwrap();
                passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
            } else if ECL_RE.is_match(feild) {
                let cap = ECL_RE.captures(feild).unwrap();
                passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
            } else if PID_RE.is_match(feild) {
                let cap = PID_RE.captures(feild).unwrap();
                passport_map.insert(String::from(&cap[1]), String::from(&cap[2]));
            }
        }
        
        println!("{:?}", passport_map);
        if passport_map.contains_key("byr") {
            if passport_map.contains_key("iyr")
                && passport_map.contains_key("eyr")
                && passport_map.contains_key("hgt")
                && passport_map.contains_key("hcl")
                && passport_map.contains_key("ecl")
                && passport_map.contains_key("pid")
            {
                valid_num += 1;
            }
        }
    }

    println!("{}", valid_num);
}
