#![allow(dead_code)]
#![warn(clippy::style)]
#![allow(clippy::needless_return)]

use rand::Rng;

const NUM_CANDIDATES: usize = 5;
const NUM_BALLOTS: usize = 100;

enum Ballot {
    Plurality(usize),
    Ranked([usize; NUM_CANDIDATES]),
}

impl Ballot {
    const fn as_plurality(&self) -> Self {
        return match *self {
            Self::Plurality(choice) => Self::Plurality(choice),
            Self::Ranked(choices) => Self::Plurality(choices[0]),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hash {
    key: usize,
    value: usize,
}

enum ElectionResult {
    SingleWinner(usize),
    Tie(Vec<usize>),
}

enum MinOrMax {
    Min,
    Max,
}

fn create_ballot_plurality() -> Ballot {
    let mut rng = rand::thread_rng();
    let choice: usize = rng.gen_range(0..NUM_CANDIDATES);

    return Ballot::Plurality(choice);
}

fn simulate_plurality(ballots: &[Ballot], num_candidates: usize) -> ElectionResult {
    assert!(!ballots.is_empty(), "ballot must have a size");

    let mut candidates_votes: Vec<usize> = vec![0; num_candidates];

    for ballot in ballots {
        if let Ballot::Plurality(choice) = ballot {
            candidates_votes[*choice] += 1;
        }
    }

    println!("{candidates_votes:?}");

    let winners = extrema_indeces(&candidates_votes, MinOrMax::Max);

    if winners.len() == 1 {
        return ElectionResult::SingleWinner(winners[0]);
    }

    return ElectionResult::Tie(winners);
}

fn create_ballot_ranked() -> Ballot {
    let mut choices = [0; NUM_CANDIDATES];

    let mut rng = rand::thread_rng();
    let mut index_choices: Vec<usize> = (0..NUM_CANDIDATES).collect();

    // shuffle ballot
    for (i, choice) in choices.iter_mut().enumerate() {
        let random_choice = rng.gen_range(0..(NUM_CANDIDATES - i));
        *choice = index_choices.swap_remove(random_choice);
    }

    return Ballot::Ranked(choices);
}

fn simulate_ranked(ballots: &[Ballot], num_candidates: usize) -> ElectionResult {
    let win_threshold = NUM_BALLOTS / 2;

    let mut candidates_votes: Vec<Hash> = vec![Hash { key: 0, value: 0 }; num_candidates];

    for (i, candidate) in candidates_votes.iter_mut().enumerate() {
        candidate.key = i;
    }

    let mut top_candidate_index = 0;
    let mut values_only: Vec<usize>;

    while candidates_votes[top_candidate_index].value < win_threshold {
        count_votes_ranked(candidates_votes.as_mut_slice(), ballots);
        println!("{candidates_votes:?}");

        values_only = candidates_votes
            .iter()
            .map(|&Hash { value: val, .. }| val)
            .collect();

        let losers = extrema_indeces(&values_only, MinOrMax::Min);

        if losers.len() == candidates_votes.len() {
            break;
        }

        // reverse so the index order is preserved
        for &loser in losers.iter().rev() {
            candidates_votes.remove(loser);
            values_only.remove(loser);
        }

        if candidates_votes.len() <= 2 {
            break;
        }

        let top_candidates = extrema_indeces(&values_only, MinOrMax::Max);
        top_candidate_index = top_candidates[0];
    }

    count_votes_ranked(candidates_votes.as_mut_slice(), ballots);

    println!("final: {candidates_votes:?}");

    values_only = candidates_votes
        .iter()
        .map(|&Hash { value: val, .. }| val)
        .collect();

    let winner_indeces = extrema_indeces(&values_only, MinOrMax::Max);

    let winners: Vec<usize> = winner_indeces
        .iter()
        .map(|winner_index| candidates_votes[*winner_index].key)
        .collect();

    if winners.len() == 1 {
        return ElectionResult::SingleWinner(winners[0]);
    }

    return ElectionResult::Tie(winners);
}

fn extrema_indeces(list: &[usize], min_or_max: MinOrMax) -> Vec<usize> {
    let extremum: &usize = match min_or_max {
        MinOrMax::Min => list.iter().min().unwrap(),
        MinOrMax::Max => list.iter().max().unwrap(),
    };

    return list
        .iter()
        .enumerate()
        .filter(|(_, item)| *item == extremum)
        .map(|(i, _)| i)
        .collect();
}

fn get_hash_index(hash_map: &[Hash], key: usize) -> Option<usize> {
    for (i, hash) in hash_map.iter().enumerate() {
        if hash.key == key {
            return Some(i);
        }
    }

    return None;
}

fn count_votes_ranked(candidates_votes: &mut [Hash], ballots: &[Ballot]) {
    for candidate in candidates_votes.iter_mut() {
        candidate.value = 0;
    }

    for ballot in ballots {
        if let Ballot::Ranked(choices) = *ballot {
            let mut first_available: usize = 0;

            for choice in choices {
                if candidates_votes
                    .iter()
                    .any(|&Hash { key: hash, .. }| hash == choice)
                {
                    first_available = choice;
                    break;
                }
            }

            let fac_index = get_hash_index(candidates_votes, first_available).unwrap();

            candidates_votes[fac_index].value += 1;
        }
    }
}

fn main() {
    let ranked_ballots: Vec<Ballot> = (0..NUM_BALLOTS)
        .map(|_| create_ballot_ranked())
        .collect();

    let plurality_ballots: Vec<Ballot> = ranked_ballots
        .iter()
        .map(Ballot::as_plurality)
        .collect();

    let plurality_result = simulate_plurality(&plurality_ballots, NUM_CANDIDATES);

    match plurality_result {
        ElectionResult::SingleWinner(winner) => println!("{winner} won\n"),
        ElectionResult::Tie(tied_members) => {
            print!("tie between ");

            for member in tied_members {
                print!("{member}, ");
            }

            println!("\n");
        }
    }

    let ranked_result = simulate_ranked(&ranked_ballots, NUM_CANDIDATES);

    match ranked_result {
        ElectionResult::SingleWinner(winner) => println!("{winner} won"),
        ElectionResult::Tie(tied_members) => {
            print!("tie between ");

            for member in tied_members {
                print!("{member}, ");
            }

            println!();
        }
    }
}
