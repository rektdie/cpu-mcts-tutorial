use std::ops::{Index, IndexMut};

use crate::chess::Move;

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