use std::collections::HashMap;

use regex::Regex;

fn solve(input: String) -> i32 {
    splits(&input)
        .iter()
        .map(parse_passport)
        .filter(validate_passport)
        .count() as i32
}

fn validate_passport(map: &HashMap<&str, &str>) -> bool {
    let required_fields: Vec<&str> = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        // "cid"
    ];

    required_fields
        .iter()
        .all(|&field| validate_field(map, field))
}

fn validate_field(map: &HashMap<&str, &str>, field: &str) -> bool {
    if !map.contains_key(field) {
        return false;
    }

    let value = map
        .get(field)
        .expect(format!("Field {} missing", field).as_str());

    validate_single_field(field, value)
}

fn validate_single_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => {
            let parsed: i32 = value.parse().expect("Not a number");
            (1920..=2002).contains(&parsed)
        }
        "iyr" => {
            let parsed: i32 = value.parse().expect("Not a number");

            (2010..=2020).contains(&parsed)
        }
        "eyr" => {
            let parsed: i32 = value.parse().expect("Not a number");

            (2020..=2030).contains(&parsed)
        }
        "hgt" => {
            let re = Regex::new(r"\b(?P<value>\d{2,3})(?P<unit>in|cm)\b").unwrap();
            let caps = re.captures(value);

            if !caps.is_some() {
                return false;
            }

            let caps = caps.unwrap();

            let value = caps.name("value").map(|m| m.as_str());
            let unit = caps.name("unit").map(|m| m.as_str());

            match (value, unit) {
                (Some(value), Some("in")) => value
                    .parse::<u32>()
                    .ok()
                    .map(|parsed| (59..=76).contains(&parsed))
                    .unwrap_or(false),

                (Some(value), Some("cm")) => value
                    .parse::<u32>()
                    .ok()
                    .map(|parsed| (150..=193).contains(&parsed))
                    .unwrap_or(false),
                _ => false,
            }
        }
        "hcl" => {
            let re = Regex::new(r"(#[0-9a-f]{6})\b").unwrap();
            re.is_match(value)
        }
        "ecl" => {
            let re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
            re.is_match(value)
        }
        "pid" => {
            let re = Regex::new(r"([0-9]{9})\b").unwrap();
            re.is_match(value)
        }
        _ => false,
    }
}

fn parse_passport(input: &String) -> HashMap<&str, &str> {
    let re = Regex::new(r"(?P<key>[a-z]{3}):(?P<value>[\S]*)\b").unwrap();

    re.captures_iter(&input)
        .map(|cap| {
            (
                cap.name("key").unwrap().as_str(),
                cap.name("value").unwrap().as_str(),
            )
        })
        .collect::<HashMap<&str, &str>>()
}

fn splits(input: &String) -> Vec<String> {
    input
        .split("\n\n")
        .map(|pass| pass.replace("\n", " "))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn testing() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            .into();
        let result = solve(input);
        // assert_eq!(result, 2)
    }

    #[test]
    fn test_byr() {
        assert_eq!(validate_single_field("byr", "2002"), true);
        assert_eq!(validate_single_field("byr", "2003"), false);
        assert_eq!(validate_single_field("byr", "1920"), true);
        assert_eq!(validate_single_field("byr", "1919"), false);
    }

    #[test]
    fn test_iyr() {
        assert_eq!(validate_single_field("iyr", "2020"), true);
        assert_eq!(validate_single_field("iyr", "2021"), false);
        assert_eq!(validate_single_field("iyr", "2010"), true);
        assert_eq!(validate_single_field("iyr", "2009"), false);
    }
    #[test]
    fn test_hgt() {
        assert_eq!(validate_single_field("hgt", "60in"), true);
        assert_eq!(validate_single_field("hgt", "190cm"), true);
        assert_eq!(validate_single_field("hgt", "190in"), false);
        assert_eq!(validate_single_field("hgt", "190"), false);
    }
    #[test]
    fn test_hcl() {
        assert_eq!(validate_single_field("hcl", "#123abc"), true);
        assert_eq!(validate_single_field("hcl", "#123abz"), false);
        assert_eq!(validate_single_field("hcl", "123abc"), false);
    }

    #[test]
    fn first_problem_test_input() {
        let input = read_to_string("src/day4/input.txt").expect("Couldnt read file");
        let result = solve(input);

        println!("{}", result);
    }
}
