use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/day22.txt");

type Deck = VecDeque<usize>;

fn parse_deck(input: &'static str) -> Deck {
    input.lines().skip(1).map(|l| l.parse().unwrap()).collect()
}

fn score_deck(deck: &Deck) -> usize {
    deck.iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (deck.len() - i) * v)
        .sum()
}

// True if player 1 wins
fn play_game(mut deck1: Deck, mut deck2: Deck) -> (bool, usize) {
    let mut visited = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        // Infinite recursion rule
        if !visited.insert((deck1.clone(), deck2.clone())) {
            return (true, score_deck(&deck1));
        }

        let (card1, card2) = (deck1.pop_front().unwrap(), deck2.pop_front().unwrap());
        let winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            // Recurse
            let sub_deck1 = deck1.iter().copied().take(card1).collect();
            let sub_deck2 = deck2.iter().copied().take(card2).collect();
            play_game(sub_deck1, sub_deck2).0
        } else {
            // Settle
            card1 > card2
        };

        if winner {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck2.is_empty() {
        (true, score_deck(&deck1))
    } else {
        (false, score_deck(&deck2))
    }
}

fn main() {
    let mut player_it = INPUT.split("\n\n");
    let deck1 = parse_deck(player_it.next().unwrap());
    let deck2 = parse_deck(player_it.next().unwrap());

    let res = play_game(deck1, deck2).1;
    println!("{}", res);
}
