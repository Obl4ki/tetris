#![allow(unused_imports, dead_code)]

use crate::tetris::blocks::BlockType;
use crate::tetris::game::Game;
use crate::tetris::piece::Piece;

use crate::tetris::piece::{get_i, get_random_falling_piece};
pub struct GameBuilder {
    data: Vec<Vec<BlockType>>,
    width: usize,
    height: usize,
    falling_piece: Option<Piece>,
}

impl GameBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![BlockType::None; height]; width],
            width,
            height,
            falling_piece: None,
        }
    }

    pub fn add_blocks(mut self, blocks: Vec<(usize, usize)>, block_type: BlockType) -> Self {
        for (x, y) in blocks {
            self.data[x][y] = block_type;
        }

        self
    }

    pub fn set_falling_piece(mut self, piece: Piece) -> Self {
        self.falling_piece = Some(piece);
        self
    }

    pub fn compile(self) -> Game {
        let falling_piece = self.falling_piece.unwrap_or_else(get_random_falling_piece);
        Game {
            board: self.data,
            falling_piece,
            width: self.width,
            height: self.height,
        }
    }
}