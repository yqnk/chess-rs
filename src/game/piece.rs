pub trait FromStr {
    fn from_str(s: &str) -> Self;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl FromStr for PieceType {
    fn from_str(s: &str) -> Self {
        match s {
            "K" | "k" => PieceType::King,
            "Q" | "q" => PieceType::Queen,
            "R" | "r" => PieceType::Rook,
            "B" | "b" => PieceType::Bishop,
            "N" | "n" => PieceType::Knight,
            "P" | "p" => PieceType::Pawn,
            _ => panic!("Invalid piece type: {}", s),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl FromStr for Color {
    fn from_str(s: &str) -> Self {
        if s == s.to_uppercase() { Color::White } else { Color::Black }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color
}

impl FromStr for Piece {
    fn from_str(s: &str) -> Self {
        Piece { 
            piece_type: PieceType::from_str(s),
            color: Color::from_str(s)
        }
    }
}

impl Piece {
    // Crée une nouvelle pièce avec le type et la couleur spécifiés
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}