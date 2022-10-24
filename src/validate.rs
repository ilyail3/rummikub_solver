use crate::common::{ConsecutiveSet, first_non_joker, same_color, same_domination};
use crate::Piece;
use crate::piece::Color;

fn consecutive(pieces: &Vec<Piece>) -> bool {
    // The values are 1 to 13, so you can't have more than that,
    // even if including jokers(because jokers will have to be duplicates)
    if pieces.len() > 13 {
        return false;
    } else {
        match ConsecutiveSet::new(pieces) {
            None => false,
            Some(set) => set.is_valid()
        }
    }
}

fn repeating_colors(pieces: &Vec<Piece>) -> bool {
    if pieces.len() > 4 {
        return true;
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
            (same_color(first_piece, &pieces) && consecutive(&pieces));
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