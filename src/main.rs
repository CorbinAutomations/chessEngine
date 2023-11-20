use std::{any::type_name, collections::HashMap, io::stdin};
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
    legal_moves: &mut Vec<String>,
    terminating: bool,
    increment_list: Vec<(i8, i8)>,
) -> Vec<String> {
    for value in increment_list.iter() {
        let first_index_increment: i8 = value.0;
        let secound_index_increment: i8 = value.1;
        let mut first_index_clone = first_index.clone();
        let mut secound_index_clone = secound_index.clone();
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
                legal_moves
                    .push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
            } else if board[first_index_clone as usize][secound_index_clone as usize] < 0 {
                if color == "white" {
                    legal_moves.push(
                        first_index_clone.to_string() + "," + &secound_index_clone.to_string(),
                    );
                }
                break;
            } else {
                if color == "black" {
                    legal_moves.push(
                        first_index_clone.to_string() + "," + &secound_index_clone.to_string(),
                    );
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
    fn rook_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];

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
    fn bishop_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];
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
    fn knight_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];

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
                    legal_moves.push(
                        (first_index + first_index_increment).to_string()
                            + ","
                            + &(secound_index + secound_index_increment).to_string(),
                    );
                } else if board[(first_index + first_index_increment) as usize]
                    [(secound_index + secound_index_increment) as usize]
                    < 0
                {
                    if self.color == "white" {
                        legal_moves.push(
                            (first_index + first_index_increment).to_string()
                                + ","
                                + &(secound_index + secound_index_increment).to_string(),
                        );
                    }
                } else {
                    if self.color == "black" {
                        legal_moves.push(
                            (first_index + first_index_increment).to_string()
                                + ","
                                + &(secound_index + secound_index_increment).to_string(),
                        );
                    }
                }
            }
        }
        return legal_moves;
    }
}

