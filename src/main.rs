use std::time::Instant;
use std::{collections::HashMap, io::stdin};
struct Rook {
    position: (i8, i8),
    color: String,
}
struct Bishop {
    position: (i8, i8),
    color: String,
    square_color: String,
}

struct Knight {
    position: (i8, i8),
    color: String,
}

struct Queen {
    position: (i8, i8),
    color: String,
}

struct Pawn {
    position: (i8, i8),
    color: String,
}

struct King {
    position: (i8, i8),
    color: String,
}
enum Pieces {
    Rook(Rook),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Bishop(Bishop),
    King(King),
}
fn general_piece_moves(
    color: String,
    first_index: i8,
    secound_index: i8,
    board: [[i8; 8]; 8],
    legal_moves: &mut Vec<(i8, i8)>,
    terminating: bool,
    increment_list: Vec<(i8, i8)>,
) -> Vec<(i8, i8)> {
    for value in increment_list.iter() {
        let first_index_increment: i8 = value.0;
        let secound_index_increment: i8 = value.1;
        let mut first_index_clone = first_index;
        let mut secound_index_clone = secound_index;
        loop {
            if first_index_increment < 0 && first_index_clone == 0 {
                break;
            }
            if secound_index_increment < 0 && secound_index_clone == 0 {
                break;
            }
            if first_index_increment > 0 && first_index_clone == 7 {
                break;
            }
            if secound_index_increment > 0 && secound_index_clone == 7 {
                break;
            }
            first_index_clone += first_index_increment;
            secound_index_clone += secound_index_increment;
            if board[first_index_clone as usize][secound_index_clone as usize] == 0 {
                legal_moves.push((first_index_clone, secound_index_clone));
            } else if board[first_index_clone as usize][secound_index_clone as usize] < 0 {
                if color == "white" {
                    legal_moves.push((first_index_clone, secound_index_clone));
                }
                break;
            } else {
                if color == "black" {
                    legal_moves.push((first_index_clone, secound_index_clone));
                }
                break;
            }
            if terminating {
                break;
            }
        }
    }
    return legal_moves.to_vec();
}
//generate possible moves not accounting for complex rules such as checks or castling/pins
impl Rook {
    fn rook_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];

        general_piece_moves(
            self.color.clone(),
            first_index,
            secound_index,
            board,
            &mut legal_moves,
            false,
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        );
        return legal_moves;
    }
}

impl Bishop {
    fn bishop_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];
        general_piece_moves(
            self.color.clone(),
            first_index,
            secound_index,
            board,
            &mut legal_moves,
            false,
            vec![(1, 1), (-1, -1), (1, -1), (-1, 1)],
        );
        return legal_moves;
    }
}
impl Knight {
    fn knight_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];

        let move_list: Vec<(i8, i8)> = vec![
            (2, -1),
            (2, 1),
            (-2, -1),
            (-2, 1),
            (1, -2),
            (1, 2),
            (-1, -2),
            (-1, 2),
        ];
        for piece_move in move_list {
            let first_index_increment = piece_move.0;
            let secound_index_increment = piece_move.1;
            if first_index + first_index_increment <= 7
                && first_index + first_index_increment >= 0
                && secound_index + secound_index_increment >= 0
                && secound_index + secound_index_increment <= 7
            {
                if board[(first_index + first_index_increment) as usize]
                    [(secound_index + secound_index_increment) as usize]
                    == 0
                {
                    legal_moves.push((
                        first_index + first_index_increment,
                        secound_index + secound_index_increment,
                    ));
                } else if board[(first_index + first_index_increment) as usize]
                    [(secound_index + secound_index_increment) as usize]
                    < 0
                {
                    if self.color == "white" {
                        legal_moves.push((
                            first_index + first_index_increment,
                            secound_index + secound_index_increment,
                        ));
                    }
                } else {
                    if self.color == "black" {
                        legal_moves.push((
                            first_index + first_index_increment,
                            secound_index + secound_index_increment,
                        ));
                    }
                }
            }
        }
        return legal_moves;
    }
}

impl Queen {
    fn queen_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];
        general_piece_moves(
            self.color.clone(),
            first_index,
            secound_index,
            board,
            &mut legal_moves,
            false,
            vec![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ],
        );
        return legal_moves;
    }
}

