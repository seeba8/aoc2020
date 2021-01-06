use std::collections::VecDeque;

pub fn play(input: &str) -> usize {
    let mut decks: Vec<VecDeque<u8>> = get_decks(input).unwrap();
    while decks.iter().all(|d| d.len() > 0) {
        //println!("{:?}", decks);
        let mut round: Vec<u8> = decks
            .iter_mut()
            .map(|mut deck| deck.pop_front().unwrap())
            .collect();
        let mut iter = round.iter().enumerate();
        let init = iter.next().unwrap();
        let (best_player, _) = round.iter().enumerate().fold(init, |(best_player, best_card), (player, card)| {
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
    get_winning_score(decks)
}

fn start_play_recursive(input: &str) -> usize {
    let mut decks = get_decks(input).unwrap();


    0
}

fn play_recursive(decks: &mut Vec<VecDeque<u8>>) {
    if decks.iter().any(|deck| deck.len() == 0) {
        return;
    }
    while decks.iter().all(|d| d.len() > 0) {
        //println!("{:?}", decks);
        let mut round: Vec<u8> = decks
            .iter_mut()
            .map(|mut deck| deck.pop_front().unwrap())
            .collect();
        if round.iter().enumerate()
            .all(|(player, &card)| decks.get(player).unwrap().len() <= card as usize) {
            // All players have at least their card's value number of cards
            let mut sub_deck: Vec<VecDeque<u8>> = Vec::new();
            for (player, deck) in decks.iter().enumerate() {
                sub_deck.push(deck.range(0..*round.get(player).unwrap() as usize).copied().collect());
            }
            play_recursive(&mut sub_deck);

        }
    }
}

fn get_winning_score(decks: Vec<VecDeque<u8>>) -> usize {
    for deck in decks {
        if deck.len() > 0 {
            let mut score = 0;
            return deck.iter().rev().enumerate().fold(0, |acc, (k, &v)| {
                acc + (k + 1) * v as usize
            });
        }
    }
    panic!("No deck with any cards");
}

fn get_decks(input: &str) -> Option<Vec<VecDeque<u8>>> {
    let mut decks = Vec::new();
    for player in input.trim().split("\n\n") {
        decks.push(player.lines().skip(1).map(|card| card.parse().unwrap()).collect())
    }
    Some(decks)
}

#[cfg(test)]
mod tests {
    use crate::day22::{get_decks, play};
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
        assert_eq!(expected, get_decks(&input).unwrap());
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
}