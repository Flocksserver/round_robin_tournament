pub mod round_robin_tournament {
    /// Implementation of the round robin tournament algorithm.
    ///
    /// For a given `number_of_players` it return the pairs and rounds.
    pub fn retrieve_encounters(number_of_players: u32) -> Vec<Vec<(u32, u32)>> {
        let nop = if number_of_players % 2 == 0 { number_of_players } else { number_of_players + 1 };
        let mut players: Vec<u32> = (0..nop).collect();

        let mut rounds: Vec<Vec<(u32, u32)>> = Vec::new();
        let fixed_player = players.pop().unwrap().clone();
        for i in 0..(nop - 1) {
            let mut pairs = Vec::new();
            if i != 0 { players.rotate_left(1) }

            let mut players_clone = players.clone();
            players_clone.rotate_left(1);

            let first = players_clone.pop().unwrap();
            pairs.push((first.clone(), fixed_player.clone()));

            let split_list = players_clone.split_at_mut(((nop - 2) / 2) as usize);
            split_list.1.reverse();
            for j in 0..((nop - 2) / 2) {
                let a: Option<&u32> = split_list.0.get(j as usize);
                let b: Option<&u32> = split_list.1.get(j as usize);
                if a.is_some() && b.is_some() {
                    pairs.push((a.unwrap().clone(), b.unwrap().clone()))
                }
            }
            rounds.push(pairs)
        }
        rounds
    }
}

#[cfg(test)]
mod tests {
    use crate::round_robin_tournament::retrieve_encounters;

    fn tournament_test_data() -> Vec<Vec<(u32, u32)>> {
        vec![
            vec![(0, 9), (1, 8), (2, 7), (3, 6), (4, 5)],
            vec![(1, 9), (2, 0), (3, 8), (4, 7), (5, 6)],
            vec![(2, 9), (3, 1), (4, 0), (5, 8), (6, 7)],
            vec![(3, 9), (4, 2), (5, 1), (6, 0), (7, 8)],
            vec![(4, 9), (5, 3), (6, 2), (7, 1), (8, 0)],
            vec![(5, 9), (6, 4), (7, 3), (8, 2), (0, 1)],
            vec![(6, 9), (7, 5), (8, 4), (0, 3), (1, 2)],
            vec![(7, 9), (8, 6), (0, 5), (1, 4), (2, 3)],
            vec![(8, 9), (0, 7), (1, 6), (2, 5), (3, 4)],
        ]
    }

    #[test]
    fn round_robin_10_player() {
        let result = retrieve_encounters(10);
        assert_eq!(result, tournament_test_data());
    }

    #[test]
    fn round_robin_9_player() {
        let result = retrieve_encounters(11);
        assert_eq!(result, tournament_test_data());
    }
}