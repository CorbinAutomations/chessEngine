use std::{collections::HashMap, io::stdin,};
struct Rook{
    position: String,
    color: String,
    is_alive: bool,
}
struct Bishop{
    position: String,
    color: String,
    is_alive: bool,
}

struct Knight{
    position: String,
    color: String,
    is_alive: bool,
}

struct Queen{
    position: String,
    color: String,
    is_alive: bool,
}

struct Pawn{
    position: String,
    color: String,
    is_alive: bool,
}

struct King{
    position: String,
    color: String,
    is_alive: bool,
}
enum Pieces {
    Rook(Rook),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Bishop(Bishop),
    King(King),
}
impl Rook {
    fn rook_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        
        let mut first_index_clone: i32 = first_index.clone();
        let secound_index_clone: i32 = secound_index.clone();
        while first_index_clone < 7{
            first_index_clone += 1;
            if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
            }
            else if board[first_index_clone as usize][*secound_index as usize] < 0{
                if self.color == "white"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }else{
                if self.color == "black"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }

        let mut first_index_clone = first_index.clone();
        while first_index_clone > 0 {
            first_index_clone -= 1;
            if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                legal_moves.push(first_index.to_string() + "," + &secound_index_clone.to_string());
            }
            else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                if self.color == "white"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }else{
                if self.color == "black"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }
        }
        let first_index_clone = first_index.clone();
        let mut secound_index_clone = secound_index.clone();
        while secound_index_clone < 7 {
            secound_index_clone += 1;
            if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
            }
            else if board[first_index as usize][*secound_index as usize] < 0{
                if self.color == "white"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }else{
                if self.color == "black"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }
}

        let mut secound_index_clone = secound_index.clone();
        while secound_index_clone > 0 {
            secound_index_clone -= 1;
            if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
            }
            else if board[first_index as usize][*secound_index as usize] < 0{
                if self.color == "white"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }else{
                if self.color == "black"{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                break;
            }
        }
        }
        return legal_moves;
}
            }

impl Bishop {
    fn bishop_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        loop{
            let mut first_index_clone: i32 = first_index.clone();
            let mut secound_index_clone: i32 = secound_index.clone();
            while first_index_clone < 7 && secound_index_clone < 7 {
                first_index_clone += 1;
                secound_index_clone += 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][*secound_index as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }

            let mut first_index_clone = first_index.clone();
            let mut secound_index_clone: i32 = secound_index.clone();
            while first_index_clone > 0 && secound_index_clone > 0 {
                first_index_clone -= 1;
                secound_index_clone -= 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            let mut first_index_clone: i32 = first_index.clone();
            let mut secound_index_clone: i32 = secound_index.clone();
            while first_index_clone < 7 && secound_index_clone > 0 {
                first_index_clone += 1;
                secound_index_clone -= 1 ;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index as usize][*secound_index as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
                else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
}

            let mut first_index_clone = first_index.clone();
            let mut secound_index_clone = secound_index.clone();
            while first_index_clone > 0 && secound_index_clone < 7 {
                first_index_clone -= 1;
                secound_index_clone += 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index as usize][*secound_index as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            break legal_moves;
            }
}
            }
        
impl Knight {
    fn knight_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        if first_index + 2 < 7 && secound_index - 1 >= 0{
            if board[first_index as usize + 2][*secound_index as usize - 1] == 0 {
                legal_moves.push((first_index + 2).to_string() + "," + &(secound_index - 1).to_string());
            }
            else if board[first_index as usize + 2][*secound_index as usize - 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 2).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 2).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
        }
        if first_index + 2 < 7 && secound_index + 1 <= 7{
            if board[first_index as usize + 2][*secound_index as usize + 1] == 0 {
                legal_moves.push((first_index + 2).to_string() + "," + &(secound_index + 1).to_string());
            }
            else if board[first_index as usize + 2][*secound_index as usize + 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 2).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 2).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
        }
        if first_index - 2 > 0 && secound_index - 1 >= 0{
            if board[first_index as usize - 2][*secound_index as usize - 1] == 0 {
                legal_moves.push((first_index - 2).to_string() + "," + &(secound_index - 1).to_string());
            }
            else if board[first_index as usize - 2][*secound_index as usize - 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 2).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 2).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
        }
        if first_index - 2 > 0 && secound_index + 1 <= 7{
            if board[first_index as usize - 2][*secound_index as usize + 1] == 0 {
                legal_moves.push((first_index - 2).to_string() + "," + &(secound_index + 1).to_string());
            }
            else if board[first_index as usize - 2][*secound_index as usize + 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 2).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 2).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
        }

        if first_index + 1 < 7 && secound_index - 2 > 0{
            if board[first_index as usize + 1][*secound_index as usize - 2] == 0 {
                legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 2).to_string());
            }
            else if board[first_index as usize + 1][*secound_index as usize - 2] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 2).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 2).to_string());
                }
            }
        }
        if first_index + 1 < 7 && secound_index + 2 < 8{
            if board[first_index as usize + 1][*secound_index as usize + 2] == 0 {
                legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 2).to_string());
            }
            else if board[first_index as usize + 1][*secound_index as usize + 2] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 2).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 2).to_string());
                }
            }
        }
        if first_index - 1 > 0 && secound_index - 2 > 0{
            if board[first_index as usize - 1][*secound_index as usize - 2] == 0 {
                legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 2).to_string());
            }
            else if board[first_index as usize - 1][*secound_index as usize - 2] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 2).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 2).to_string());
                }
            }
        }
        if first_index - 1 > 0 && secound_index + 2 < 8{
            if board[first_index as usize - 1][*secound_index as usize + 2] == 0 {
                legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 2).to_string());
            }
            else if board[first_index as usize - 1][*secound_index as usize + 2] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 2).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 2).to_string());
                }
            }
        }
        return legal_moves;
        }
    }

