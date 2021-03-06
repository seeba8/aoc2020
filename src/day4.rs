use regex::Regex;

lazy_static! {
    static ref REQUIRED_FIELDS: [&'static str;7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

fn is_valid_passport(input: &str) -> bool {
    let input = input.trim();
    let field_names: Vec<&str> = input.split_ascii_whitespace()
        .map(|x| &x[..3])
        .collect();
    for requirement in REQUIRED_FIELDS.iter() {
        if !field_names.contains(&requirement) {
            return false;
        }
    }
    true
}

fn is_valid_byr(field_value: &str) -> bool {
    let field_value: usize = field_value.parse().unwrap();
    (1920..=2002).contains(&field_value)
}

fn is_valid_iyr(field_value: &str) -> bool {
    let field_value: usize = field_value.parse().unwrap();
    (2010..=2020).contains(&field_value)
}

fn is_valid_eyr(field_value: &str) -> bool {
    let field_value: usize = field_value.parse().unwrap();
    (2020..=2030).contains(&field_value)
}

fn is_valid_hgt(field_value: &str) -> bool {
    if !field_value.ends_with("in") && !field_value.ends_with("cm") { return false; }
    let height: usize = field_value[..field_value.len() - 2].parse().unwrap();
    if field_value.ends_with("cm") {
        if !(150..=193).contains(&height) { return false; }
    } else if !(59..=76).contains(&height) { return false; }
    true
}

fn is_valid_hcl(field_value: &str) -> bool {
    lazy_static! {
        static ref COLOUR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    COLOUR_RE.is_match(field_value)
}

fn is_valid_ecl(field_value: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field_value)
}

fn is_valid_pid(field_value: &str) -> bool {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    PID_RE.is_match(field_value)
}

fn is_valid_passport_data(input: &str) -> bool {
    let input = input.trim();
    for field in input.split_ascii_whitespace() {
        let field_name = &field[..3];
        let field_value = &field[4..];
        match field_name {
            "byr" => { if !is_valid_byr(field_value) { return false; } }
            "iyr" => { if !is_valid_iyr(field_value) { return false; } }
            "eyr" => { if !is_valid_eyr(field_value) { return false; } }
            "hgt" => { if !is_valid_hgt(field_value) { return false; } }
            "hcl" => { if !is_valid_hcl(field_value) { return false; } }
            "ecl" => { if !is_valid_ecl(field_value) { return false; } }
            "pid" => { if !is_valid_pid(field_value) { return false; } }
            &_ => {}
        }
    }
    true
}

pub fn count_valid_passports(batch_file: &str, check_data: bool) -> usize {
    batch_file.split("\n\n")
        .map(|passport|
            if is_valid_passport(passport)
                && (!check_data || is_valid_passport_data(passport))
            { 1 } else { 0 }
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_passport() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
        let mut iter = input.split("\n\n");
        assert_eq!(true, is_valid_passport(iter.next().unwrap()));
        assert_eq!(false, is_valid_passport(iter.next().unwrap()));
        assert_eq!(true, is_valid_passport(iter.next().unwrap()));
        assert_eq!(false, is_valid_passport(iter.next().unwrap()));
    }

    #[test]
    fn test_count_valid_passports() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
        assert_eq!(2, count_valid_passports(input, false));
    }

    #[test]
    fn test_part1() {
        println!("{}", count_valid_passports(std::fs::read_to_string("resources/day4.txt").unwrap().as_str(), false));
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(false, is_valid_passport_data(r"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"));
        assert_eq!(false, is_valid_passport_data(r"iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946"));
        assert_eq!(false, is_valid_passport_data(r"hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"));
        assert_eq!(false, is_valid_passport_data(r"hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"));
        assert_eq!(true, is_valid_passport_data(r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"));
        assert_eq!(true, is_valid_passport_data(r"eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"));
        assert_eq!(true, is_valid_passport_data(r"hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"));
        assert_eq!(true, is_valid_passport_data(r"
iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"));
    }

    #[test]
    fn test_part2() {
        println!("{}", count_valid_passports(std::fs::read_to_string("resources/day4.txt").unwrap().as_str(), true));
    }

    #[test]
    fn test_individual_values() {
        assert_eq!(true, is_valid_byr("2002"));
        assert_eq!(false, is_valid_byr("2003"));
        assert_eq!(true, is_valid_hgt("60in"));
        assert_eq!(true, is_valid_hgt("190cm"));
        assert_eq!(false, is_valid_hgt("190in"));
        assert_eq!(false, is_valid_hgt("190"));
        assert_eq!(true, is_valid_hcl("#123abc"));
        assert_eq!(false, is_valid_hcl("#123abz"));
        assert_eq!(false, is_valid_hcl("123abc"));
        assert_eq!(true, is_valid_ecl("brn"));
        assert_eq!(false, is_valid_ecl("wat"));
        assert_eq!(true, is_valid_pid("000000001"));
        assert_eq!(false, is_valid_pid("0123456789"));
    }
}
