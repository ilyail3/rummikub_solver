use std::collections::BTreeSet;

use itertools::Itertools;

use crate::Piece;
use crate::sort_set::sort_sets;
use crate::validate::valid_set;

fn tile_target(target: usize) -> Vec<[u8; 3]> {
    let mut results = Vec::new();

    for c in 0..((target / 5) + 1) {
        let left = target - (c * 5);

        for b in 0..((left / 4) + 1) {
            let left2 = left - (b * 4);

            if left2 % 3 == 0 {
                results.push([
                    (left2 / 3) as u8,
                    b as u8,
                    c as u8
                ])
            }
        }
    }

    results
}

fn find_valid(pieces: &Vec<Piece>, group_sizes: [u8; 3], left: &mut BTreeSet<usize>, current: &mut Vec<Vec<usize>>) -> bool {
    let final_round = group_sizes[0] + group_sizes[1] + group_sizes[2] == 1;

    let (next_group_size, group_size) = if group_sizes[0] > 0 {
        ([group_sizes[0] - 1, group_sizes[1], group_sizes[2]], 3)
    } else if group_sizes[1] > 0 {
        ([group_sizes[0], group_sizes[1] - 1, group_sizes[2]], 4)
    } else {
        ([group_sizes[0], group_sizes[1], group_sizes[2] - 1], 5)
    };

    let snapshot = left.clone();
    let mut resulting_set = Vec::new();

    for combination in snapshot.iter().combinations(group_size) {
        resulting_set.clear();

        for i in &combination {
            resulting_set.push(pieces[**i].clone())
        }

        if valid_set(&resulting_set) {
            let mut indexes = Vec::new();
            for i in &combination {
                left.remove(*i);
                indexes.push(**i);
            }

            current.push(indexes);

            if final_round {
                return true;
            } else {
                if find_valid(pieces, next_group_size, left, current) {
                    return true;
                } else {
                    // Revert the current push
                    current.pop();

                    // and the removal of the options from "left"
                    for i in &combination {
                        left.insert(**i);
                    }
                }
            }
        }
    }
    false
}


pub fn solve_board(pieces: Vec<Piece>) -> Option<Vec<Vec<Piece>>> {
    let mut left = BTreeSet::new();
    let mut current: Vec<Vec<usize>> = Vec::new();

    for i in 0..pieces.len() {
        left.insert(i);
    }

    for group_sizes in tile_target(pieces.len()) {
        if find_valid(&pieces, group_sizes, &mut left, &mut current) {
            let mut results = Vec::new();

            for c in current {
                let mut set = Vec::new();

                for i in c {
                    set.push(pieces[i].clone());
                }

                results.push(set);
            }

            return Some(sort_sets(results));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{Color, Piece};
    use crate::solve_tile::{solve_board, tile_target};

    #[test]
    fn allow_case_with_2_groups() {
        assert_eq!(solve_board(vec!(
            Piece::normal(1, Color::Black),
            Piece::normal(1, Color::Red),
            Piece::normal(1, Color::Blue),
            Piece::normal(2, Color::Black),
            Piece::Joker,
            Piece::normal(2, Color::Blue)
        )), Some(vec!(
            vec!(
                Piece::normal(1, Color::Black),
                Piece::normal(1, Color::Blue),
                Piece::normal(1, Color::Red),
            ),
            vec!(
                Piece::normal(2, Color::Black),
                Piece::normal(2, Color::Blue),
                Piece::Joker,
            )
        )));
    }

    #[test]
    fn test_1_option() {
        assert_eq!(
            tile_target(5),
            vec!(
                [0, 0, 1]
            )
        );

        assert_eq!(
            tile_target(6),
            vec!(
                [2, 0, 0]
            )
        );

        assert_eq!(
            tile_target(20),
            vec!(
                [4, 2, 0],
                [0, 5, 0],
                [5, 0, 1],
                [1, 3, 1],
                [2, 1, 2],
                [0, 0, 4]
            )
        );
    }

    #[test]
    fn tile_full_board() {
        let options = tile_target(98);
        assert_eq!(options.len(), 90);
    }
}