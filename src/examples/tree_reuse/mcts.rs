use core::f32;

use crate::chess::{ChessBoard, ChessPosition, Move, Piece, Side};
use super::tree::{GameResult, Node, Tree};

#[allow(unused)]
pub fn search(position: &ChessPosition, tree: &mut Tree, nodes: usize) -> (Move, f32) {
    let mut iters = 0;

    loop {
        //We need to copy the position, to make sure that reference stays unaffected
        //as we apply the moves during tree travels
        let mut position = *position;

        iteration(&mut position, tree);

        iters += 1;

        if nodes <= iters {
            break;
        }
    }

    return best_move(&tree);
}

fn iteration(position: &mut ChessPosition, tree: &mut Tree) {

    //1. Create the selection stack to have a path you will unroll
    //in backpropagation
    let mut selection_stack: Vec<usize> = Vec::new();
    let mut current_node = 0;
    selection_stack.push(current_node);
    
    //2. Keep selecting the node until you end up on the node without children (leaf of terminal)
    while tree[current_node].children.len() > 0 {
        current_node = select(current_node, tree);
        selection_stack.push(current_node);
        position.make_move_no_mask(tree[current_node].mv);
    }

    //3. Expand the leaf and select the child with highest policy (unless the selected node is terminal)
    //Mind that we only do it on SECOND visit now, skipping it entirely if child.vists == 0
    if tree[current_node].visits == 1 && tree[current_node].result == GameResult::Ongoing {
        expand(current_node, position, tree);
        current_node = select(current_node, tree);
        selection_stack.push(current_node);
        position.make_move_no_mask(tree[current_node].mv);
    }

    //4. Obtain an evaluation of the position from side to move POV
    let mut score = simulate(current_node, position, tree);

    let mut child_idx = None;

    //5. Backpropagate the evaluation of the node
    for &node_idx in selection_stack.iter().rev() {
        backpropagate(node_idx, &mut score, tree);

        //Offset result backprop by 1, to make sure we have access to parent and child
        //at the same time. We dont have to set the result of the leaf child like we did
        //with score, because that's already handled in the simulation step.
        if let Some(idx) = child_idx {
            backpropagate_result(node_idx, idx, tree);
            child_idx = Some(node_idx);
        } else {
            child_idx = Some(node_idx);
        }
    }
}

//Retuns index of a selected node
fn select(parent_idx: usize, tree: &Tree) -> usize {
    //Approx sqrt(2), it's good default value, but you want to spsa it later
    const C: f32 = 1.41;

    let parent_node = &tree[parent_idx];

    let mut best_node = parent_node.children[0];
    let mut best_score = f32::NEG_INFINITY;

    for &child_idx in parent_node.children.iter() {
        let child = &tree[child_idx];

        let q = if child.visits == 0 {
            //We dont have any information about this child, so we assume it's neutral
            0.5
        } else {
            child.score / child.visits as f32
        };

        let p = child.policy;
        let expl_factor = (parent_node.visits.max(1) as f32).sqrt() / (child.visits as f32 + 1.0);

        let score = q + C * p * expl_factor;

        if score > best_score {
            best_node = child_idx;
            best_score = score;
        }
    }

    return best_node;
}

fn expand(parent_idx: usize, position: &ChessPosition, tree: &mut Tree) {
    let mut legal_moves = Vec::new();
    position.board().map_legal_moves(|mv| legal_moves.push(mv));

    let policy = 1.0 / legal_moves.len() as f32;

    for mv in legal_moves {
        let new_node_idx = tree.push_node(&Node { 
            score: 0.0, 
            visits: 0, 
            policy, 
            mv, 
            result: GameResult::Ongoing, 
            children: Vec::new() 
        });

        tree[parent_idx].children.push(new_node_idx);
    }
}