impl Pawn {
    fn pawn_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];
        if self.color == "white" {
            if first_index + 2 <= 7 && first_index == 1 {
                if board[first_index as usize + 2][secound_index as usize] == 0
                    && board[first_index as usize + 1][secound_index as usize] == 0
                {
                    legal_moves.push((first_index + 2, secound_index));
                    legal_moves.push((first_index + 1, secound_index));
                }
            } else if first_index + 1 <= 7 {
                if board[first_index as usize + 1][secound_index as usize] == 0 {
                    legal_moves.push((first_index + 1, secound_index));
                }
            }
            if first_index + 1 <= 7 && secound_index - 1 >= 0 {
                if board[first_index as usize + 1][secound_index as usize - 1] < 0 {
                    legal_moves.push((first_index + 1, secound_index - 1));
                }
            }

            if first_index + 1 <= 7 && secound_index + 1 <= 7 {
                if board[first_index as usize + 1][secound_index as usize + 1] < 0 {
                    legal_moves.push((first_index + 1, secound_index + 1));
                }
            }
        } else if self.color == "black" {
            if first_index - 2 >= 0 && first_index == 6 {
                if board[first_index as usize - 2][secound_index as usize] == 0
                    && board[first_index as usize - 1][secound_index as usize] == 0
                {
                    legal_moves.push((first_index - 2, secound_index));
                    legal_moves.push((first_index - 1, secound_index));
                }
            } else if first_index - 1 >= 0 {
                if board[first_index as usize - 1][secound_index as usize] == 0 {
                    legal_moves.push((first_index - 1, secound_index));
                }
            }
            if first_index - 1 >= 0 && secound_index - 1 >= 0 {
                if board[first_index as usize - 1][secound_index as usize - 1] > 0 {
                    legal_moves.push((first_index + 1, secound_index - 1));
                }
            }
            if first_index - 1 >= 0 && secound_index + 1 <= 7 {
                if board[first_index as usize - 1][secound_index as usize + 1] < 0 {
                    legal_moves.push((first_index - 1, secound_index + 1));
                }
            }
        }
        return legal_moves;
    }
    fn pawn_attacks(&self) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut attack_moves: Vec<(i8, i8)> = vec![];
        if self.color == "white" {
            if first_index + 1 < 7 && secound_index - 1 >= 0 {
                attack_moves.push((first_index + 1, secound_index - 1));
            }
            if first_index + 1 < 7 && secound_index + 1 <= 7 {
                attack_moves.push((first_index + 1, secound_index + 1));
            }
        } else if self.color == "black" {
            if first_index - 1 > 0 && secound_index - 1 >= 0 {
                attack_moves.push((first_index - 1, secound_index - 1));
            }
            if first_index - 1 > 0 && secound_index + 1 <= 7 {
                attack_moves.push((first_index - 1, secound_index + 1));
            }
        }

        return attack_moves;
    }
}
impl King {
    fn king_moves(&self, board: [[i8; 8]; 8]) -> Vec<(i8, i8)> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<(i8, i8)> = vec![];

        general_piece_moves(
            self.color.clone(),
            first_index,
            secound_index,
            board,
            &mut legal_moves,
            true,
            vec![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ],
        );
        return legal_moves;
    }
}