impl Queen {
    fn queen_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        loop{
            let mut first_index_clone: i32 = first_index.clone();
            let secound_index_clone: i32 = secound_index.clone();
            while first_index_clone < 7 {
                first_index_clone += 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }

            let mut first_index_clone = first_index.clone();
            while first_index_clone > 0 {
                first_index_clone -= 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            let first_index_clone = first_index.clone();
            let mut secound_index_clone = secound_index.clone();
            while secound_index_clone < 7 {
                secound_index_clone += 1;
                println!("{}", secound_index_clone as usize);
                println!("board is {}", board[first_index_clone as usize][secound_index_clone as usize]);
                println!("first index is {}", first_index_clone);

                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
}

            let mut secound_index_clone = secound_index.clone();
            while secound_index_clone > 0 {
                secound_index_clone -= 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            }
            let mut first_index_clone: i32 = first_index.clone();
            let mut secound_index_clone: i32 = secound_index.clone();
            while first_index_clone < 7 && secound_index_clone < 7 {
                first_index_clone += 1;
                secound_index_clone += 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }

            let mut first_index_clone = first_index.clone();
            let mut secound_index_clone: i32 = secound_index.clone();
            while first_index_clone > 0 && secound_index_clone > 0{
                first_index_clone -= 1;
                secound_index_clone -= 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            let mut first_index_clone = first_index.clone();
            let mut secound_index_clone = secound_index.clone();
            while first_index_clone < 7 && secound_index_clone > 0 {
                first_index_clone += 1;
                secound_index_clone -= 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
}

            let mut first_index_clone = first_index.clone();
            let mut secound_index_clone = secound_index.clone();
            while first_index_clone > 0 && secound_index_clone < 7{
                first_index_clone -= 1;
                secound_index_clone += 1;
                if board[first_index_clone as usize][secound_index_clone as usize] == 0{
                    legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                }
                else if board[first_index_clone as usize][secound_index_clone as usize] < 0{
                    if self.color == "white"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }else{
                    if self.color == "black"{
                        legal_moves.push(first_index_clone.to_string() + "," + &secound_index_clone.to_string());
                    }
                    break;
                }
            }
            }
            break legal_moves;
        }
            }
        }
