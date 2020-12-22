use std::collections::{VecDeque, HashSet};
use itertools::Itertools;

use crate::lib::*;

pub fn first() {
    let input = get_input_raw(22, 1);
    let mut decks = input
        .split("\n\n")
        .map(|deck| {
            deck.lines().skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<_>>()
        });

    let mut player1 = decks.next().unwrap();
    let mut player2 = decks.next().unwrap();

    while player1.len() > 0 && player2.len() > 0 {
        let a = player1.pop_front().unwrap();
        let b = player2.pop_front().unwrap();
        if a > b {
            player1.push_back(a);
            player1.push_back(b);
        } else {
            player2.push_back(b);
            player2.push_back(a);
        }
    }

    let winner = if player1.is_empty() {
        player2
    } else {
        player1
    };

    let score = winner.into_iter().rev().enumerate().fold(0, |acc, (i, n)| acc + (i+1)*n);
    dbg!(score);
}

pub fn play_game(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (bool, VecDeque<usize>) {
    let mut prev_states: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    while deck1.len() > 0 && deck2.len() > 0 {
        let configuration = (deck1.iter().cloned().collect_vec(), deck2.iter().cloned().collect_vec());
        if prev_states.contains(&configuration) {
            // deck 1 wins
            return (true, deck1)
        }
        prev_states.insert(configuration);

        let a = deck1.pop_front().unwrap();
        let b = deck2.pop_front().unwrap();

        if deck1.len() >= a && deck2.len() >= b {
            let mut subdeck1 = deck1.clone();
            let mut subdeck2 = deck2.clone();
            subdeck1.truncate(a);
            subdeck2.truncate(b);
            let (deck1_won, _) = play_game(subdeck1, subdeck2);
            if deck1_won {
                deck1.push_back(a);
                deck1.push_back(b);
            } else {
                deck2.push_back(b);
                deck2.push_back(a);
            }
        } else {
            if a > b {
                deck1.push_back(a);
                deck1.push_back(b);
            } else {
                deck2.push_back(b);
                deck2.push_back(a);
            }
        }
    }

    if deck1.is_empty() {
        (false, deck2)
    } else {
        (true, deck1)
    }
}

pub fn second() {
    let input = get_input_raw(22, 1);
    let mut decks = input
        .split("\n\n")
        .map(|deck| {
            deck.lines().skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<_>>()
        });

    let deck1 = decks.next().unwrap();
    let deck2 = decks.next().unwrap();

    let (_, winner) = play_game(deck1, deck2);

    let score = winner.into_iter().rev().enumerate().fold(0, |acc, (i, n)| acc + (i+1)*n);
    dbg!(score);
}
