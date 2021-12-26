use crate::solver::AoCSolver;
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

type Deck = VecDeque<u32>;

pub struct Solver {
    decks: Vec<Deck>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            decks: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let mut decks = self.decks.clone();

        while decks.iter().all(|deck| !deck.is_empty()) {
            // Get the top card off each player's decks
            let cards = draw_cards(&mut decks);

            // Find the player with the highest card
            let winner = get_winner_by_high_card(&cards);

            // Add cards to winning player's deck, adding the winner's card first
            decks[winner].push_back(cards[winner]);
            decks[winner].push_back(cards[1 - winner]);
        }

        // Get deck of winning player
        let winner = decks.iter().filter(|deck| !deck.is_empty()).next().unwrap();

        // Score winning deck
        return score_deck(&winner).to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut decks = self.decks.clone();
        let winner = play_recursive_combat(&mut decks);
        return score_deck(&decks[winner]).to_string();
    }
}

fn play_recursive_combat(decks: &mut Vec<Deck>) -> usize {
    let mut game_decks = HashSet::<Vec<Deck>>::new();

    while decks.iter().all(|deck| !deck.is_empty()) {
        // Player 1 wins if this round has already been played
        if game_decks.contains(decks) {
            return 0;
        }

        // Store the current decks to prevent infinite loops
        game_decks.insert(decks.clone());

        let cards = draw_cards(decks);

        // Check if both players have enough cards in their deck to recurse
        let can_recurse = cards
            .iter()
            .enumerate()
            .all(|(i, &card)| decks[i].len() >= card as usize);

        let winner = if can_recurse {
            let mut sub_game_decks = decks
                .iter()
                .enumerate()
                .map(|(i, deck)| deck.iter().take(cards[i] as usize).cloned().collect())
                .collect();
            play_recursive_combat(&mut sub_game_decks)
        } else {
            get_winner_by_high_card(&cards)
        };

        // Add cards to winning player's deck
        decks[winner].push_back(cards[winner]);
        decks[winner].push_back(cards[1 - winner]);
    }

    return if decks[0].is_empty() { 1 } else { 0 };
}

fn draw_cards(decks: &mut Vec<Deck>) -> Vec<u32> {
    decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect()
}

fn get_winner_by_high_card(cards: &Vec<u32>) -> usize {
    let (winner, _) = cards
        .iter()
        .enumerate()
        .max_by_key(|(_, &card)| card)
        .unwrap();
    return winner;
}

fn score_deck(deck: &Deck) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i as u32 + 1))
        .sum()
}

fn parse_input() -> Vec<Deck> {
    let file = File::open("src/2020/day_22.txt").unwrap();
    let reader = BufReader::new(file);

    let mut decks = Vec::<Deck>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            continue;
        } else if line.starts_with("Player") {
            decks.push(Deck::new());
        } else {
            decks.last_mut().unwrap().push_back(line.parse().unwrap());
        }
    }
    return decks;
}