impl Pawn {
    fn pawn_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        if self.color == "white"{
            if first_index + 1 < 7{
                if board[first_index as usize + 1][*secound_index as usize] == 0 {
                    legal_moves.push((first_index + 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 2 < 7 && first_index == 1 {
                if board[first_index as usize + 2][*secound_index as usize] == 0 {
                    legal_moves.push((first_index + 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index + 1 < 7 && secound_index - 1 >= 0{
                if board[first_index as usize + 1][*secound_index as usize - 1] < 0 {
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }

            if first_index + 1 < 7 && secound_index + 1 <= 7{
                if board[first_index as usize + 1][*secound_index as usize + 1] < 0 {
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 1).to_string());
                }
            }

        }

        if self.color == "black"{
            if first_index - 1 > 0{
                if board[first_index as usize - 1][*secound_index as usize] == 0 {
                    legal_moves.push((first_index - 1).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index - 2 > 0 && first_index == 6 {
                if board[first_index as usize - 2][*secound_index as usize] == 0 {
                    legal_moves.push((first_index - 2).to_string() + "," + &secound_index.to_string());
                }
            }
            if first_index -1 > 0 && secound_index - 1 >= 0{
                if board[first_index as usize - 1][*secound_index as usize - 1] > 0 {
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            if first_index -1 > 0 && secound_index + 1 <= 7{
                if board[first_index as usize - 1][*secound_index as usize + 1] < 0 {
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
        }
        return legal_moves;
    }
}
impl King {
    fn king_moves(&self, board: [[i8;8];8], alphabet_hash: &HashMap<String, i32>) -> Vec<String>{
        let curent_position = &self.position;
        let secound_index = String::from(curent_position.chars().nth(0).unwrap());
        let first_index = String::from(curent_position.chars().nth(1).unwrap());
        let secound_index = alphabet_hash.get(&secound_index).unwrap();
        let mut first_index: i32 = first_index.parse().unwrap();
        first_index -= 1;
        let mut legal_moves:Vec<String> = vec![];
        
        if first_index + 1 < 7 && secound_index + 1 <= 7{
            println!("{}", board[first_index as usize + 1][*secound_index as usize + 1]);
            if board[first_index as usize + 1][*secound_index as usize + 1] == 0 {
                legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 1).to_string());
            }           
            else if board[first_index as usize + 1][*secound_index as usize + 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
            else if board[first_index as usize + 1][*secound_index as usize + 1] > 0{
                if self.color == "black"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index + 1).to_string());
                    println!("{}", board[first_index as usize + 1][*secound_index as usize + 1])
                }
            }
        }
        
        if first_index + 1 < 7 && secound_index - 1 >= 0{
            if board[first_index as usize + 1][*secound_index as usize - 1] == 0 {
                legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
            }
            else if board[first_index as usize + 1][*secound_index as usize - 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
        }

        if first_index -1 > 0 && secound_index - 1 >= 0{
            if board[first_index as usize - 1][*secound_index as usize - 1] == 0 {
                legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 1).to_string());
            }
            else if board[first_index as usize - 1][*secound_index as usize - 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
        }

        if first_index -1 > 0 && secound_index + 1 <= 7{
            if board[first_index as usize - 1][*secound_index as usize + 1] == 0 {
                legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 1).to_string());
            }
            else if board[first_index as usize - 1][*secound_index as usize + 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 1).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
        }        
        
        if first_index -1 > 0{
            if board[first_index as usize - 1][*secound_index as usize] == 0 {
                legal_moves.push((first_index - 1).to_string() + "," + &secound_index.to_string());
            }
            else if board[first_index as usize - 1][*secound_index as usize] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index - 1).to_string() + "," + &secound_index.to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index - 1).to_string() + "," + &secound_index.to_string());
                }
            }
        }

        if first_index + 1 < 7{
            if board[first_index as usize + 1][*secound_index as usize] == 0 {
                legal_moves.push((first_index + 1).to_string() + "," + &secound_index.to_string());
            }
            else if board[first_index as usize + 1][*secound_index as usize] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index + 1).to_string() + "," + &secound_index.to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index + 1).to_string() + "," + &secound_index.to_string());
                }
            }
        }

        if secound_index - 1 >= 0{
            if board[first_index as usize][*secound_index as usize - 1] == 0 {
                legal_moves.push(first_index.to_string() + "," + &(secound_index - 1).to_string());
            }
            else if board[first_index as usize][*secound_index as usize - 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index).to_string() + "," + &(secound_index - 1).to_string());
                }
            }
        }

        if secound_index + 1 <= 7{
            if board[first_index as usize][*secound_index as usize + 1] == 0 {
                legal_moves.push(first_index.to_string() + "," + &(secound_index + 1).to_string());
            }
            else if board[first_index as usize][*secound_index as usize + 1] < 0{
                if self.color == "white"{
                    legal_moves.push((first_index).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
            else{
                if self.color == "black"{
                    legal_moves.push((first_index).to_string() + "," + &(secound_index + 1).to_string());
                }
            }
        }
        return legal_moves
    }
}

fn index_to_move(value: String, int_to_alphabet_hash: &HashMap<String, String>) -> String{
    let first_value = value.chars().nth(0).unwrap().to_string();
    let secound_value = value.chars().nth(1).unwrap().to_string();
    let third_value = value.chars().nth(2).unwrap().to_string();
    let six_value = value.chars().nth(3).unwrap().to_string();
    let alphabet_value = int_to_alphabet_hash.get(&value.chars().nth(5).unwrap().to_string());
    let mut six_value: i32 = six_value.parse().unwrap();
    six_value += 1;
    return first_value + &secound_value + &third_value + alphabet_value.unwrap() + &six_value.to_string();
}
fn index_to_position(first_index: &u32, secound_index: &u32) -> String{
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
    return final_charector
}
fn main(){
    // 0 = empty, 1 = pawn, 2 = knight, 3 = pawn, 4 = rook, 
    // 5 = queen, 6 = king, positive equals white negative = black
    let mut alphabet_hash = HashMap::new();
    alphabet_hash.insert(String::from("a"), 0);
    alphabet_hash.insert(String::from("b"), 1);
    alphabet_hash.insert(String::from("c"), 2);
    alphabet_hash.insert(String::from("d"), 3);
    alphabet_hash.insert(String::from("e"), 4);
    alphabet_hash.insert(String::from("f"), 5);
    alphabet_hash.insert(String::from("g"), 6);
    alphabet_hash.insert(String::from("h"), 7);

    let mut int_to_alphabet: HashMap<String, String> = HashMap::new();
    int_to_alphabet.insert("0".to_string(), "a".to_string());
    int_to_alphabet.insert("1".to_string(), "b".to_string());
    int_to_alphabet.insert("2".to_string(), "c".to_string());
    int_to_alphabet.insert("3".to_string(), "d".to_string());
    int_to_alphabet.insert("4".to_string(), "e".to_string());
    int_to_alphabet.insert("5".to_string(), "f".to_string());
    int_to_alphabet.insert("6".to_string(), "g".to_string());
    int_to_alphabet.insert("7".to_string(), "h".to_string());

    let rank_one:[i8;8] = [4, 2, 3, 6, 5, 3, 2, 4];
    let rank_two:[i8;8] = [1, 1, 1, 1, 1, 1, 1, 1];
    let rank_three:[i8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_four:[i8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_five:[i8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_six:[i8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let rank_seven:[i8;8] = [-1, -1, -1, -1, -1, -1, -1, -1];
    let rank_eight:[i8;8] = [-4, -2, -3, -6, -5, -3, -2, -4];
    let mut board_state:[[i8;8];8] = [rank_one, rank_two, rank_three, rank_four, rank_five, rank_six, rank_seven, rank_eight];
    for value in board_state.iter(){
        let array_iterator = value.iter();
        for value in array_iterator{
            print!("{}", value)
        }
        println!()
    }
    
    let mut game_has_ended = false;
    let mut is_white_to_move = true;
    while game_has_ended == false{
        let mut board_state_vector: Vec<Pieces> = vec![];
        let mut first_index_counter = 0;
        for value in board_state.iter(){
            let mut secound_index_counter = 0;
            let mut struct_color = "";
            for value in value.iter(){
                if *value < 0{
                    struct_color = "black";
                }else if *value > 0 {
                    struct_color = "white";
                }
                let position: String = index_to_position(&first_index_counter, &secound_index_counter);
                if *value == 1 || *value == -1{
                    board_state_vector.push(Pieces::Pawn(Pawn{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                if *value == 2 || *value == -2{
                    board_state_vector.push(Pieces::Knight(Knight{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                if *value == 3 || *value == -3{
                    board_state_vector.push(Pieces::Bishop(Bishop{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                if *value == 4 || *value == -4{
                    board_state_vector.push(Pieces::Rook(Rook{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                if *value == 5 || *value == -5{
                    board_state_vector.push(Pieces::Queen(Queen{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                if *value == 6 || *value == -6{
                    board_state_vector.push(Pieces::King(King{
                        position: String::from(&position),
                        color: String::from(struct_color),
                        is_alive: true,
                    }));
                }
                secound_index_counter += 1;
            }
            first_index_counter += 1
        }
        let mut total_moves:Vec<String> = vec![];
        let mut white_piece_moves:Vec<String> = vec![];
        let mut black_piece_moves:Vec<String> = vec![];
        for value in board_state_vector.iter(){
            let mut piece_position = "";
            let mut piece_color = "";
            let mut piece_is_alive = false;
            let mut piece_moves = vec![];
            match value {
                Pieces::Rook(rook) => {piece_color = &rook.color;
                piece_is_alive = rook.is_alive;
            piece_position = &rook.position;
            piece_moves = rook.rook_moves(board_state, &alphabet_hash)
        },
            Pieces::Knight(knight) => {piece_color = &knight.color;
                piece_is_alive = knight.is_alive;
            piece_position = &knight.position;
            piece_moves = knight.knight_moves(board_state, &alphabet_hash)},
            Pieces::Pawn(pawn) => {piece_color = &pawn.color;
                piece_is_alive = pawn.is_alive;
            piece_position = &pawn.position;
            piece_moves = pawn.pawn_moves(board_state, &alphabet_hash)},
            Pieces::Queen(queen) => {piece_color = &queen.color;
                piece_is_alive = queen.is_alive;
            piece_position = &queen.position;
            piece_moves = queen.queen_moves(board_state, &alphabet_hash)},
            Pieces::Bishop(bishop) => {piece_color = &bishop.color;
                piece_is_alive = bishop.is_alive;
            piece_position = &bishop.position;
            piece_moves = bishop.bishop_moves(board_state, &alphabet_hash)},
            Pieces::King(king) => {piece_color = &king.color;
                piece_is_alive = king.is_alive;
            piece_position = &king.position;
            piece_moves = king.king_moves(board_state, &alphabet_hash)}
            }
            if piece_is_alive == true{
                for value in piece_moves.iter(){
                    let parsed_move =  index_to_move(String::from(piece_position) + (":") + value, &int_to_alphabet);
                    let parsed_move_clone = parsed_move.clone();
                    total_moves.push(parsed_move);
                    if piece_color == "white"{
                        white_piece_moves.push(parsed_move_clone);
                    }else{
                        black_piece_moves.push(parsed_move_clone);
                    }
                }
                }
    
        }
            
            let total_moves_iter = total_moves.iter();
            let mut x = 0;
    
            for _ in total_moves_iter{
                x += 1
            }
            println!("{}", x);
        loop {
            let mut custom_move = String::new();
            stdin().read_line(&mut custom_move).unwrap();
            let custom_move = custom_move.trim().to_string();
            if is_white_to_move == true{
                let white_move_iter = white_piece_moves.iter();
                for _ in white_move_iter{
                    //println!{"white move value is {}", value}
                }
            }else{
                    let black_moves_iter = black_piece_moves.iter();
                    for _ in black_moves_iter{
                        //println!("black move value is {}", value)
                    }

                }
            if is_white_to_move == true{
                if white_piece_moves.contains(&custom_move){
                    
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
                    let starting_position = board_state[secound_value as usize][*first_value as usize];
                    board_state[fourth_value as usize][*third_value as usize] = starting_position;
                    println!("starting position is {}", starting_position);
                    board_state[secound_value as usize][*first_value as usize] = 0;
                    if is_white_to_move == true{
                        is_white_to_move = false;
                        }else {                        
                            is_white_to_move = true;
                        }
                        for value in board_state.iter(){
                            let array_iterator = value.iter();
                            for value in array_iterator{
                                print!("{}", value)
                            }
                            println!()}
                        break;
                }else{
                    println!("invalid move please try again");
                }
            }else{
                    if black_piece_moves.contains(&custom_move){
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
                        let starting_position = board_state[secound_value as usize][*first_value as usize];
                        board_state[fourth_value as usize][*third_value as usize] = starting_position;
                        println!("starting position is {}", starting_position);
                        board_state[secound_value as usize][*first_value as usize] = 0;
                        if is_white_to_move == true{
                        is_white_to_move = false;
                        }else {                        
                            is_white_to_move = true;
                        }
                        for value in board_state.iter(){
                            let array_iterator = value.iter();
                            for value in array_iterator{
                                print!("{}", value)
                            }
                            println!()}
                        break;

                }else{
                    println!("invalid move please try again")
                }
        }
    }
        }
    }