impl Queen {
    fn queen_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];
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
    fn pawn_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];
        if self.color == "white" {
            if first_index + 1 <= 7 {
                if board[first_index as usize + 1][secound_index as usize] == 0 {
                    legal_moves
                        .push((first_index + 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 2 <= 7 && first_index == 1 {
                if board[first_index as usize + 2][secound_index as usize] == 0
                    && board[first_index as usize + 1][secound_index as usize] == 0
                {
                    legal_moves
                        .push((first_index + 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 1 <= 7 && secound_index - 1 >= 0 {
                if board[first_index as usize + 1][secound_index as usize - 1] < 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index - 1).to_string(),
                    );
                }
            }

            if first_index + 1 <= 7 && secound_index + 1 <= 7 {
                if board[first_index as usize + 1][secound_index as usize + 1] < 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index + 1).to_string(),
                    );
                }
            }
        }

        if self.color == "black" {
            if first_index - 1 >= 0 {
                if board[first_index as usize - 1][secound_index as usize] == 0 {
                    legal_moves
                        .push((first_index - 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index - 2 >= 0 && first_index == 6 {
                if board[first_index as usize - 2][secound_index as usize] == 0
                    && board[first_index as usize - 1][secound_index as usize] == 0
                {
                    legal_moves
                        .push((first_index - 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index - 1 >= 0 && secound_index - 1 >= 0 {
                if board[first_index as usize - 1][secound_index as usize - 1] > 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index - 1).to_string(),
                    );
                }
            }
            if first_index - 1 >= 0 && secound_index + 1 <= 7 {
                if board[first_index as usize - 1][secound_index as usize + 1] < 0 {
                    legal_moves.push(
                        (first_index - 1).to_string() + "," + &(secound_index + 1).to_string(),
                    );
                }
            }
        }
        return legal_moves;
    }
    fn pawn_attacks(&self) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut attack_moves: Vec<String> = vec![];
        if self.color == "white" {
            if first_index + 1 < 7 && secound_index - 1 >= 0 {
                attack_moves
                    .push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
            }

            if first_index + 1 < 7 && secound_index + 1 <= 7 {
                attack_moves
                    .push((first_index + 1).to_string() + "," + &(secound_index + 1).to_string());
            }
        }
        if self.color == "black" {
            if first_index - 1 > 0 && secound_index - 1 >= 0 {
                attack_moves
                    .push((first_index - 1).to_string() + "," + &(secound_index - 1).to_string());
            }
            if first_index - 1 > 0 && secound_index + 1 <= 7 {
                attack_moves
                    .push((first_index - 1).to_string() + "," + &(secound_index + 1).to_string());
            }
        }

        return attack_moves;
    }
}
impl King {
    fn king_moves(&self, board: [[i8; 8]; 8]) -> Vec<String> {
        let first_index = self.position.0;
        let secound_index = self.position.1;
        let mut legal_moves: Vec<String> = vec![];

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
    legal_moves: &mut Vec<String>,
    board: [[i8; 8]; 8],
    int_to_alphabet: &HashMap<String, String>,
    alphabet_hash: &HashMap<String, i32>,
    is_white: bool,
    white_moves: &mut Vec<String>,
    black_moves: &mut Vec<String>,
) -> (Vec<String>, Vec<String>) {
    let mut legal_move_counter = 0;
    let mut color = "";
    if is_white {
        color = "white";
    } else {
        color = "black";
    }
    for Move in legal_moves.iter() {
        let mut board_state = board.clone();
        let first_value = Move.chars().nth(0).unwrap().to_string();
        let first_value = alphabet_hash.get(&first_value).unwrap();
        let secound_value = Move.chars().nth(1).unwrap().to_string();
        let mut secound_value: i32 = secound_value.parse().unwrap();
        secound_value -= 1;
        let third_value = Move.chars().nth(3).unwrap().to_string();
        let third_value = alphabet_hash.get(&third_value).unwrap();
        let fourth_value = Move.chars().nth(4).unwrap().to_string();
        let mut fourth_value: i32 = fourth_value.parse().unwrap();
        fourth_value -= 1;
        let starting_position = board_state[secound_value as usize][*first_value as usize];
        board_state[fourth_value as usize][*third_value as usize] = starting_position;
        board_state[secound_value as usize][*first_value as usize] = 0;

        let moves_and_attacks = moves_and_attacks_from_board_state(board_state, &int_to_alphabet);
        let white_squares_attacked = moves_and_attacks.2;
        let black_squares_attacked = moves_and_attacks.4;
        let white_king_position = moves_and_attacks.5;
        let black_king_position = moves_and_attacks.6;
        if color == "white" {
            if black_squares_attacked.contains(&white_king_position) {
                let move_to_remove = legal_moves.get(legal_move_counter).unwrap();
                let index = white_moves.iter().position(|x| *x == *move_to_remove);
                println!("your white king is on {}", white_king_position);
                if index.is_some() {
                    white_moves.remove(index.unwrap());
                    println!("we just removed something")
                }
            }
        } else {
            if white_squares_attacked.contains(&black_king_position) {
                let move_to_remove = legal_moves.get(legal_move_counter).unwrap();
                let index = black_moves.iter().position(|x| *x == *move_to_remove);
                if index.is_some() {
                    black_moves.remove(index.unwrap());
                    println!("we just removed something")
                }
            }
        }
        legal_move_counter += 1;
    }
    return (white_moves.to_vec(), black_moves.to_vec());
}

fn index_to_move(value: String, int_to_alphabet_hash: &HashMap<String, String>) -> String {
    let first_value = value.chars().nth(0).unwrap().to_string();
    let secound_value = value.chars().nth(1).unwrap().to_string();
    let third_value = value.chars().nth(2).unwrap().to_string();
    let six_value = value.chars().nth(3).unwrap().to_string();
    let alphabet_value = int_to_alphabet_hash.get(&value.chars().nth(5).unwrap().to_string());
    let mut six_value: i32 = six_value.parse().unwrap();
    six_value += 1;
    return first_value
        + &secound_value
        + &third_value
        + alphabet_value.unwrap()
        + &six_value.to_string();
}
fn un_passant(
    board: [[i8; 8]; 8],
    last_move: String,
    alphabet_hash: &HashMap<String, i32>,
    black_moves: &mut Vec<String>,
    white_moves: &mut Vec<String>,
)-> (Vec<String>, Vec<String>, String){
    let mut unpassant_move = "".to_string();

    if last_move != ""{
        let first_value = last_move.chars().nth(0).unwrap().to_string();
    let first_value = alphabet_hash.get(&first_value).unwrap();
    let secound_value = last_move.chars().nth(1).unwrap().to_string();
    let mut secound_value: i32 = secound_value.parse().unwrap();
    secound_value -= 1;
    let third_value = last_move.chars().nth(3).unwrap().to_string();
    let third_value = alphabet_hash.get(&third_value).unwrap();
    let fourth_value = last_move.chars().nth(4).unwrap().to_string();
    let mut fourth_value: i32 = fourth_value.parse().unwrap();
    fourth_value -= 1;
    if board[fourth_value as usize][*third_value as usize] == 1 {
        if fourth_value - secound_value == 2 {
            if board[fourth_value as usize][(*third_value - 1) as usize] == -1 {
                if board[(fourth_value - 1) as usize][*third_value as usize] == 0 {
                    unpassant_move = index_to_position(&(fourth_value as u32), &((third_value - 1) as u32))
                    + ":" + &index_to_position(
                        &((fourth_value - 1) as u32),
                        &(*third_value as u32),
                    );
                    black_moves.push(
                       unpassant_move.clone() 
                    )
                }
            }
            else if board[fourth_value as usize][(*third_value + 1) as usize] == -1 {
                if board[(fourth_value - 1) as usize][*third_value as usize] == 0 {
                    let unpassant_move = index_to_position(&(fourth_value as u32), &((third_value + 1) as u32))
                            + ":" + &index_to_position(
                                &((fourth_value - 1) as u32),
                                &(*third_value as u32),
                                
                            );
                    black_moves.push(
                        unpassant_move.clone()
                    )
                }
            }
        }
    }
    if board[fourth_value as usize][*third_value as usize] == -1 {
        if fourth_value - secound_value == -2 {
            if board[fourth_value as usize][(*third_value - 1) as usize] == 1 {
                if board[(fourth_value + 1) as usize][*third_value as usize] == 0 {
                    unpassant_move = index_to_position(&(fourth_value as u32), &((third_value - 1) as u32))
                    + ":" + &index_to_position(
                        &((fourth_value + 1) as u32),
                        &(*third_value as u32),
                    );
                    white_moves.push(
                        unpassant_move.clone()
                    )
                }
            }
            else if board[fourth_value as usize][(*third_value + 1) as usize] == -1 {
                if board[(fourth_value - 1) as usize][*third_value as usize] == 0 {
                    unpassant_move = index_to_position(&(fourth_value as u32), &((third_value - 1) as u32))
                    + ":" + &index_to_position(
                        &((fourth_value + 1) as u32),
                        &(*third_value as u32),
                    );
                    white_moves.push(
                        unpassant_move.clone()
                    )
                }
            }
        }
    }
    }
    return (white_moves.to_vec(), black_moves.to_vec(), unpassant_move)
}
fn moves_and_attacks_from_board_state(
    board_state: [[i8; 8]; 8],
    int_to_alphabet: &HashMap<String, String>,
) -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    Vec<String>,
    Vec<Pieces>,
) {
    let mut board_state_vector: Vec<Pieces> = vec![];
    let mut first_index_counter: i8 = 0;
    let mut pawn_positions: Vec<String> = vec![];
    for value in board_state.iter() {
        let mut secound_index_counter: i8 = 0;
        let mut struct_color = "";
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
            }
            if *value == 2 || *value == -2 {
                board_state_vector.push(Pieces::Knight(Knight {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            }
            if *value == 3 || *value == -3 {
                let mut square_color = "";
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
            }
            if *value == 4 || *value == -4 {
                board_state_vector.push(Pieces::Rook(Rook {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            }
            if *value == 5 || *value == -5 {
                board_state_vector.push(Pieces::Queen(Queen {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            }
            if *value == 6 || *value == -6 {
                board_state_vector.push(Pieces::King(King {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
                }));
            }
            secound_index_counter += 1;
        }
        first_index_counter += 1
    }
    let mut total_moves: Vec<String> = vec![];
    let mut white_piece_moves: Vec<String> = vec![];
    let mut black_piece_moves: Vec<String> = vec![];
    let mut black_squares_attacked: Vec<String> = vec![];
    let mut white_squares_attacked: Vec<String> = vec![];
    let mut white_king_position: String = "".to_string();
    let mut black_king_position: String = "".to_string();
    for value in board_state_vector.iter() {
        let mut piece_color = "";
        let mut piece_moves = vec![];
        let mut piece_attacks: Vec<String> = vec![];
        let mut piece_position: (i8, i8) = (10, 10);
        match value {
            Pieces::Rook(rook) => {
                piece_color = &rook.color;
                piece_position = rook.position;
                piece_moves = rook.rook_moves(board_state);
                piece_attacks = rook.rook_moves(board_state)
            }
            Pieces::Knight(knight) => {
                piece_color = &knight.color;
                piece_position = knight.position;
                piece_moves = knight.knight_moves(board_state);
                piece_attacks = knight.knight_moves(board_state)
            }
            Pieces::Pawn(pawn) => {
                piece_color = &pawn.color;
                piece_position = pawn.position;
                piece_moves = pawn.pawn_moves(board_state);
                piece_attacks = pawn.pawn_attacks();
                pawn_positions.push(index_to_position(
                    &(piece_position.0 as u32),
                    &(piece_position.1 as u32),
                ))
            }
            Pieces::Queen(queen) => {
                piece_color = &queen.color;
                piece_position = queen.position;
                piece_moves = queen.queen_moves(board_state);
                piece_attacks = queen.queen_moves(board_state)
            }
            Pieces::Bishop(bishop) => {
                piece_color = &bishop.color;
                piece_position = bishop.position;
                piece_moves = bishop.bishop_moves(board_state);
                piece_attacks = bishop.bishop_moves(board_state)
            }
            Pieces::King(king) => {
                piece_color = &king.color;
                piece_position = king.position;
                piece_moves = king.king_moves(board_state);
                piece_attacks = king.king_moves(board_state);
                if piece_color == "white" {
                    white_king_position =
                        index_to_position(&(piece_position.0 as u32), &(piece_position.1 as u32));
                } else {
                    black_king_position =
                        index_to_position(&(piece_position.0 as u32), &(piece_position.1 as u32));
                }
            }
        }
        for value in piece_moves.iter() {
            let parsed_move = index_to_move(
                String::from(index_to_position(
                    &(piece_position.0 as u32),
                    &(piece_position.1 as u32),
                )) + (":")
                    + value,
                &int_to_alphabet,
            );
            let parsed_move_clone = parsed_move.clone();
            total_moves.push(parsed_move);
            if piece_color == "white" {
                white_piece_moves.push(parsed_move_clone.clone());
            } else {
                black_piece_moves.push(parsed_move_clone.clone());
            }
        }
        for value in piece_attacks.iter() {
            let parsed_move = index_to_move(
                String::from(index_to_position(
                    &(piece_position.0 as u32),
                    &(piece_position.1 as u32),
                )) + (":")
                    + value,
                &int_to_alphabet,
            );
            if piece_color == "white" {
                white_squares_attacked.push(
                    parsed_move.chars().nth(3).unwrap().to_string()
                        + &parsed_move.chars().nth(4).unwrap().to_string(),
                )
            } else {
                black_squares_attacked.push(
                    parsed_move.chars().nth(3).unwrap().to_string()
                        + &parsed_move.chars().nth(4).unwrap().to_string(),
                )
            }
        }
    }
    return (
        total_moves,
        white_piece_moves,
        white_squares_attacked,
        black_piece_moves,
        black_squares_attacked,
        white_king_position.to_string(),
        black_king_position.to_string(),
        pawn_positions,
        board_state_vector,
    );
}
fn index_to_position(first_index: &u32, secound_index: &u32) -> String {
    let mut alphabet_hash: HashMap<u32, &str> = HashMap::new();
    alphabet_hash.insert(0, "h");
    alphabet_hash.insert(1, "g");
    alphabet_hash.insert(2, "f");
    alphabet_hash.insert(3, "e");
    alphabet_hash.insert(4, "d");
    alphabet_hash.insert(5, "c");
    alphabet_hash.insert(6, "b");
    alphabet_hash.insert(7, "a");
    let first_index: String = (first_index + 1).to_string();
    let charector_position = alphabet_hash.get(&secound_index).unwrap();
    let final_charector = charector_position.to_string() + &first_index;
    return final_charector;
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

    let mut int_to_alphabet: HashMap<String, String> = HashMap::new();
    int_to_alphabet.insert("0".to_string(), "h".to_string());
    int_to_alphabet.insert("1".to_string(), "g".to_string());
    int_to_alphabet.insert("2".to_string(), "f".to_string());
    int_to_alphabet.insert("3".to_string(), "e".to_string());
    int_to_alphabet.insert("4".to_string(), "d".to_string());
    int_to_alphabet.insert("5".to_string(), "c".to_string());
    int_to_alphabet.insert("6".to_string(), "b".to_string());
    int_to_alphabet.insert("7".to_string(), "a".to_string());

    let rank_one: [i8; 8] = [4, 0, 0, 6, 0, 0, 0, 4];
    let rank_two: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_three: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_four: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_five: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_six: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_seven: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_eight: [i8; 8] = [-4, 0, 0, -6, 0, 0, 0, -4];
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
    let mut last_move = "";
    let mut custom_move: String = "".to_string();
    let mut white_can_caste_kingside = true;
        let mut white_can_caste_queenside = true;
        let mut black_can_caste_kingside = true;
        let mut black_can_caste_queenside = true;
        let mut white_has_castled = false;
        let mut black_has_castled = false;
    while game_has_ended == false {
        if ! white_has_castled{
            if board_state[0][0] != 4{
                white_can_caste_kingside = false;
            }
            if board_state[0][7] != 4{
                white_can_caste_queenside = false
            }
            if board_state[0][3] != 6{
                white_can_caste_kingside = false;
                white_can_caste_queenside = false;
            }
        }
        if ! black_has_castled{
            if board_state[7][0] != -4{
                black_can_caste_kingside = false;
            }
            if board_state[7][7] != -4{
                black_can_caste_queenside = false
            }
            if board_state[7][3] != -6{
                black_can_caste_kingside = false;
                black_can_caste_queenside = false;
            }
        }
        let moves_and_attacks = moves_and_attacks_from_board_state(board_state, &int_to_alphabet);
        let mut total_moves = moves_and_attacks.0;
        let mut white_piece_moves = moves_and_attacks.1;
        let mut white_piece_attacks = moves_and_attacks.2;
        let mut black_piece_moves = moves_and_attacks.3;
        let mut black_piece_attacks = moves_and_attacks.4;
        println!("there are {} total moves", (total_moves.iter().len()));
        let white_and_black_moves = un_passant(board_state, last_move.to_string(), &alphabet_hash, &mut black_piece_moves, &mut white_piece_moves);
        let unpassant_move = white_and_black_moves.2;
        let mut white_piece_moves = white_and_black_moves.0;
        let mut black_piece_moves = white_and_black_moves.1;
        let parsed_piece_moves = prune_ilegal_moves(
            &mut total_moves,
            board_state,
            &int_to_alphabet,
            &alphabet_hash,
            is_white_to_move,
            &mut white_piece_moves,
            &mut black_piece_moves,
        );
        let mut white_piece_moves = parsed_piece_moves.0;
        let mut black_piece_moves = parsed_piece_moves.1;
        if  !(black_piece_attacks.contains(&"g1".to_string()) ||
                    black_piece_attacks.contains(&"f1".to_string()) ||
                    black_piece_attacks.contains(&"e1".to_string())) && white_can_caste_kingside
                    && board_state[0][2] == 0 &&
                    board_state[0][1] == 0{
                        println!("white can castle kingside");
                        white_piece_moves.push("castle kingside".to_string());
                    }
        if ! (black_piece_attacks.contains(&"c1".to_string()) ||
        black_piece_attacks.contains(&"d1".to_string()) ||
        black_piece_attacks.contains(&"e1".to_string())) && white_can_caste_queenside
        && board_state[0][6] == 0 &&
                    board_state[0][5] == 0{
            white_piece_moves.push("castle queenside".to_string());
            println!("white can castle queenside")
        }
        if ! (white_piece_attacks.contains(&"g8".to_string()) ||
                    white_piece_attacks.contains(&"f8".to_string()) ||
                    white_piece_attacks.contains(&"e8".to_string())) && black_can_caste_kingside
                    && board_state[7][2] == 0 &&
                    board_state[7][1] == 0{
                        black_piece_moves.push("castle kingside".to_string());
                        println!("black can castle kingside")
                    }
        if  !(white_piece_attacks.contains(&"c8".to_string()) ||
        white_piece_attacks.contains(&"d8".to_string()) ||
        white_piece_attacks.contains(&"e8".to_string())) && (black_can_caste_queenside
        && board_state[7][6] == 0 &&
                    board_state[7][5] == 0){
            black_piece_moves.push("castle queenside".to_string());
            println!("black can castle queenside")
        }
        loop {
            custom_move = String::new();
            for value in white_piece_moves.iter(){
                println!("white can go {}", value)
            }
            stdin().read_line(&mut custom_move).unwrap();
            custom_move = custom_move.trim().to_string();
            if is_white_to_move {
                if white_piece_moves.contains(&custom_move) {
                    last_move = &custom_move;
                    if custom_move == "castle kingside"{
                        last_move = "";
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
                }
                    else if custom_move == "castle queenside"{
                        last_move = "";
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
                    }
                    else {
                        let first_value = custom_move.chars().nth(0).unwrap().to_string();
                    let first_value = alphabet_hash.get(&first_value).unwrap();
                    let secound_value = custom_move.chars().nth(1).unwrap().to_string();
                    let mut secound_value: i32 = secound_value.parse().unwrap();
                    secound_value -= 1;
                    let third_value = custom_move.chars().nth(3).unwrap().to_string();
                    let third_value = alphabet_hash.get(&third_value).unwrap();
                    let fourth_value = custom_move.chars().nth(4).unwrap().to_string();
                    let mut fourth_value: i32 = fourth_value.parse().unwrap();
                    fourth_value -= 1;
                        let starting_position =
                        board_state[secound_value as usize][*first_value as usize];
                        let pawn_positions = moves_and_attacks.7;
                    if board_state[fourth_value as usize][*third_value as usize] < 0
                        || pawn_positions.contains(&custom_move[0..2].to_string())
                    {
                        fifty_move_counter = 0.0
                    } else {
                        fifty_move_counter += 0.5;
                    }
                    if fifty_move_counter == 50.0 {
                        result = "draw by fifty move rule";
                        game_has_ended = true
                    } 
                    board_state[fourth_value as usize][*third_value as usize] = starting_position;
                    board_state[secound_value as usize][*first_value as usize] = 0;
                    if custom_move == unpassant_move{
                        board_state[(fourth_value - 1) as usize][*third_value as usize] = 0
                    }
                    if fourth_value == 7 {
                        if board_state[fourth_value as usize][*third_value as usize] == 1 {
                            loop {
                                let mut promotion_piece = String::new();
                                stdin().read_line(&mut promotion_piece).unwrap();
                                promotion_piece = promotion_piece.trim().to_string();
                                if promotion_piece == "rook" {
                                    board_state[fourth_value as usize][*third_value as usize] = 4;
                                    break;
                                } else if promotion_piece == "bishop" {
                                    board_state[fourth_value as usize][*third_value as usize] = 3;
                                    break;
                                } else if promotion_piece == "knight" {
                                    board_state[fourth_value as usize][*third_value as usize] = 2;
                                    break;
                                } else if promotion_piece == "queen" {
                                    board_state[fourth_value as usize][*third_value as usize] = 5;
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
                    let moves_and_attacks =
                        moves_and_attacks_from_board_state(board_state, &int_to_alphabet);
                    let mut white_piece_moves = moves_and_attacks.1;
                    let white_squares_attacked = moves_and_attacks.2;
                    let mut black_piece_moves = moves_and_attacks.3;
                    let black_king_position = moves_and_attacks.6;
                    let mut total_moves = moves_and_attacks.0;
                    let list_of_pieces = moves_and_attacks.8;
                    let parsed_piece_moves = prune_ilegal_moves(
                        &mut total_moves,
                        board_state,
                        &int_to_alphabet,
                        &alphabet_hash,
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
                        let bishop_color = "";
                        for value in list_of_pieces.iter() {
                            match value {
                                Pieces::Knight(knight) => number_of_knights += 1,
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
                    last_move = &custom_move;
                    if custom_move == "castle kingside"{
                        last_move = "";
                        board_state[7][3] = 0;
                        board_state[7][0] = 0;
                        board_state[7][1] = 6;
                        board_state[7][2] = 4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                    if fifty_move_counter == 50.0 {
                        result = "draw by fifty move rule";
                        game_has_ended = true
                    }
                }
                    else if custom_move == "castle queenside"{
                        last_move = "";
                        board_state[7][3] = 0;
                        board_state[7][7] = 0;
                        board_state[7][5] = 6;
                        board_state[7][4] = 4;
                        fifty_move_counter = 0.0;
                        fifty_move_counter += 0.5;
                    if fifty_move_counter == 50.0 {
                        result = "draw by fifty move rule";
                        game_has_ended = true
                    }
                    }
                    else {
                        let first_value = custom_move.chars().nth(0).unwrap().to_string();
                    let first_value = alphabet_hash.get(&first_value).unwrap();
                    let secound_value = custom_move.chars().nth(1).unwrap().to_string();
                    let mut secound_value: i32 = secound_value.parse().unwrap();
                    secound_value -= 1;
                    let third_value = custom_move.chars().nth(3).unwrap().to_string();
                    let third_value = alphabet_hash.get(&third_value).unwrap();
                    let fourth_value = custom_move.chars().nth(4).unwrap().to_string();
                    let mut fourth_value: i32 = fourth_value.parse().unwrap();
                    fourth_value -= 1;
                    let starting_position =
                        board_state[secound_value as usize][*first_value as usize];
                    let pawn_positions = moves_and_attacks.7;
                    if board_state[fourth_value as usize][*third_value as usize] > 0
                        || pawn_positions.contains(&custom_move[0..2].to_string())
                    {
                        fifty_move_counter = 0.0
                    } else {
                        fifty_move_counter += 0.5;
                    }
                    if fifty_move_counter == 50.0 {
                        result = "draw by fifty move rule";
                        game_has_ended = true
                    }
                    board_state[fourth_value as usize][*third_value as usize] = starting_position;
                    board_state[secound_value as usize][*first_value as usize] = 0;
                    if custom_move == unpassant_move{
                        board_state[(fourth_value + 1) as usize][*third_value as usize] = 0
                    }
                    if fourth_value == 0 {
                        if board_state[fourth_value as usize][*third_value as usize] == -1 {
                            loop {
                                let mut promotion_piece = String::new();
                                stdin().read_line(&mut promotion_piece).unwrap();
                                promotion_piece = promotion_piece.trim().to_string();
                                if promotion_piece == "rook" {
                                    board_state[fourth_value as usize][*third_value as usize] = -4;
                                    break;
                                } else if promotion_piece == "bishop" {
                                    board_state[fourth_value as usize][*third_value as usize] = -3;
                                    break;
                                } else if promotion_piece == "knight" {
                                    board_state[fourth_value as usize][*third_value as usize] = -2;
                                    break;
                                } else if promotion_piece == "queen" {
                                    board_state[fourth_value as usize][*third_value as usize] = -5;
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
                    let moves_and_attacks =
                        moves_and_attacks_from_board_state(board_state, &int_to_alphabet);
                    let mut white_piece_moves = moves_and_attacks.1;
                    let mut black_piece_moves = moves_and_attacks.3;
                    let black_squares_attacked = moves_and_attacks.4;
                    let white_king_position = moves_and_attacks.5;
                    let mut total_moves = moves_and_attacks.0;
                    let parsed_piece_moves = prune_ilegal_moves(
                        &mut total_moves,
                        board_state,
                        &int_to_alphabet,
                        &alphabet_hash,
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
                                Pieces::Knight(Knight) => number_of_knights += 1,
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