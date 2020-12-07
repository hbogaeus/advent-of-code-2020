use std::collections::HashMap;

use regex::Regex;

fn solve(input: String) {
    let raw_passports = splits(&input);

    for raw in raw_passports {
        parse_passport(&raw)
    }
}

fn validate_passport(map: HashMap<&str, &str>) -> bool {}

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
        solve(input);
    }
}
