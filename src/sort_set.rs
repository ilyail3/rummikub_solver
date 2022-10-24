use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::{Color, Piece};
use crate::common::{ConsecutiveSet, first_non_joker, same_domination};

struct SameDominationPiece {
    piece: Piece,
    effective_color: Color,
}

struct SameColorPiece {
    piece: Piece,
    effective_domination: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SetOrder {
    effective_domination: u8,
    effective_color: Color,
    set: Vec<Piece>,
}

impl SetOrder {
    fn new(input: Vec<Piece>) -> SetOrder {
        let first_piece = first_non_joker(&input).unwrap();

        if same_domination(first_piece, &input) {
            let mut available_colors = BTreeSet::new();
            available_colors.insert(Color::Black);
            available_colors.insert(Color::Blue);
            available_colors.insert(Color::Red);
            available_colors.insert(Color::Orange);

            for piece in &input {
                if let Piece::Normal(n) = piece {
                    available_colors.remove(&n.color);
                }
            }

            let mut domination_pieces = Vec::new();

            for piece in &input {
                match piece {
                    Piece::Joker => {
                        let first = available_colors.iter().next().unwrap().to_owned();

                        domination_pieces.push(SameDominationPiece {
                            piece: Piece::Joker,
                            effective_color: available_colors.take(&first).unwrap(),
                        });
                    }
                    Piece::Normal(n) => {
                        domination_pieces.push(SameDominationPiece {
                            piece: Piece::Normal(n.clone()),
                            effective_color: n.color,
                        })
                    }
                }
            }

            domination_pieces.sort_by(|dp1, dp2| {
                dp1.effective_color.partial_cmp(&dp2.effective_color).unwrap()
            });

            let first_color = domination_pieces[0].effective_color;

            SetOrder {
                effective_color: first_color,
                effective_domination: first_piece.domination,
                set: domination_pieces.iter().map(|dp| dp.piece).collect(),
            }
        } else {
            let set = ConsecutiveSet::new(&input)
                .unwrap();

            let mut available_domination = set.available_domination();

            for piece in &input {
                if let Piece::Normal(normal) = piece {
                    available_domination.remove(&normal.domination);
                }
            }

            let mut color_pieces = Vec::new();

            for piece in &input {
                match piece {
                    Piece::Joker => {
                        let first = *available_domination.iter().next().unwrap();

                        color_pieces.push(SameColorPiece {
                            piece: Piece::Joker,
                            effective_domination: available_domination.take(&first).unwrap(),
                        });
                    }
                    Piece::Normal(n) => {
                        color_pieces.push(SameColorPiece {
                            piece: Piece::Normal(n.clone()),
                            effective_domination: n.domination,
                        })
                    }
                }
            }

            color_pieces.sort_by(|cp1, cp2| {
                cp1.effective_domination.partial_cmp(&cp2.effective_domination).unwrap()
            });

            SetOrder {
                effective_color: first_piece.color,
                effective_domination: set.actual_first(),
                set: color_pieces.iter().map(|dp| dp.piece).collect(),
            }
        }
    }
}

impl PartialOrd for SetOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SetOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.effective_domination.cmp(&other.effective_domination) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.effective_color.cmp(&other.effective_color),
            Ordering::Greater => Ordering::Greater
        }
    }
}

pub(crate) fn sort_sets(sets: Vec<Vec<Piece>>) -> Vec<Vec<Piece>> {
    let mut sorted = Vec::new();

    for set in sets {
        sorted.push(SetOrder::new(set));
    }

    sorted.sort();

    // figure out if possible not to clone
    sorted.iter().map(|s| s.set.clone()).collect()
}


#[cfg(test)]
mod tests {
    use crate::Piece;
    use crate::piece::Color;
    use crate::sort_set::{SetOrder, sort_sets};

