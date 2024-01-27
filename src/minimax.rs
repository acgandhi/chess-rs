use std::cmp::{max, min};
use shakmaty::{Board, Chess, Move, Position, Color};

fn evaluate_pos(pos: &Chess) -> i32 {
    let board = pos.board();
    let mut board_value: i32 = 0;

    let black = board.black();
    let white = board.white();

    board_value += (board.pawns() & black).count() as i32 * 1;
    board_value += (board.bishops() & black).count() as i32 * 3;
    board_value += (board.knights() & black).count() as i32 * 3;
    board_value += (board.rooks() & black).count() as i32 * 5;
    board_value += (board.queens() & black).count() as i32 * 9;
    board_value += (board.kings() & black).count() as i32 * 90;

    board_value += (board.pawns() & white).count() as i32 * -1;
    board_value += (board.bishops() & white).count() as i32 * -3;
    board_value += (board.knights() & white).count() as i32 * -3;
    board_value += (board.rooks() & white).count() as i32 * -5;
    board_value += (board.queens() & white).count() as i32 * -9;
    board_value += (board.kings() & white).count() as i32 * -90;

    return board_value;
}

pub(crate) fn minimax(pos: &Chess, depth: i32, num_nodes: &mut i64) -> Option<Move> {
    let moves = pos.legal_moves();
    let is_minimizing = if pos.turn() == Color::White {true} else {false};
    let mut best_move_value = if is_minimizing {i32::MAX} else {i32::MIN};
    let mut best_move = None;

    for m in moves {
        let mut new_pos = pos.clone();
        new_pos.play_unchecked(&m);
        let mm_value = minimax_recurse(&new_pos, depth-1, i32::MIN, i32::MAX, num_nodes);
        if is_minimizing {
            if mm_value <= best_move_value {
                best_move_value = mm_value;
                best_move = Some(m);
            }
        } else {
            if mm_value >= best_move_value {
                best_move_value = mm_value;
                best_move = Some(m);
            }
        }
    }

    return best_move;
}

fn minimax_recurse(pos: &Chess, depth: i32, mut alpha: i32, mut beta: i32, num_nodes: &mut i64) -> i32 {
    if depth == 0 {
        *num_nodes += 1;
        return evaluate_pos(pos);
    }

    let mut best_move_value = 0;
    let moves = pos.legal_moves();
    let is_minimizing = if pos.turn() == Color::White {true} else {false};

    if is_minimizing {
        best_move_value = i32::MAX;
        for m in moves {
            let mut new_pos = pos.clone();
            new_pos.play_unchecked(&m);
            best_move_value = min(
                best_move_value,
                minimax_recurse(&new_pos, depth - 1, alpha, beta, num_nodes)
            );
            beta = min(beta, best_move_value);
            if beta <= alpha {
                return best_move_value;
            }
        }
    } else {
        best_move_value = i32::MIN;
        for m in moves {
            let mut new_pos = pos.clone();
            new_pos.play_unchecked(&m);
            best_move_value = max(
                best_move_value,
                minimax_recurse(&new_pos, depth - 1, alpha, beta, num_nodes)
            );
            alpha = max(alpha, best_move_value);
            if beta <= alpha {
                return best_move_value;
            }
        }
    }

    return best_move_value;
}

