use std::error::Error;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Rule {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

impl Rule {
    pub fn new(rule: &str) -> Option<Rule> {
        fn convert_to_u32_range(range: &str) -> Option<(u32, u32)> {
            let (v1, v2) = range.split_once('-')?;
            Some((v1.parse().ok()?, v2.parse().ok()?))
        }

        let (name, rest) = rule.trim().split_once(": ")?;
        let (range1, range2) = rest.trim().split_once(" or ")?;
        Some(Rule {
            name: name.to_string(),
            range1: convert_to_u32_range(range1)?,
            range2: convert_to_u32_range(range2)?,
        })
    }

    fn is_valid(&self, value: u32) -> bool {
        value >= self.range1.0 && value <= self.range1.1
            || value >= self.range2.0 && value <= self.range2.1
    }
}

fn get_sum_of_invalid_values(rules: &[Rule], ticket: &str) -> u32 {
    ticket.split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .filter(|v| rules.iter().all(|r| !r.is_valid(*v)))
        .sum()
}

fn is_valid(ticket: &str, rules: &[Rule]) -> bool {
    for field in ticket.split(',') {
        let field = field.parse::<u32>();
        if field.is_err() { return false; }
        let field = field.unwrap();
        if rules.iter().all(|rule| !rule.is_valid(field)) {
            return false;
        }
    }
    true
}

pub fn get_ticket_scanning_error_rate(input: &str) -> Result<u32, Box<dyn Error>> {
    let rules = get_rules(input).ok_or("Error getting rules")?;
    let nearby_tickets = get_nearby_tickets(input).ok_or("Error getting nearby tickets")?;
    let mut sum = 0;
    for ticket in nearby_tickets {
        sum += get_sum_of_invalid_values(&rules, &ticket);
    }
    Ok(sum)
}

pub fn get_field_order(rules: &[Rule], tickets: &[&str]) -> Result<HashMap<String, Vec<usize>>, Box<dyn Error>> {
    let mut valid_tickets: Vec<&str> = Vec::new();
    for &ticket in tickets {
        if is_valid(ticket, rules) {
            valid_tickets.push(ticket);
        }
    }
    let mut possibilities: HashMap<String, Vec<usize>> = HashMap::new();
    for i in 0..valid_tickets.get(0).ok_or("No valid tickets")?.split(',').count() {
        'outer: for rule in rules {
            for &ticket in &valid_tickets {
                let val: u32 = ticket.split(',').nth(i).ok_or("No nth segment in ticket")?.parse()?;
                if !rule.is_valid(val) {
                    // println!("Field {} is not rule {:?} because of ticket {}", i, rule, ticket);
                    continue 'outer;
                }
            }
            match possibilities.get_mut(&rule.name) {
                None => {
                    possibilities.insert(rule.name.clone(), Vec::new());
                    possibilities.get_mut(&rule.name).ok_or("no rule found")?.push(i);
                }
                Some(p) => {
                    p.push(i);
                }
            }
        }
    }
    Ok(possibilities)
}

fn remove_used_options(mappings: &mut HashMap<String, Vec<usize>>) {
    while mappings.iter().any(|(_field_name, field_positions)| field_positions.len() > 1) {
        let mut singles: HashSet<usize> = HashSet::new();
        for (_field_name, field_positions) in mappings.iter() {
            if field_positions.len() == 1 {
                singles.insert(*field_positions.get(0).unwrap());
            }
        }
        for single in singles {
            for (_field_name, field_positions) in mappings.iter_mut() {
                if field_positions.len() > 1 {
                    field_positions.retain(|x| *x != single);
                }
            }
        }
    }
}

fn get_rules(input: &str) -> Option<Vec<Rule>> {
    let mut rules = Vec::new();
    let mut sections = input.split("\n\n");
    let rules_str = sections.next()?;
    for r in rules_str.trim().split('\n') {
        rules.push(Rule::new(r)?);
    }
    Some(rules)
}

fn get_nearby_tickets(input: &str) -> Option<Vec<String>> {
    Some(input
        .split("\n\n")
        .nth(2)?
        .trim()
        .split('\n')
        .skip(1)
        .map(|s| s.to_owned())
        .collect())
}

fn get_my_ticket(input: &str) -> Option<Vec<usize>> {
    let my_ticket = input
        .split("\n\n")
        .nth(1)?
        .trim()
        .split_once('\n')?.1;

    Some(my_ticket.split(',').map(|v| v.parse().unwrap()).collect())
}

pub fn get_checksum(input: &str) -> Option<usize> {
    let nearby_tickets = get_nearby_tickets(&input)?;
    let nearby_tickets: Vec<&str> = nearby_tickets.iter().map(|s| s.as_str()).collect();
    let rules = get_rules(&input)?;
    let mut field_order = get_field_order(&rules, &nearby_tickets).ok()?;
    remove_used_options(&mut field_order);
    let my_ticket = get_my_ticket(&input)?;
    Some(field_order.iter()
        .filter(|(field_name, _)| field_name.starts_with("departure"))
        .map(|(_, field_positions)| my_ticket[field_positions[0]])
        .product())
}


#[cfg(test)]
mod tests {
    use crate::day16::*;

