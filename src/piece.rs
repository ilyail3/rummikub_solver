use std::slice::Iter;
use serde::{Serialize,Deserialize};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Color {
    Black,
    Blue,
    Red,
    Orange
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 4] = [Color::Black, Color::Blue, Color::Red, Color::Orange];
        COLORS.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct NormalPiece {
    pub domination: u8,
    pub color:Color
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Piece {
    Joker,
    Normal(NormalPiece)
}

impl Piece {
    pub fn normal(domination: u8, color:Color) -> Self {
        if domination > 13 {
            panic!("got domination of more than 13: {}", domination);
        }

        return Piece::Normal(NormalPiece{ domination, color })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Piece};

    #[test]
    fn parse_json_table() {
        let data = r#"
        [
            {"type":"Joker"},
            {"type":"Normal", "domination":1, "color":"Red"}
        ]"#;

        let v: Vec<Piece> = serde_json::from_str(data).unwrap();

        assert_eq!(v, vec!(Piece::Joker, Piece::normal(1, Color::Red)));
    }
}