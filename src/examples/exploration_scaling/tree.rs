use std::ops::{Index, IndexMut};

use crate::chess::{ChessPosition, Move, ZobristKey};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Ongoing,
    Win(u8),
    Lose(u8),
    Draw,
}

impl Default for GameResult {
    fn default() -> Self {
        Self::Ongoing
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub score: f32,
    pub visits: u32,
    pub policy: f32,
    pub mv: Move,
    pub result: GameResult,
    pub children: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Tree {
    pub nodes: Vec<Node>
}

impl Tree {
    pub fn push_node(&mut self, node: &Node) -> usize {
        self.nodes.push(node.clone());
        self.nodes.len() - 1
    }

    #[allow(unused)]
    pub fn move_root(&mut self, new_root: usize) {
        self[0] = self[new_root].clone();
    }

    #[allow(unused)]
    pub fn find_new_root(&self, base_position: &ChessPosition, target_hash: ZobristKey) -> Option<usize> {
        self.find_new_root_internal(0, base_position, target_hash, 2)
    }

    fn find_new_root_internal(&self, node_idx: usize, position: &ChessPosition, target: ZobristKey, depth: usize) -> Option<usize> {
        if position.board().hash() == target {
            return Some(node_idx);
        } 

        if depth == 0 {
            return None;
        }

        for &child_idx in self[node_idx].children.iter() {
            let child = &self[child_idx];
            let mut new_position = position.clone();
            new_position.make_move_no_mask(child.mv);

            let result = self.find_new_root_internal(child_idx, &new_position, target, depth - 1);

            if result.is_some() {
                return result;
            }
        }

        return None;
    }
}

impl Index<usize> for Tree {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Tree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}