    #[test]
    fn test_new_rule() {
        assert_eq!(Rule {
            name: "class".to_string(),
            range1: (1, 3),
            range2: (5, 7),
        }, Rule::new("class: 1-3 or 5-7").unwrap());
        assert_ne!(Rule {
            name: "class".to_string(),
            range1: (1, 3),
            range2: (5, 7),
        }, Rule::new("class: 1-3 or 5-8").unwrap());
        assert_ne!(Rule {
            name: "class".to_string(),
            range1: (1, 3),
            range2: (5, 7),
        }, Rule::new("clasz: 1-3 or 5-7").unwrap());
    }

    #[test]
    fn test_is_valid() {
        let r1 = Rule::new("class: 1-3 or 5-7").unwrap();
        assert_eq!(true, r1.is_valid(3));
        assert_eq!(false, r1.is_valid(4));
        assert_eq!(true, r1.is_valid(5));
    }

    #[test]
    fn test_sum_invalids() {
        let _rules = vec![Rule::new("class: 1-3 or 5-7").unwrap(),
                          Rule::new("row: 6-11 or 33-44").unwrap(),
                          Rule::new("seat: 13-40 or 45-50").unwrap()
        ];
        let tickets = r"7,3,47
40,4,50
55,2,20
38,6,12";
        for _ticket in tickets.split('\n') {
            /* ticket.split(',')
                 .map(|v| v.parse::<u32>().unwrap())
                 .filter(|v| rules.iter().all(|r| !r.is_valid(*v)))
                 .sum()*/
        }
    }

    #[test]
    fn test_get_sum_of_invalid_values() {
        let rules = vec![Rule::new("class: 1-3 or 5-7").unwrap(),
                         Rule::new("row: 6-11 or 33-44").unwrap(),
                         Rule::new("seat: 13-40 or 45-50").unwrap()
        ];
        assert_eq!(0, get_sum_of_invalid_values(&rules, "7,3,47"));
        assert_eq!(4, get_sum_of_invalid_values(&rules, "40,4,50"));
        assert_eq!(55, get_sum_of_invalid_values(&rules, "55,2,20"));
        assert_eq!(12, get_sum_of_invalid_values(&rules, "38,6,12"));
    }

    #[test]
    fn test_ticket_scanning_error_rate() {
        let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(71, get_ticket_scanning_error_rate(input).unwrap());
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day16.txt").unwrap();
        println!("{}", get_ticket_scanning_error_rate(input.as_str()).unwrap());
    }

    #[test]
    fn test_is_valid_ticket() {
        let rules = vec![Rule::new("class: 1-3 or 5-7").unwrap(),
                         Rule::new("row: 6-11 or 33-44").unwrap(),
                         Rule::new("seat: 13-40 or 45-50").unwrap()
        ];
        let ticket = "7,3,47";
        assert_eq!(true, is_valid(ticket, &rules));
        let ticket = "40,4,50";
        assert_eq!(false, is_valid(ticket, &rules));
        let ticket = "55,2,20";
        assert_eq!(false, is_valid(ticket, &rules));
        let ticket = "38,6,12";
        assert_eq!(false, is_valid(ticket, &rules));
    }

    #[test]
    fn test_field_order() {
        let rules = vec![Rule::new("class: 0-1 or 4-19").unwrap(),
                         Rule::new("row: 0-5 or 8-19").unwrap(),
                         Rule::new("seat: 0-13 or 16-19").unwrap()
        ];
        let tickets: Vec<&str> = r"3,9,18
15,1,5
5,14,9".trim().split('\n').collect();
        println!("{:?}", get_field_order(&rules, &tickets));
    }

    #[test]
    fn test_get_nearby_tickets() {
        let input = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let nearby_tickets: Vec<&str> = "3,9,18
15,1,5
5,14,9".split('\n').collect();
        assert_eq!(nearby_tickets, get_nearby_tickets(input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day16.txt").unwrap();
        let nearby_tickets = get_nearby_tickets(&input).unwrap();
        let nearby_tickets: Vec<&str> = nearby_tickets.iter().map(|s| s.as_str()).collect();
        let rules = get_rules(&input).unwrap();
        println!("{:?}", get_field_order(&rules, &nearby_tickets));

        let my_ticket: Vec<u64> = "131,67,137,61,149,107,109,79,71,127,173,157,167,139,151,163,59,53,113,73"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{}", my_ticket[14] * my_ticket[0] * my_ticket[7] * my_ticket[1] * my_ticket[3] * my_ticket[8]);
        /*
        {"train": [10],
        "departure date": [14],
        "type": [17],
        "row": [9 ],
        "seat": [4 ],
        "route": [19],
        "arrival track": [18],
        "zone": [2],
        "departure time": [0],
        "arrival location": [12 ],
        "departure track": [ 7],
        "price": [15 ],
        "departure location": [1],
        "duration": [13 ],
        "departure station": [3],
        "departure platform": [ 8],
        "class": [11],
        "arrival platform": [16],
        "arrival station": [6],
        "wagon": [5]}
         */
    }

    #[test]
    fn test_get_hash() {
        let input = std::fs::read_to_string("resources/day16.txt").unwrap();
        assert_eq!(453459307723, get_checksum(&input).unwrap());
    }
}