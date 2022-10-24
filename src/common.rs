use std::collections::BTreeSet;
use crate::{NormalPiece, Piece};


pub(crate) struct ConsecutiveSet {
    holes: u8,
    jokers: u8,
    first: u8,
    items: u8
}

impl ConsecutiveSet {
    pub(crate) fn new(pieces: &Vec<Piece>) -> Option<Self> {
        let mut dom:BTreeSet<u8> = BTreeSet::new();
        let mut jokers = 0;

        for piece in pieces {
            if let Piece::Normal(value) = piece {
                if dom.contains(&value.domination) {
                    return None;
                }

                dom.insert(value.domination);
            } else {
                jokers += 1;
            }
        }

        let mut last_value_opt:Option<u8> = None;
        let mut holes = 0;
        let mut first:u8 = 0;

        for domination in dom {
            if let Some(last_value) = last_value_opt {
                holes += domination - last_value - 1;
            }

            if first == 0 {
                first = domination;
            }

            last_value_opt = Some(domination);
        }

        Some(ConsecutiveSet{
            first,
            holes,
            jokers,
            items: pieces.len() as u8
        })
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.holes <= self.jokers
    }

    pub(crate) fn actual_first(&self) -> u8 {
        let free_jokers = self.jokers - self.holes;

        if self.first <= free_jokers {
            1
        } else {
            self.first - free_jokers
        }
    }

    fn actual_last(&self) -> u8 {
        let first = self.actual_first();

        first + self.jokers + self.items - 1
    }

    pub(crate) fn available_domination(&self) -> BTreeSet<u8> {
        let mut result = BTreeSet::new();

        for i in self.actual_first() .. (self.actual_last() + 1) {
            result.insert(i as u8);
        }

        result
    }
}

pub(crate) fn first_non_joker(pieces: &Vec<Piece>) -> Option<&NormalPiece> {
    for piece in pieces {
        if let Piece::Normal(value) = piece {
            return Some(value);
        }
    }

    None
}

pub(crate) fn same_domination(first: &NormalPiece, pieces: &Vec<Piece>) -> bool {
    pieces.iter().all(|v| match v {
        Piece::Joker => true,
        Piece::Normal(normal) => first.domination == normal.domination
    })
}

pub(crate) fn same_color(first: &NormalPiece, pieces: &Vec<Piece>) -> bool {
    pieces.iter().all(|v| match v {
        Piece::Joker => true,
        Piece::Normal(normal) => first.color == normal.color
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use crate::common::ConsecutiveSet;

    #[test]
    fn generate_correct_available_domination() {
        let mut set:BTreeSet<u8> = BTreeSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        assert_eq!(ConsecutiveSet{
            holes: 1,
            jokers: 2,
            first: 2,
            items: 2
        }.available_domination(), set);
    }
}