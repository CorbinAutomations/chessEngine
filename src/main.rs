use std::{collections::HashMap, io::stdin};
struct Rook {
    position: (i8, i8),
    color: String,
}
struct Bishop {
    position: (i8, i8),
    color: String,
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
    for value in legal_moves.iter() {
        println!("value is {}", value)
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
            if first_index + 1 < 7 {
                if board[first_index as usize + 1][secound_index as usize] == 0 {
                    legal_moves
                        .push((first_index + 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 2 < 7 && first_index == 1 {
                if board[first_index as usize + 2][secound_index as usize] == 0
                    && board[first_index as usize + 1][secound_index as usize] == 0
                {
                    legal_moves
                        .push((first_index + 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 1 < 7 && secound_index - 1 >= 0 {
                if board[first_index as usize + 1][secound_index as usize - 1] < 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index - 1).to_string(),
                    );
                }
            }

            if first_index + 1 < 7 && secound_index + 1 <= 7 {
                if board[first_index as usize + 1][secound_index as usize + 1] < 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index + 1).to_string(),
                    );
                }
            }
        }

        if self.color == "black" {
            if first_index - 1 > 0 {
                if board[first_index as usize - 1][secound_index as usize] == 0 {
                    legal_moves
                        .push((first_index - 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index - 2 > 0 && first_index == 6 {
                if board[first_index as usize - 2][secound_index as usize] == 0
                    && board[first_index as usize - 1][secound_index as usize] == 0
                {
                    legal_moves
                        .push((first_index - 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index - 1 > 0 && secound_index - 1 >= 0 {
                if board[first_index as usize - 1][secound_index as usize - 1] > 0 {
                    legal_moves.push(
                        (first_index + 1).to_string() + "," + &(secound_index - 1).to_string(),
                    );
                }
            }
            if first_index - 1 > 0 && secound_index + 1 <= 7 {
                if board[first_index as usize - 1][secound_index as usize + 1] < 0 {
                    legal_moves.push(
                        (first_index - 1).to_string() + "," + &(secound_index + 1).to_string(),
                    );
                }
            }
        }
        return legal_moves;
    }
    fn pawn_attacks(&self, board: [[i8; 8]; 8]) -> Vec<String> {
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
fn moves_and_attacks_from_board_state(
    board_state: [[i8; 8]; 8],
    int_to_alphabet: &HashMap<String, String>,
) -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
) {
    let mut board_state_vector: Vec<Pieces> = vec![];
    let mut first_index_counter: i8 = 0;
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
                board_state_vector.push(Pieces::Bishop(Bishop {
                    position: (first_index_counter, secound_index_counter),
                    color: String::from(struct_color),
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
                piece_attacks = pawn.pawn_attacks(board_state)
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
                println!(
                    "{} king is at {}",
                    piece_color,
                    index_to_position(&(piece_position.0 as u32), &(piece_position.1 as u32))
                );
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
    for value in board_state.iter() {
        let array_iterator = value.iter();
        for value in array_iterator {
            print!("{}", value)
        }
        println!()
    }

    let mut game_has_ended = false;
    let mut is_white_to_move = true;
    while game_has_ended == false {
        let moves_and_attacks = moves_and_attacks_from_board_state(board_state, &int_to_alphabet);
        let total_moves = moves_and_attacks.0;
        let white_piece_moves = moves_and_attacks.1;
        let white_squares_attacked = moves_and_attacks.2;
        let black_piece_moves = moves_and_attacks.3;
        let black_squares_attacked = moves_and_attacks.4;
        println!("there are {} total moves", (total_moves.iter().len()));
        loop {
            let mut custom_move = String::new();
            stdin().read_line(&mut custom_move).unwrap();
            let custom_move = custom_move.trim().to_string();
            if is_white_to_move {
                if white_piece_moves.contains(&custom_move) {
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
                    board_state[fourth_value as usize][*third_value as usize] = starting_position;
                    println!("starting position is {}", starting_position);
                    board_state[secound_value as usize][*first_value as usize] = 0;
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
                    break;
                } else {
                    println!("invalid move please try again");
                }
            } else {
                if black_piece_moves.contains(&custom_move) {
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
                    board_state[fourth_value as usize][*third_value as usize] = starting_position;
                    println!("starting position is {}", starting_position);
                    board_state[secound_value as usize][*first_value as usize] = 0;
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
                    break;
                } else {
                    println!("invalid move please try again")
                }
            }
        }
    }
}
