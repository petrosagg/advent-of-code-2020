use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn validate(key: &str, value: &str) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.
    match key {
        "byr" => {
            if let Ok(n) = value.parse::<u64>() {
                return value.len() == 4 && 1920 <= n && n <= 2002
            }
        },
        "iyr" => {
            if let Ok(n) = value.parse::<u64>() {
                return value.len() == 4 && 2010 <= n && n <= 2020
            }
        },
        "eyr" => {
            if let Ok(n) = value.parse::<u64>() {
                return value.len() == 4 && 2020 <= n && n <= 2030
            }
        },
        "hgt" => {
            let (scalar, unit) = value.split_at(value.len() - 2);
            let scalar = scalar.parse::<u64>();
            match (scalar, unit) {
                (Ok(n), "cm") => return 150 <= n && n <= 193,
                (Ok(n), "in") => return 59 <= n && n <= 76,
                _ => ()
            }
        } 
        "ecl" => {
            let valid = &[ "amb", "blu",  "brn", "gry", "grn", "hzl", "oth" ];
            for v in valid {
                if *v == value {
                    return true
                }
            }

        } 
        "hcl" => {
            if value.chars().next().unwrap() == '#' {
                return value.chars().skip(1).all(|c| c.is_digit(16));
            }
        } 
        "pid" => {
            return value.len() == 9 && value.chars().all(|c| c.is_digit(10));
        } 
        _ => panic!(),
    }
    return false;
}

pub fn first() {
    let path = format!("data/day{}/{}", 4, 1);
    let input = fs::read_to_string(&path).unwrap();

    let passports = input
        .lines()
        .filter_map({
            let mut fields = HashSet::new();
            move |l| {
                if l.is_empty() {
                    let result = fields.clone();
                    fields.clear();
                    return Some(result);
                }
                let new_fields = l.split(' ').map(|f| f.split_at(3).0.to_string());
                fields.extend(new_fields);
                None
            }
        });
        //.collect::<Vec<_>>();
    
    let required = &[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];
    let required: HashSet<_> = required.iter().map(|s| s.to_string()).collect();

    let mut valid = 0;
    for passport in passports {
        if passport.is_superset(&required) {
            valid += 1;
        }
    }

    println!("{:?}", valid);
}

pub fn second() {
    let path = format!("data/day{}/{}", 4, "1");
    let input = fs::read_to_string(&path).unwrap();

    let passports = input
        .lines()
        .filter_map({
            let mut fields = HashMap::new();
            move |l| {
                if l.is_empty() {
                    let result = fields.clone();
                    fields.clear();
                    return Some(result);
                }
                let new_fields = l.split(' ').map(|f| f.split_at(3)).map(|(a, b)| (a.to_string(), b[1..].to_string()));
                fields.extend(new_fields);
                None
            }
        });

    let required = &[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];
    let required = required.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let mut total_valid = 0;
    for passport in passports {
        let mut valid = true;
        for key in &required {
            if !passport.contains_key(key) {
                valid = false;
                break;
            }
            let value = passport.get(key).unwrap();
            if !validate(&key, value) {
                valid = false;
            }
        }
        if valid {
            total_valid += 1;
        }
    }

    println!("{:?}", total_valid);
}
