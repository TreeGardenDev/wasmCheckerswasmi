use super::import::{PIECEMOVED_INDEX, PIECECROWNED_INDEX};
use wasmi::{Externals, RuntimeArgs, RuntimeValue, Trap};

pub struct Runtime{}
impl Runtime{
    pub fn new()->Self{
        Runtime{}
    }
    fn handle_piece_moved(&self, from:(i32,i32), to:(i32,i32))->Result<Option<RuntimeValue>, Trap>{
        println!("Piece moved from {:?} to {:?}", from, to);
        Ok(None)
    }
    fn handle_piece_crowned(&self, loc:(i32,i32))->Result<Option<RuntimeValue>, Trap>{
        println!("Piece crowned at {:?}", loc);
        Ok(None)
    }
    
}
impl Externals for Runtime{
    fn invoke_index(&mut self, index:usize, args:RuntimeArgs)->Result<Option<RuntimeValue>, Trap>{
        match index{
            PIECEMOVED_INDEX => {
                let from_x: i32 = args.nth(0);
                let from_y: i32 = args.nth(1);
                let to_x: i32 = args.nth(2);
                let to_y: i32 = args.nth(3);

                self.handle_piece_moved((from_x,from_y),(to_x,to_y))
            }
            PIECECROWNED_INDEX => {
                let piece_x: i32 = args.nth(0);
                let piece_y: i32 = args.nth(1);
                self.handle_piece_crowned((piece_x, piece_y))
            }   
            _ => panic!("unknown function index"),
        }
    }
}

