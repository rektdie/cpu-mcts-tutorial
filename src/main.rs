#[allow(unused)]
use crate::chess::{ChessBoard, ChessPosition, FEN, Move, MoveFlag, Square};

mod chess;
pub mod utils;
mod examples;

fn main() {
    
    use examples::exploration_scaling::tree::{Tree, Node};
    use examples::exploration_scaling::mcts::search;
    
    let mut tree = Tree::default();
    tree.nodes.push(Node::default());
    
    let position = ChessPosition::from(ChessBoard::from(&FEN::from("rnb1kbnr/pppppppp/8/1q6/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1")));
    let (best_move, _) = search(&position, &mut tree, 1_000_000);
    assert_eq!(best_move.to_string(false), "f1b5");

    tree = Tree::default();
    tree.nodes.push(Node::default());

    let position = ChessPosition::from(ChessBoard::from(&FEN::from("8/8/k7/8/4K3/8/3R4/1R6 w - - 0 1")));
    let (best_move, _) = search(&position, &mut tree, 1_000_000);
    assert_eq!(best_move.to_string(false), "d2a2");

    tree = Tree::default();
    tree.nodes.push(Node::default());

    let position = ChessPosition::from(ChessBoard::from(&FEN::from("8/8/K7/8/4k3/8/3r4/1r6 b - - 0 1")));
    let (best_move, _) = search(&position, &mut tree, 1_000_000);
    assert_eq!(best_move.to_string(false), "d2a2");

    tree = Tree::default();
    tree.nodes.push(Node::default());

    let position = ChessPosition::from(ChessBoard::from(&FEN::from("6k1/r4pp1/r5p1/8/1p1Q4/qp6/5PPP/6K1 w - - 0 1")));
    let (best_move, _) = search(&position, &mut tree, 1_000_000);
    assert_eq!(best_move.to_string(false), "d4d8");

    tree = Tree::default();
    tree.nodes.push(Node::default());

    let position = ChessPosition::from(ChessBoard::from(&FEN::from("2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b - - 0 1")));
    let (_, score) = search(&position, &mut tree, 1_000_000);
    assert!(score > 0.9);

    tree = Tree::default();
    tree.nodes.push(Node::default());

    let position = ChessPosition::from(ChessBoard::from(&FEN::from("8/8/K7/8/4k3/8/3r4/1r6 w - - 0 1")));
    let (_, score) = search(&position, &mut tree, 1_000_000);
    assert!(score < 0.01);

    // tree = Tree::default();
    // tree.nodes.push(Node::default());
    // let mut position = ChessPosition::from(ChessBoard::from(&FEN::kiwipete_position()));
    // let (best_move, score) = search(&position, &mut tree, 1_000_000);
    // println!("{}  Q: {}  N:  {}", best_move.to_string(false), score, tree[0].visits);

    // let old_position = position.clone();

    // position.make_move_no_mask(best_move);
    // position.make_move_no_mask(Move::from_squares(Square::A6, Square::E2, MoveFlag::CAPTURE));

    // if let Some(new_root) = tree.find_new_root(&old_position, position.board().hash()) {
    //     tree.move_root(new_root);
    //     println!("Found reuse!")
    // } else {
    //     tree = Tree::default();
    //     tree.nodes.push(Node::default());
    // }

    // let (best_move, score) = search(&position, &mut tree, 1_000_000);
    // println!("{}  Q: {}  N:  {}", best_move.to_string(false), score, tree[0].visits);
}