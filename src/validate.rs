use std::collections::BTreeSet;
use crate::Piece;
use crate::piece::{Color, NormalPiece};

fn first_non_joker(pieces: &Vec<Piece>) -> Option<&NormalPiece> {
    for piece in pieces {
        if let Piece::Normal(value) = piece {
            return Some(value);
        }
    }

    None
}

fn same_domination(first: &NormalPiece, pieces: &Vec<Piece>) -> bool {
    pieces.iter().all(|v| match v {
        Piece::Joker => true,
        Piece::Normal(normal) => first.domination == normal.domination
    })
}

fn same_color(first: &NormalPiece, pieces: &Vec<Piece>) -> bool {
    pieces.iter().all(|v| match v {
        Piece::Joker => true,
        Piece::Normal(normal) => first.color == normal.color
    })
}

fn consecutive(pieces: &Vec<Piece>) -> bool {
    // The values are 1 to 13, so you can't have more than that,
    // even if including jokers(because jokers will have to be duplicates)
    if pieces.len() > 13 {
        return false
    }

    let mut dom:BTreeSet<u8> = BTreeSet::new();
    let mut jokers = 0;


    for piece in pieces {
        if let Piece::Normal(value) = piece {
            if dom.contains(&value.domination) {
                return false;
            }

            dom.insert(value.domination);
        } else {
            jokers += 1;
        }
    }

    let mut last_value_opt:Option<u8> = None;
    let mut holes = 0;

    for domination in dom {
        if let Some(last_value) = last_value_opt {
            holes += domination - last_value - 1;
        }

        last_value_opt = Some(domination);
    }

    // The set is consecutive if the number of jokers is enough to make up for the holes
    // even if there aren't holes, the jokers are ok, because they can be used mid/end set
    return holes <= jokers;
}

fn repeating_colors(pieces: &Vec<Piece>) -> bool {
    if pieces.len() > 4 {
        return true
    }

    let mut red = false;
    let mut orange = false;
    let mut black = false;
    let mut blue = false;

    for piece in pieces {
        // Given there are up to 4 pieces, joker can never be repeating
        // so you can ignore it in this check
        if let Piece::Normal(value) = piece {
            match value.color {
                Color::Black => {
                    if black {
                        return true;
                    } else {
                        black = true;
                    }
                }
                Color::Blue => {
                    if blue {
                        return true;
                    } else {
                        blue = true;
                    }
                }
                Color::Red => {
                    if red {
                        return true;
                    } else {
                        red = true;
                    }
                }
                Color::Orange => {
                    if orange {
                        return true;
                    } else {
                        orange = true;
                    }
                }
            }
        }
    }

    false
}

pub fn valid_set(pieces: &Vec<Piece>) -> bool {
    if pieces.len() < 3 {
        return false;
    }

    // Check if all members are the same domination but different colors
    if let Some(first_piece) = first_non_joker(&pieces) {
        return (same_domination(first_piece, &pieces) && !repeating_colors(&pieces)) ||
            (same_color(first_piece, &pieces) && consecutive(&pieces))

    } else {
        panic!("A set full of jokers that is more than 3 pieces in size detected");
    }
}

#[cfg(test)]
mod tests {
    use crate::Piece;
    use crate::piece::Color;
    use crate::validate::valid_set;

    #[test]
    fn reject_small_sets() {
        assert_eq!(valid_set(&vec!()), false);
    }

    #[test]
    fn allow_consecutive() {
        // No jokers
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Red),
                Piece::normal(2, Color::Red),
                Piece::normal(3, Color::Red),
            )),
            true
        );

        // Use jokers to fill single hole
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Black),
                Piece::Joker,
                Piece::Joker,
                Piece::normal(4, Color::Black),
            )),
            true
        );


        // Use jokers to fill multiple holes
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Blue),
                Piece::Joker,
                Piece::normal(3, Color::Blue),
                Piece::Joker,
                Piece::normal(5, Color::Blue),
            )),
            true
        );

        // Reject set with hole too large for the amount of jokers
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Orange),
                Piece::Joker,
                Piece::Joker,
                Piece::normal(6, Color::Orange),
            )),
            false
        );

        // Allow jokers at the start or end of a consecutive set
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Red),
                Piece::Joker,
                Piece::Joker,
            )),
            true
        );

        // Reject consecutive set of different colors
        assert_eq!(
            valid_set(&vec!(
                Piece::normal(1, Color::Black),
                Piece::Joker,
                Piece::normal(3, Color::Blue),
            )),
            false
        );

    }
}