fn prune_ilegal_moves(
    legal_moves: &mut Vec<((i8, i8), (i8, i8))>,
    board: [[i8; 8]; 8],
    is_white: bool,
    white_moves: &mut Vec<((i8, i8), (i8, i8))>,
    black_moves: &mut Vec<((i8, i8), (i8, i8))>,
) -> (Vec<((i8, i8), (i8, i8))>, Vec<((i8, i8), (i8, i8))>) {
    let start_of_pruning_moves = Instant::now();
    let mut legal_move_counter = 0;
    let color: &str;
    if is_white {
        color = "white";
    } else {
        color = "black";
    }
    for value in legal_moves.iter() {
        let mut board_state = board;
        let first_value = value.0 .0;
        let secound_value = value.0 .1;
        let third_value = value.1 .0;
        let fourth_value = value.1 .1;
        let starting_position = board_state[first_value as usize][secound_value as usize];
        board_state[third_value as usize][fourth_value as usize] = starting_position;
        board_state[first_value as usize][secound_value as usize] = 0;

        let moves_and_attacks = moves_and_attacks_from_board_state(board_state);
        let white_squares_attacked = moves_and_attacks.2;
        let black_squares_attacked = moves_and_attacks.4;
        let white_king_position = moves_and_attacks.5;
        let black_king_position = moves_and_attacks.6;
        if color == "white" {
            if black_squares_attacked.contains(&white_king_position) {
                let move_to_remove = legal_moves.get(legal_move_counter).unwrap();
                white_moves.retain(|x| *x != *move_to_remove);
            }
        } else {
            if white_squares_attacked.contains(&black_king_position) {
                let move_to_remove = legal_moves.get(legal_move_counter).unwrap();
                black_moves.retain(|x| *x != *move_to_remove);
            }
        }
        legal_move_counter += 1;
    }
    println!("there have been {} microsecounds in pruning illigal moves", start_of_pruning_moves.elapsed().as_micros());
    return (white_moves.to_vec(), black_moves.to_vec());
}

