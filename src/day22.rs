use std::collections::{VecDeque, HashSet};

pub fn play(input: &str) -> usize {
    let mut decks: Vec<VecDeque<u8>> = get_decks(input);
    while decks.iter().all(|d| !d.is_empty()) {
        //println!("{:?}", decks);
        let mut round: Vec<u8> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        let mut iter = round.iter().enumerate();
        let init = iter.next().unwrap();
        let (best_player, _) = iter.fold(init, |(best_player, best_card), (player, card)| {
            if card > best_card {
                (player, card)
            } else {
                (best_player, best_card)
            }
        });
        round.sort_unstable();
        round.reverse();
        decks.get_mut(best_player).unwrap().extend(round);
    }
    //println!("{:?}", decks);
    for deck in decks {
        if !deck.is_empty() {
            return get_winning_score(&deck);
        }
    }
    panic!("No deck with any cards");
}

pub fn start_play_recursive(input: &str) -> usize {
    let mut decks = get_decks(input);
    let winner = play_recursive(&mut decks);
    //println!("final decks: {:?}", decks);
    get_winning_score(&decks.get(winner).unwrap())
}

fn play_recursive(decks: &mut Vec<VecDeque<u8>>) -> usize {
    let mut previous_rounds: HashSet<Vec<VecDeque<u8>>> = HashSet::new();
    while decks.iter().all(|d| !d.is_empty()) {
        if !previous_rounds.insert(decks.clone()) {
            // seen before
            return 0;
        }
        //println!("{:?}", decks);
        let mut round: Vec<u8> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        if round.iter().enumerate()
            .all(|(player, &card)| decks.get(player).unwrap().len() >= card as usize) {
            //println!("{:?}: start subgame", round);
            // All players have at least their card's value number of cards
            let mut sub_deck: Vec<VecDeque<u8>> = Vec::new();
            for (player, deck) in decks.iter().enumerate() {
                sub_deck.push(deck.range(0..*round.get(player).unwrap() as usize).copied().collect());
            }
            //println!("{:?}", sub_deck);
            let winner = play_recursive(&mut sub_deck);
            //println!("player {} wins subgame", winner+1);
            decks.get_mut(winner).unwrap().push_back(round.remove(winner));
            decks.get_mut(winner).unwrap().push_back(round.remove(0));
        } else {
            let mut iter = round.iter().enumerate();
            let init = iter.next().unwrap();
            let (best_player, _) = iter.fold(init, |(best_player, best_card), (player, card)| {
                if card > best_card {
                    (player, card)
                } else {
                    (best_player, best_card)
                }
            });
            round.sort_unstable();
            round.reverse();
            decks.get_mut(best_player).unwrap().extend(round);
        }
    }
    for deck in decks.iter().enumerate() {
        if !deck.1.is_empty() {
            return deck.0;
        }
    }
    panic!()
}

fn get_winning_score(deck: &VecDeque<u8>) -> usize {
    return deck.iter().rev().enumerate().fold(0, |acc, (k, &v)| {
        acc + (k + 1) * v as usize
    });
}

fn get_decks(input: &str) -> Vec<VecDeque<u8>> {
    let mut decks = Vec::new();
    for player in input.trim().split("\n\n") {
        decks.push(player.lines().skip(1).map(|card| card.parse().unwrap()).collect())
    }
    decks
}

#[cfg(test)]
mod tests {
    use crate::day22::{get_decks, play, start_play_recursive};
    use std::collections::VecDeque;

    fn get_example_input() -> String {
        String::from(r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
")
    }

    #[test]
    fn test_get_decks() {
        let input = get_example_input();
        let expected = vec![VecDeque::from(vec![9, 2, 6, 3, 1]),
                            VecDeque::from(vec![5, 8, 4, 7, 10])];
        assert_eq!(expected, get_decks(&input));
    }

    #[test]
    fn test_play() {
        let input = get_example_input();
        assert_eq!(306, play(&input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day22.txt").unwrap();
        println!("{}", play(&input));
    }

    #[test]
    fn test_recursive_game() {
        let input = get_example_input();
        assert_eq!(291, start_play_recursive(&input));
    }

    #[test]
    fn test_infinite_game() {
        let input = r"Player 1:
43
19

Player 2:
2
29
14";
        println!("{}", start_play_recursive(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day22.txt").unwrap();
        println!("{}", start_play_recursive(&input));
    }
}