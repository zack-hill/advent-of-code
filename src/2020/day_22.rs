use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_puzzle_1() -> u32 {
    let mut decks = parse_decks();

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
    return score_deck(&winner);
}

pub fn solve_puzzle_2() -> u32 {
    let mut decks = parse_decks();
    let winner = play_recursive_combat(&mut decks);
    return score_deck(&decks[winner]);
}

fn play_recursive_combat(decks: &mut Vec<VecDeque<u32>>) -> usize {
    let mut game_decks = HashSet::<Vec<VecDeque<u32>>>::new();

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

fn draw_cards(decks: &mut Vec<VecDeque<u32>>) -> Vec<u32> {
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

fn score_deck(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i as u32 + 1))
        .sum()
}

fn parse_decks() -> Vec<VecDeque<u32>> {
    let file = File::open("src/day_22.txt").unwrap();
    let reader = BufReader::new(file);

    let mut decks = Vec::<VecDeque<u32>>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            continue;
        } else if line.starts_with("Player") {
            decks.push(VecDeque::new());
        } else {
            decks.last_mut().unwrap().push_back(line.parse().unwrap());
        }
    }
    return decks;
}
