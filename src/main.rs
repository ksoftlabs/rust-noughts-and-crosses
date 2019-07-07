use std::io;
fn main() {
    let mut board=[" "," "," "," "," "," "," "," "," "];
    for i in 0..10{
        
    }


}

fn draw_board(board: &mut[&str;9]){
    print!("{}{}{}",board[0],board[1],board[2]);
    print!("{}{}{}",board[3],board[4],board[5]);
    print!("{}{}{}",board[6],board[7],board[8]);
}

fn get_player_symbol()->String{
    let mut player_symbol =String::new();

    println!("Enter your symbol? (X or O)");
    io::stdin().read_line(&mut player_symbol)
        .expect("Error!");

    if player_symbol.trim() == "X" || player_symbol.trim()== "x"{
        player_symbol="X".to_owned();
    } else if player_symbol.trim() == "O" || player_symbol.trim()== "o" {
        player_symbol="O".to_owned();
    } else {
        player_symbol="X".to_owned();
    }

    return player_symbol;
}

fn get_player_move()->String{
    println!("Enter your move");
    let mut player_move =String::new();
    io::stdin().read_line(&mut player_move)
        .expect("Error!");

    return player_move
}