fn simulate(node_idx: usize, position: &ChessPosition, tree: &mut Tree) -> f32 {
    
    //If it's the first time we evaluate the node, we want to check it's terminal state
    if tree[node_idx].visits == 0 {
        let mut possible_moves = 0;
        position.board().map_legal_moves(|_| possible_moves += 1);

        let key = position.board().hash();
        let is_draw =  position.board().half_moves() >= 100 
                || position.board().is_insufficient_material() 
                || position.history().get_repetitions(key) >= 2;

        let result = if possible_moves == 0 {
            if position.board().is_in_check() {
                GameResult::Lose(0)
            } else {
                GameResult::Draw
            }
        } else if is_draw {
            GameResult::Draw
        } else {
            GameResult::Ongoing
        };

        tree[node_idx].result = result;
    }

    match tree[node_idx].result {
        GameResult::Draw => 0.5,
        GameResult::Lose(_) => 0.0,
        GameResult::Win(_) => 1.0,
        _ => {
            //Sigmoid of material count to make sure that score is from 0 to 1
            1.0 / (1.0 + (material_count(position.board()) as f32 / -400.0).exp())
        }
    }
}

fn backpropagate(node_idx: usize, score: &mut f32, tree: &mut Tree) {
    
    //We reverse the score from the very start, to make sure that nodes store score from opposite POV,
    //so we can use it directly in the puct, without the need to flip it there
    *score = 1.0 - *score;

    tree[node_idx].score += *score;
    tree[node_idx].visits += 1;
}   

//Implementation based on the Monty engine
fn backpropagate_result(node_idx: usize, child_idx: usize, tree: &mut Tree) {
    match tree[child_idx].result {
        GameResult::Lose(x) => {
            tree[node_idx].result = GameResult::Win(x + 1);
        }
        GameResult::Win(x) => {
            let mut proven_loss = true;
            let mut prove_length = x;

            for &idx in tree[node_idx].children.iter() {
                if let GameResult::Win(y) = tree[idx].result {
                    prove_length = x.max(y);
                } else {
                    proven_loss = false;
                }
            }

            if proven_loss {
                tree[node_idx].result = GameResult::Lose(prove_length + 1);
            }
        }
        _ => (),
    }
}   

fn best_move(tree: &Tree) -> (Move, f32) {
    let root_node = &tree[0];

    let mut best_move = Move::NULL;
    let mut best_score = f32::NEG_INFINITY;

    for &child_idx in root_node.children.iter() {
        let child = &tree[child_idx];

        //We don't want to select moves that were not explored at all
        //Not skipping them might result in weird scenario where unexplored node has highest
        //score, effectivly making out engine walk blind
        if child.visits == 0 {
            continue;
        }

        let score = child.score / child.visits as f32;

        if score > best_score {
            best_move = child.mv;
            best_score = score;
        }
    }

    (best_move, best_score)
}

fn material_count(board: &ChessBoard) -> i32 {
    let white_pieces = board.piece_mask_for_side(Piece::PAWN, Side::WHITE).pop_count() as i32 * 100 + 
        board.piece_mask_for_side(Piece::KNIGHT, Side::WHITE).pop_count() as i32 * 300 + 
        board.piece_mask_for_side(Piece::BISHOP, Side::WHITE).pop_count() as i32 * 300 + 
        board.piece_mask_for_side(Piece::ROOK, Side::WHITE).pop_count() as i32 * 500 + 
        board.piece_mask_for_side(Piece::QUEEN, Side::WHITE).pop_count() as i32 * 900;

    let black_pieces = board.piece_mask_for_side(Piece::PAWN, Side::BLACK).pop_count() as i32 * 100 + 
        board.piece_mask_for_side(Piece::KNIGHT, Side::BLACK).pop_count() as i32 * 300 + 
        board.piece_mask_for_side(Piece::BISHOP, Side::BLACK).pop_count() as i32 * 300 + 
        board.piece_mask_for_side(Piece::ROOK, Side::BLACK).pop_count() as i32 * 500 + 
        board.piece_mask_for_side(Piece::QUEEN, Side::BLACK).pop_count() as i32 * 900;

    if board.side() == Side::WHITE {
        white_pieces - black_pieces
    } else {
        black_pieces - white_pieces
    }
}