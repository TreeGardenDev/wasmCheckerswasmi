use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use wasmi::{
    ExternVal, ImportsBuilder, MemoryRef, Module, ModuleImportResolver, ModuleInstance, ModuleRef,
    RuntimeValue,
};

use super::import::RuntimeModuleImportResolver;
use super::runtime::Runtime;

pub struct CheckerGame {
    module_instance: ModuleRef,
    runtime: Runtime,
}
impl CheckerGame {
    pub fn new(module_file: &str) -> CheckerGame {
        let import_resolver = RuntimeModuleImportResolver::new();
        let module_instance = load_instance(&import_resolver, module_file).unwrap();
        let runtime = Runtime::new();

        CheckerGame {
            module_instance,
            runtime,
        }
    }
    pub fn init(&mut self) -> Result<()> {
        self.module_instance
            .invoke_export("initBoard", &[], &mut self.runtime)?;
        Ok(())
    }
    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> Result<bool> {
        let res = self.module_instance.invoke_export(
            "move",
            &[
                RuntimeValue::I32(from.0),
                RuntimeValue::I32(from.1),
                RuntimeValue::I32(to.0),
                RuntimeValue::I32(to.1),
            ],
            &mut self.runtime,
        )?;
        match res {
            Some(RuntimeValue::I32(x)) => Ok(x != 0),
            _ => {
                println!("Did not get an appropriate response");
                Ok(false)
            }
        }
    }
    pub fn get_turn_owner(&mut self) -> Result<PieceColor> {
        let res = self
            .module_instance
            .invoke_export("getTurnOwner", &[], &mut self.runtime)?;
        match res {
            Some(RuntimeValue::I32(x)) => {
                if x != 0 {
                    Ok(PieceColor::White)
                } else {
                    Ok(PieceColor::Black)
                }
            }
            _ => Err(From::from("Did not get an appropriate response")),
        }
    }
    pub fn get_board_contents(&mut self) -> Result<String> {
        let export = self.module_instance.export_by_name("memory");
        let header = r#"
            0 1 2 3 4 5 6 7
            +-+-+-+-+-+-+-+"#;
        let footer = "+-+-+-+-+-+-+-+";
        let middle_string = match export {
            Some(ExternVal::Memory(mr)) => gen_board(&mr),
            _ => "-- No board found --".to_string(),
        };
        Ok(format!("{}\n{}\n{}", header, middle_string, footer))
    }
}
fn gen_board(memory: &MemoryRef) -> String {
    let mut vals = Vec::<String>::new();
    for y in 0..8 {
        vals.push(format!("{}|", y));
        for x in 0..8 {
            let offset = calc_offset(x, y);
            let bytevec: Vec<u8> = memory.get(offset, 4).unwrap();
            let value = to_u32(&bytevec[..]);
            vals.push(format!("{}|", value_label(value)));
        }
        vals.push("\n".to_string());
    }
    vals.join("")
}
fn value_label(v: u32) -> String {
    match v {
        0 => " ".to_string(),
        1 => "B".to_string(),
        2 => "w".to_string(),
        5 => "B*".to_string(),
        6 => "W*".to_string(),
        _ => "???".to_string(),
    }
    .into()
}
fn to_u32(bytes: &[u8]) -> u32 {
    bytes.iter().rev().fold(0, |acc, &b| (acc * 2) + b as u32)
}
fn calc_offset(x: i32, y: i32) -> u32 {
    (y * 8 + x) as u32
}

#[derive(Debug)]
pub enum PieceColor {
    Black,
    White,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Coordinate = (i32, i32);

fn load_instance(
    import_resolver: &impl ModuleImportResolver,
    module_file: &str,
) -> Result<ModuleRef> {
    let mut buffer = Vec::new();
    let mut file = File::open(module_file)?;
    file.read_to_end(&mut buffer)?;
    let module = Module::from_buffer(buffer)?;
    let mut builder = ImportsBuilder::new();
    builder.push_resolver("events", import_resolver);
    Ok(ModuleInstance::new(&module, &builder)
        .expect("Failed to Instantiate WASM MODULE")
        .assert_no_start())
}
