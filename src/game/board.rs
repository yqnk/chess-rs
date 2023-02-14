use std::collections::HashMap;

use super::piece::{Piece, PieceType, Color, FromStr};

const GRID_SIZE: usize = 8;
const INITIAL_POSITIONS: [(&str, usize); 8] = [
    ("R", 0), ("N", 1), ("B", 2), ("Q", 3), ("K", 4), ("B", 5), ("N", 6), ("R", 7),
];

#[derive(Debug)]
pub struct Board {
    pub grid: [[Option<Piece>; GRID_SIZE]; GRID_SIZE],
    pub pieces: HashMap<usize, Piece>,
    next_id: usize
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [[None; GRID_SIZE]; GRID_SIZE];
        let mut pieces = HashMap::new();
        let mut next_id = 0;
    
        for row in [0, 7].iter() {
            for (piece_type, col) in INITIAL_POSITIONS.iter() {
                let color = if row < &2 { Color::White } else { Color::Black };
                let pawn_row = if row < &2 { &1 } else { &6 };
                
                let piece = Piece::from_str(piece_type);
                let pawn = Piece::new(PieceType::Pawn, color);
                
                let id = next_id;
                let pawn_id = next_id + 1;
                
                grid[*row][*col] = Some(piece);
                grid[*pawn_row][*col] = Some(pawn);
                
                pieces.insert(id, piece);
                pieces.insert(pawn_id, piece);
                
                next_id += 2;
            }
        }

        Board {
            grid,
            pieces,
            next_id,
        }
    }

    // Ajoute une pièce au plateau de jeu
    pub fn add_piece(&mut self, piece: Piece, row: usize, col: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.grid[row][col] = Some(piece);
        self.pieces.insert(id, piece);
        id
    }

    // Déplace une pièce sur le plateau de jeu
    pub fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        if let Some(piece) = self.grid[from_row][from_col] {
            self.grid[from_row][from_col] = None;
            self.grid[to_row][to_col] = Some(piece);
        }
    }

    // Retire une pièce du plateau de jeu
    pub fn remove_piece(&mut self, id: usize) -> Option<Piece> {
        if let Some(piece) = self.pieces.remove(&id) {
            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    if self.grid[row][col] == Some(piece) {
                        self.grid[row][col] = None;
                        break;
                    }
                }
            }
            Some(piece)
        } else {
            None
        }
    }  
}