fn un_passant(
    board: [[i8; 8]; 8],
    last_move: ((i8, i8), (i8, i8)),
    black_moves: &mut Vec<((i8, i8), (i8, i8))>,
    white_moves: &mut Vec<((i8, i8), (i8, i8))>,
) -> (
    Vec<((i8, i8), (i8, i8))>,
    Vec<((i8, i8), (i8, i8))>,
    ((i8, i8), (i8, i8)),
) {
    let mut unpassant_move: ((i8, i8), (i8, i8)) = ((10, 10), (10, 10));
    if last_move != ((10, 10), (10, 10)) {
        let first_value = last_move.0 .0;
        let secound_value = last_move.0 .1;
        let third_value = last_move.1 .0;
        let fourth_value = last_move.1 .1;
        if third_value - 1 >= 0 {
            if board[third_value as usize][fourth_value as usize] == 1 {
                if third_value - first_value == 2 {
                    println!("white moved up two squares");
                    if fourth_value - 1 >= 0 {
                        if board[third_value as usize][(fourth_value - 1) as usize] == -1 {
                            if board[(third_value - 1) as usize][fourth_value as usize] == 0 {
                                unpassant_move = (
                                    (third_value, fourth_value - 1),
                                    (third_value - 1, fourth_value),
                                );
                                black_moves.push(unpassant_move.clone())
                            }
                        } 
                    }
                    if !fourth_value + 1 <= 7{
                        if board[third_value as usize][(fourth_value + 1) as usize] == -1 {
                            if board[(third_value - 1) as usize][fourth_value as usize] == 0 {
                                let unpassant_move = (
                                    (third_value, fourth_value + 1),
                                    (third_value - 1, fourth_value),
                                );
                                black_moves.push(unpassant_move.clone())
                            }
                        }
                    }
                }
            } 
        }
        if third_value + 1 <= 7 {
            if board[third_value as usize][fourth_value as usize] == -1 {
                if third_value - first_value == -2 {
                    println!("black moved up two squares");

                    if fourth_value - 1 >= 0 {
                        if board[third_value as usize][(fourth_value - 1) as usize] == 1 {
                            if board[(third_value + 1) as usize][fourth_value as usize] == 0 {
                                unpassant_move = (
                                    (third_value, fourth_value - 1),
                                    (third_value + 1, fourth_value),
                                );
                                white_moves.push(unpassant_move.clone())
                            }
                        }
                    }
                    if fourth_value + 1 <= 7{
                        if board[third_value as usize][(fourth_value + 1) as usize] == -1 {
                            if board[(third_value - 1) as usize][fourth_value as usize] == 0 {
                                unpassant_move = (
                                    (third_value, fourth_value - 1),
                                    (third_value + 1, fourth_value),
                                );
                                white_moves.push(unpassant_move.clone())
                            }
                        }
                    }
                        
                }
            }
        }
    }
    return (white_moves.to_vec(), black_moves.to_vec(), unpassant_move);
}
fn moves_and_attacks_from_board_state(
    board_state: [[i8; 8]; 8],
) -> (
    Vec<((i8, i8), (i8, i8))>,
    Vec<((i8, i8), (i8, i8))>,
    Vec<(i8, i8)>,
    Vec<((i8, i8), (i8, i8))>,
    Vec<(i8, i8)>,
    (i8, i8),
    (i8, i8),
    Vec<(i8, i8)>,
    Vec<Pieces>,
) {
    let mut board_state_vector: Vec<Pieces> = vec![];
    let mut first_index_counter: i8 = 0;
    let mut pawn_positions: Vec<(i8, i8)> = vec![];
    for value in board_state.iter() {
        let mut secound_index_counter: i8 = 0;
        let mut struct_color: &str = "";
        for value in value.iter() {
            if *value < 0 {
                struct_color = "black";
            } else if *value > 0 {
                struct_color = "white";
            }
            if *value == 1 || *value == -1 {
                board_state_vector.push(Pieces::Pawn(Pawn {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            } else if *value == 2 || *value == -2 {
                board_state_vector.push(Pieces::Knight(Knight {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            } else if *value == 3 || *value == -3 {
                let square_color: &str;
                if (first_index_counter + 1 * secound_index_counter + 1) % 2 == 1 {
                    square_color = "black";
                } else {
                    square_color = "white"
                }
                board_state_vector.push(Pieces::Bishop(Bishop {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                    square_color: String::from(square_color),
                }));
            } else if *value == 4 || *value == -4 {
                board_state_vector.push(Pieces::Rook(Rook {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            } else if *value == 5 || *value == -5 {
                board_state_vector.push(Pieces::Queen(Queen {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            } else if *value == 6 || *value == -6 {
                board_state_vector.push(Pieces::King(King {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            }
            secound_index_counter += 1;
        }
        first_index_counter += 1
    }
    let mut total_moves: Vec<((i8, i8), (i8, i8))> = vec![];
    let mut white_piece_moves: Vec<((i8, i8), (i8, i8))> = vec![];
    let mut black_piece_moves: Vec<((i8, i8), (i8, i8))> = vec![];
    let mut black_squares_attacked: Vec<(i8, i8)> = vec![];
    let mut white_squares_attacked: Vec<(i8, i8)> = vec![];
    let mut white_king_position: (i8, i8) = (0, 0);
    let mut black_king_position: (i8, i8) = (0, 0);
    for value in board_state_vector.iter() {
        let piece_color: &str;
        let piece_moves: Vec<(i8, i8)>;
        let piece_attacks: Vec<(i8, i8)>;
        let piece_position: (i8, i8);
        match value {
            Pieces::Rook(rook) => {
                piece_color = &rook.color;
                piece_position = rook.position;
                let rook_moves = rook.rook_moves(board_state);
                piece_moves = rook_moves.clone();
                piece_attacks = rook_moves;
            }
            Pieces::Knight(knight) => {
                piece_color = &knight.color;
                piece_position = knight.position;
                let knight_moves = knight.knight_moves(board_state);
                piece_moves = knight_moves.clone();
                piece_attacks = knight_moves;
            }
            Pieces::Pawn(pawn) => {
                piece_color = &pawn.color;
                piece_position = pawn.position;
                piece_moves = pawn.pawn_moves(board_state);
                piece_attacks = pawn.pawn_attacks();
                pawn_positions.push(piece_position)
            }
            Pieces::Queen(queen) => {
                piece_color = &queen.color;
                piece_position = queen.position;
                let queen_moves = queen.queen_moves(board_state);
                piece_moves = queen_moves.clone();
                piece_attacks = queen_moves
            }
            Pieces::Bishop(bishop) => {
                piece_color = &bishop.color;
                piece_position = bishop.position;
                let bishop_moves = bishop.bishop_moves(board_state);
                piece_moves = bishop_moves.clone();
                piece_attacks = bishop_moves
            }
            Pieces::King(king) => {
                piece_color = &king.color;
                piece_position = king.position;
                let king_move = king.king_moves(board_state);
                piece_moves = king_move.clone();
                piece_attacks = king_move;
                if piece_color == "white" {
                    white_king_position = (piece_position.0, piece_position.1)
                } else {
                    black_king_position = (piece_position.0, piece_position.1)
                }
            }
        }
        for value in piece_moves.iter() {
            let parsed_move = (piece_position, *value);
            let parsed_move_clone = parsed_move.clone();
            total_moves.push(parsed_move);
            if piece_color == "white" {
                white_piece_moves.push(parsed_move_clone.clone());
            } else {
                black_piece_moves.push(parsed_move_clone.clone());
            }
        }
        for value in piece_attacks.iter() {
            let parsed_move = (piece_position, *value);
            if piece_color == "white" {
                white_squares_attacked.push(parsed_move.1)
            } else {
                black_squares_attacked.push(parsed_move.1)
            }
        }
    }
    return (
        total_moves,
        white_piece_moves,
        white_squares_attacked,
        black_piece_moves,
        black_squares_attacked,
        white_king_position,
        black_king_position,
        pawn_positions,
        board_state_vector,
    );
}
fn main() {
    // 0 = empty, 1 = pawn, 2 = knight, 3 = pawn, 4 = rook,
    // 5 = queen, 6 = king, positive equals white negative = black
    let mut alphabet_hash = HashMap::new();
    alphabet_hash.insert(String::from("h"), 0);
    alphabet_hash.insert(String::from("g"), 1);
    alphabet_hash.insert(String::from("f"), 2);
    alphabet_hash.insert(String::from("e"), 3);
    alphabet_hash.insert(String::from("d"), 4);
    alphabet_hash.insert(String::from("c"), 5);
    alphabet_hash.insert(String::from("b"), 6);
    alphabet_hash.insert(String::from("a"), 7);

    let rank_one: [i8; 8] = [4, 2, 3, 6, 5, 3, 2, 4];
    let rank_two: [i8; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
    let rank_three: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_four: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_five: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_six: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_seven: [i8; 8] = [-1, -1, -1, -1, -1, -1, -1, -1];
    let rank_eight: [i8; 8] = [-4, -2, -3, -6, -5, -3, -2, -4];
    let mut board_state: [[i8; 8]; 8] = [
        rank_one, rank_two, rank_three, rank_four, rank_five, rank_six, rank_seven, rank_eight,
    ];
    let mut list_of_position: Vec<[[i8; 8]; 8]> = vec![];
    for value in board_state.iter() {
        let array_iterator = value.iter();
        for value in array_iterator {
            print!("{}", value)
        }
        println!()
    }

    let mut game_has_ended = false;
    let mut is_white_to_move = true;
    let mut result = "";
    let mut fifty_move_counter: f32 = 0.0;
    let mut last_move: ((i8, i8), (i8, i8)) = ((10, 10), (10, 10));
    let mut custom_move: String;
    let mut white_can_caste_kingside = true;
    let mut white_can_caste_queenside = true;
    let mut black_can_caste_kingside = true;
    let mut black_can_caste_queenside = true;
    let mut white_has_castled = false;
    let mut black_has_castled = false;
    while game_has_ended == false {
        let now = Instant::now();
        if !white_has_castled {
            if board_state[0][0] != 4 {
                white_can_caste_kingside = false;
            } else if board_state[0][7] != 4 {
                white_can_caste_queenside = false
            } else if board_state[0][3] != 6 {
                white_can_caste_kingside = false;
                white_can_caste_queenside = false;
            }
        }
        if !black_has_castled {
            if board_state[7][0] != -4 {
                black_can_caste_kingside = false;
            } else if board_state[7][7] != -4 {
                black_can_caste_queenside = false
            } else if board_state[7][3] != -6 {
                black_can_caste_kingside = false;
                black_can_caste_queenside = false;
            }
        }
        let moves_and_attacks = moves_and_attacks_from_board_state(board_state);
        let mut total_moves = moves_and_attacks.0;
        let mut white_piece_moves = moves_and_attacks.1;
        let white_piece_attacks = moves_and_attacks.2;
        let mut black_piece_moves = moves_and_attacks.3;
        let black_piece_attacks = moves_and_attacks.4;
        println!("there are {} total moves", (total_moves.iter().len()));
        let white_and_black_moves = un_passant(
            board_state,
            last_move,
            &mut black_piece_moves,
            &mut white_piece_moves,
        );
        let unpassant_move = white_and_black_moves.2;
        let mut white_piece_moves = white_and_black_moves.0;
        let mut black_piece_moves = white_and_black_moves.1;
        let parsed_piece_moves = prune_ilegal_moves(
            &mut total_moves,
            board_state,
            is_white_to_move,
            &mut white_piece_moves,
            &mut black_piece_moves,
        );
        let mut white_piece_moves = parsed_piece_moves.0;
        let mut black_piece_moves = parsed_piece_moves.1;
        if !(black_piece_attacks.contains(&(0, 1))
            || black_piece_attacks.contains(&(0, 2))
            || black_piece_attacks.contains(&(0, 3)))
            && white_can_caste_kingside
            && board_state[0][2] == 0
            && board_state[0][1] == 0
        {
            white_piece_moves.push(((0, 3), (0, 0)));
        }
        if !(black_piece_attacks.contains(&(0, 5))
            || black_piece_attacks.contains(&(0, 6))
            || black_piece_attacks.contains(&(0, 3)))
            && white_can_caste_queenside
            && board_state[0][6] == 0
            && board_state[0][5] == 0
        {
            white_piece_moves.push(((0, 3), (0, 7)));
        }
        if !(white_piece_attacks.contains(&(7, 1))
            || white_piece_attacks.contains(&(7, 2))
            || white_piece_attacks.contains(&(7, 3)))
            && black_can_caste_kingside
            && board_state[7][2] == 0
            && board_state[7][1] == 0
        {
            black_piece_moves.push(((7, 3), (7, 0)));
        }
        if !(white_piece_attacks.contains(&(7, 5))
            || white_piece_attacks.contains(&(7, 4))
            || white_piece_attacks.contains(&(7, 3)))
            && (black_can_caste_queenside && board_state[7][4] == 0 && board_state[7][5] == 0)
        {
            white_piece_moves.push(((7, 3), (7, 7)));
        }
        println!("time since start {}", now.elapsed().as_micros());
        loop {
            custom_move = String::new();
            stdin().read_line(&mut custom_move).unwrap();
            custom_move = custom_move.trim().to_string();
            let first_value = custom_move.chars().nth(0).unwrap().to_string();
            let first_value: &i32 = alphabet_hash.get(&first_value).unwrap();
            let secound_value = custom_move.chars().nth(1).unwrap().to_string();
            let mut secound_value: i8 = secound_value.parse().unwrap();
            secound_value -= 1;
            let third_value = custom_move.chars().nth(3).unwrap().to_string();
            let third_value: &i32 = alphabet_hash.get(&third_value).unwrap();
            let fourth_value = custom_move.chars().nth(4).unwrap().to_string();
            let mut fourth_value: i8 = fourth_value.parse().unwrap();
            fourth_value -= 1;
            let custom_move: ((i8, i8), (i8, i8)) = (
                (secound_value as i8, *first_value as i8),
                (fourth_value as i8, *third_value as i8),
            );
            if is_white_to_move {
                if white_piece_moves.contains(&custom_move) {
                    last_move = custom_move;
                    if custom_move == ((0, 3), (0, 0)) {
                        white_has_castled = true;
                        last_move = ((10, 10), (10, 10));
                        board_state[0][3] = 0;
                        board_state[0][0] = 0;
                        board_state[0][1] = 6;
                        board_state[0][2] = 4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                    } else if custom_move == ((0, 3), (0, 7)) {
                        last_move = ((10, 10), (10, 10));
                        white_has_castled = true;
                        board_state[0][3] = 0;
                        board_state[0][7] = 0;
                        board_state[0][5] = 6;
                        board_state[0][4] = 4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                    } else {
                        let starting_position =
                            board_state[secound_value as usize][*first_value as usize];
                        let pawn_positions = moves_and_attacks.7;
                        if board_state[fourth_value as usize][*third_value as usize] < 0
                            || pawn_positions.contains(&custom_move.0)
                        {
                            fifty_move_counter = 0.0
                        } else {
                            fifty_move_counter += 0.5;
                        }
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                        board_state[fourth_value as usize][*third_value as usize] =
                            starting_position;
                        board_state[secound_value as usize][*first_value as usize] = 0;
                        if custom_move == unpassant_move {
                            board_state[(fourth_value - 1) as usize][*third_value as usize] = 0
                        }
                        if fourth_value == 7 {
                            if board_state[fourth_value as usize][*third_value as usize] == 1 {
                                loop {
                                    let mut promotion_piece = String::new();
                                    stdin().read_line(&mut promotion_piece).unwrap();
                                    promotion_piece = promotion_piece.trim().to_string();
                                    if promotion_piece == "rook" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            4;
                                        break;
                                    } else if promotion_piece == "bishop" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            3;
                                        break;
                                    } else if promotion_piece == "knight" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            2;
                                        break;
                                    } else if promotion_piece == "queen" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            5;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if is_white_to_move == true {
                        is_white_to_move = false;
                    } else {
                        is_white_to_move = true;
                    }
                    for value in board_state.iter() {
                        let array_iterator = value.iter();
                        for value in array_iterator {
                            print!("{}", value)
                        }
                        println!()
                    }
                    let number_of_repetitions = list_of_position
                        .iter()
                        .filter(|&n| *n == board_state)
                        .count();
                    if number_of_repetitions == 3 {
                        game_has_ended = true;
                        result = "draw by repetition"
                    }
                    let moves_and_attacks = moves_and_attacks_from_board_state(board_state);
                    let mut white_piece_moves = moves_and_attacks.1;
                    let white_squares_attacked = moves_and_attacks.2;
                    let mut black_piece_moves = moves_and_attacks.3;
                    let black_king_position = moves_and_attacks.6;
                    let mut total_moves = moves_and_attacks.0;
                    let list_of_pieces = moves_and_attacks.8;
                    let parsed_piece_moves = prune_ilegal_moves(
                        &mut total_moves,
                        board_state,
                        is_white_to_move,
                        &mut white_piece_moves,
                        &mut black_piece_moves,
                    );
                    let black_piece_moves = parsed_piece_moves.1;
                    let mut number_of_knights = 0;
                    let mut number_of_black_bishops = 0;
                    let mut number_of_white_bishops = 0;
                    let mut number_of_bishops = 0;
                    if list_of_pieces.len() <= 4 {
                        for value in list_of_pieces.iter() {
                            match value {
                                Pieces::Knight(_) => number_of_knights += 1,
                                Pieces::Bishop(bishop) => {
                                    number_of_bishops += 1;
                                    if bishop.square_color == "white" {
                                        number_of_white_bishops += 1
                                    } else {
                                        number_of_black_bishops += 1
                                    }
                                }
                                _ => println!(),
                            }
                        }
                        if number_of_bishops + number_of_knights + 2 == list_of_pieces.len() {
                            if number_of_bishops + number_of_knights <= 2 {
                                if number_of_bishops == 1 && number_of_knights == 0 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                } else if number_of_knights == 1 && number_of_bishops == 0 {
                                    result = "draw by insuffience material";
                                    game_has_ended = true;
                                } else if number_of_bishops + number_of_knights == 0 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                } else if number_of_white_bishops == 2 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true
                                } else if number_of_black_bishops == 2 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                }
                            }
                        }
                    }
                    if black_piece_moves.len() == 0 {
                        if white_squares_attacked.contains(&black_king_position) {
                            result = "white wins by checkmate";
                            game_has_ended = true;
                        } else {
                            result = "draw by stalemate";
                            game_has_ended = true;
                        }
                    }
                    break;
                } else {
                    println!("invalid move please try again");
                }
            } else {
                if black_piece_moves.contains(&custom_move) {
                    last_move = custom_move;
                    if custom_move == ((7, 3), (7, 0)) {
                        last_move = ((10, 10), (10, 10));
                        black_has_castled = true;
                        board_state[7][3] = 0;
                        board_state[7][0] = 0;
                        board_state[7][1] = -6;
                        board_state[7][2] = -4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                    } else if custom_move == ((7, 3), (7, 7)) {
                        black_has_castled = true;
                        last_move = ((10, 10), (10, 10));
                        board_state[7][3] = 0;
                        board_state[7][7] = 0;
                        board_state[7][5] = -6;
                        board_state[7][4] = -4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                    } else {
                        let starting_position =
                            board_state[secound_value as usize][*first_value as usize];
                        let pawn_positions = moves_and_attacks.7;
                        if board_state[fourth_value as usize][*third_value as usize] > 0
                            || pawn_positions.contains(&custom_move.0)
                        {
                            fifty_move_counter = 0.0
                        } else {
                            fifty_move_counter += 0.5;
                        }
                        if fifty_move_counter == 50.0 {
                            result = "draw by fifty move rule";
                            game_has_ended = true
                        }
                        board_state[fourth_value as usize][*third_value as usize] =
                            starting_position;
                        board_state[secound_value as usize][*first_value as usize] = 0;
                        if custom_move == unpassant_move {
                            board_state[(fourth_value + 1) as usize][*third_value as usize] = 0
                        }
                        if fourth_value == 0 {
                            if board_state[fourth_value as usize][*third_value as usize] == -1 {
                                loop {
                                    let mut promotion_piece = String::new();
                                    stdin().read_line(&mut promotion_piece).unwrap();
                                    promotion_piece = promotion_piece.trim().to_string();
                                    if promotion_piece == "rook" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            -4;
                                        break;
                                    } else if promotion_piece == "bishop" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            -3;
                                        break;
                                    } else if promotion_piece == "knight" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            -2;
                                        break;
                                    } else if promotion_piece == "queen" {
                                        board_state[fourth_value as usize][*third_value as usize] =
                                            -5;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if is_white_to_move == true {
                        is_white_to_move = false;
                    } else {
                        is_white_to_move = true;
                    }
                    for value in board_state.iter() {
                        let array_iterator = value.iter();
                        for value in array_iterator {
                            print!("{}", value)
                        }
                        println!()
                    }
                    list_of_position.push(board_state);
                    let number_of_repetitions = list_of_position
                        .iter()
                        .filter(|&n| *n == board_state)
                        .count();
                    if number_of_repetitions == 3 {
                        game_has_ended = true;
                        result = "draw by repetition"
                    }
                    let moves_and_attacks = moves_and_attacks_from_board_state(board_state);
                    let mut white_piece_moves = moves_and_attacks.1;
                    let mut black_piece_moves = moves_and_attacks.3;
                    let black_squares_attacked = moves_and_attacks.4;
                    let white_king_position = moves_and_attacks.5;
                    let mut total_moves = moves_and_attacks.0;
                    let parsed_piece_moves = prune_ilegal_moves(
                        &mut total_moves,
                        board_state,
                        is_white_to_move,
                        &mut white_piece_moves,
                        &mut black_piece_moves,
                    );
                    let white_piece_moves = parsed_piece_moves.0;
                    let list_of_pieces = moves_and_attacks.8;
                    let mut number_of_knights = 0;
                    let mut number_of_black_bishops = 0;
                    let mut number_of_white_bishops = 0;
                    let mut number_of_bishops = 0;
                    if list_of_pieces.len() <= 4 {
                        for value in list_of_pieces.iter() {
                            match value {
                                Pieces::Knight(_) => number_of_knights += 1,
                                Pieces::Bishop(bishop) => {
                                    number_of_bishops += 1;
                                    if bishop.square_color == "white" {
                                        number_of_white_bishops += 1
                                    } else {
                                        number_of_black_bishops += 1
                                    }
                                }
                                _ => println!(),
                            }
                        }
                        if number_of_bishops + number_of_knights + 2 == list_of_pieces.len() {
                            if number_of_bishops + number_of_knights <= 2 {
                                if number_of_bishops == 1 && number_of_knights == 0 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                } else if number_of_knights == 1 && number_of_bishops == 0 {
                                    result = "draw by insuffience material";
                                    game_has_ended = true;
                                } else if number_of_bishops + number_of_knights == 0 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                } else if number_of_white_bishops == 2 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true
                                } else if number_of_black_bishops == 2 {
                                    result = "draw by insufficent material";
                                    game_has_ended = true;
                                }
                            }
                        }
                    }
                    if white_piece_moves.len() == 0 {
                        if black_squares_attacked.contains(&white_king_position) {
                            result = "black wins by checkmate";
                            game_has_ended = true;
                        } else {
                            result = "draw by stalemate";
                            game_has_ended = true;
                        }
                    }
                    break;
                } else {
                    println!("invalid move please try again")
                }
            }
        }
        println!("{}", result);
    }
}