    #[test]
    fn sort_same_domination_set() {
        assert_eq!(SetOrder::new(vec!(
            Piece::normal(1, Color::Red),
            Piece::normal(1, Color::Orange),
            Piece::normal(1, Color::Black),
            Piece::normal(1, Color::Blue),
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Black,
            set: vec!(
                Piece::normal(1, Color::Black),
                Piece::normal(1, Color::Blue),
                Piece::normal(1, Color::Red),
                Piece::normal(1, Color::Orange),
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(1, Color::Orange),
            Piece::normal(1, Color::Blue),
            Piece::Joker,
            Piece::Joker,
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Black,
            set: vec!(
                Piece::Joker,
                Piece::normal(1, Color::Blue),
                Piece::Joker,
                Piece::normal(1, Color::Orange),
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(1, Color::Red),
            Piece::Joker,
            Piece::normal(1, Color::Black),
            Piece::Joker,
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Black,
            set: vec!(
                Piece::normal(1, Color::Black),
                Piece::Joker,
                Piece::normal(1, Color::Red),
                Piece::Joker,
            ),
        });
    }

    #[test]
    fn sort_same_color_set() {
        assert_eq!(SetOrder::new(vec!(
            Piece::normal(3, Color::Red),
            Piece::normal(2, Color::Red),
            Piece::normal(1, Color::Red),
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Red,
            set: vec!(
                Piece::normal(1, Color::Red),
                Piece::normal(2, Color::Red),
                Piece::normal(3, Color::Red),
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(3, Color::Red),
            Piece::normal(2, Color::Red),
            Piece::Joker,
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Red,
            set: vec!(
                Piece::Joker,
                Piece::normal(2, Color::Red),
                Piece::normal(3, Color::Red),
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(3, Color::Red),
            Piece::Joker,
            Piece::normal(1, Color::Red),
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Red,
            set: vec!(
                Piece::normal(1, Color::Red),
                Piece::Joker,
                Piece::normal(3, Color::Red),
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(3, Color::Red),
            Piece::normal(2, Color::Red),
            Piece::normal(1, Color::Red),
            Piece::normal(4, Color::Red),
            Piece::normal(7, Color::Red),
            Piece::normal(11, Color::Red),
            Piece::normal(8, Color::Red),
            Piece::normal(6, Color::Red),
            Piece::normal(12, Color::Red),
            Piece::normal(5, Color::Red),
            Piece::normal(10, Color::Red),
            Piece::normal(9, Color::Red),
            Piece::Joker
        )), SetOrder {
            effective_domination: 1,
            effective_color: Color::Red,
            set: vec!(
                Piece::normal(1, Color::Red),
                Piece::normal(2, Color::Red),
                Piece::normal(3, Color::Red),
                Piece::normal(4, Color::Red),
                Piece::normal(5, Color::Red),
                Piece::normal(6, Color::Red),
                Piece::normal(7, Color::Red),
                Piece::normal(8, Color::Red),
                Piece::normal(9, Color::Red),
                Piece::normal(10, Color::Red),
                Piece::normal(11, Color::Red),
                Piece::normal(12, Color::Red),
                Piece::Joker
            ),
        });

        assert_eq!(SetOrder::new(vec!(
            Piece::normal(13, Color::Red),
            Piece::Joker,
            Piece::normal(11, Color::Red),
            Piece::Joker,
        )), SetOrder {
            effective_domination: 10,
            effective_color: Color::Red,
            set: vec!(
                Piece::Joker,
                Piece::normal(11, Color::Red),
                Piece::Joker,
                Piece::normal(13, Color::Red),
            ),
        });
    }

    #[test]
    fn test_sort_sets() {
        assert_eq!(sort_sets(vec!(
            vec!(
                Piece::normal(3, Color::Red),
                Piece::normal(2, Color::Red),
                Piece::normal(1, Color::Red),
            ),
            vec!(
                Piece::normal(3, Color::Black),
                Piece::normal(2, Color::Black),
                Piece::normal(1, Color::Black),
            ),
            vec!(
                Piece::normal(4, Color::Black),
                Piece::normal(3, Color::Black),
                Piece::Joker,
            )
        )), vec!(
            vec!(
                Piece::normal(1, Color::Black),
                Piece::normal(2, Color::Black),
                Piece::normal(3, Color::Black),
            ),
            vec!(
                Piece::normal(1, Color::Red),
                Piece::normal(2, Color::Red),
                Piece::normal(3, Color::Red),
            ),
            vec!(
                Piece::Joker,
                Piece::normal(3, Color::Black),
                Piece::normal(4, Color::Black),
            )
        ))
    }
}