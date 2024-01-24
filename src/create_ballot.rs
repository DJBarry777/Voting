use crate::{ NUM_CANDIDATES, Ballot };
use rand::Rng;

pub fn create_ballot_plurality() -> Ballot {
    let mut rng = rand::thread_rng();
    let choice: usize = rng.gen_range(0..NUM_CANDIDATES);

    return Ballot::Plurality(choice);
}

pub fn create_ballot_ranked() -> Ballot {
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
