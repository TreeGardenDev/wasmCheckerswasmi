extern crate wasmi;
pub mod checkersgame;
pub mod runtime;
pub mod import;

use checkersgame::CheckerGame;
use std::error::Error;


fn main()-> Result<(), Box<dyn Error>>{
    let mut game= CheckerGame::new("checkers.wasm");
    game.init()?;

    let board_display= game.get_board_contents()?;
    println!("game board at start: \n\n{}\n", board_display);
    println!("At game start, current turn is: {:?}", game.get_turn_owner()?);
    game.move_piece(&(0,5), &(0,4))?;
    println!("After moving piece, current turn is: {:?}", game.get_turn_owner()?);
    let board_display= game.get_board_contents()?;
    println!("game board after move: \n\n{}\n", board_display);
    Ok(())
    

}
