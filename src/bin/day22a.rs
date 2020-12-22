use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day22.txt");

fn parse_deck(input: &'static str) -> VecDeque<usize> {
    input.lines().skip(1).map(|l| l.parse().unwrap()).collect()
}

fn score_deck(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (deck.len() - i) * v)
        .sum()
}

fn main() {
    let mut player_it = INPUT.split("\n\n");
    let mut deck1 = parse_deck(player_it.next().unwrap());
    let mut deck2 = parse_deck(player_it.next().unwrap());

    while !deck1.is_empty() && !deck2.is_empty() {
        let (card1, card2) = (deck1.pop_front().unwrap(), deck2.pop_front().unwrap());

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let res = score_deck(&deck1).max(score_deck(&deck2));
    println!("{}", res);
}
