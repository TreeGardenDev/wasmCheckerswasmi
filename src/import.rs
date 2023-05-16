use wasmi::{
    Error as InterpreterError, FuncInstance, FuncRef, Signature, ValueType, ModuleImportResolver,
};

pub const PIECEMOVED_INDEX: usize = 0;
pub const PIECECROWNED_INDEX: usize = 1;

pub struct RuntimeModuleImportResolver;
impl RuntimeModuleImportResolver {
    pub fn new() -> Self {
        RuntimeModuleImportResolver {}
    }
}
impl <'a>ModuleImportResolver for RuntimeModuleImportResolver{
    fn resolve_func(&self, file_name:&str, _signature:&Signature)-> Result<FuncRef, InterpreterError>{
        match file_name {
            "piecemoved" => Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32][..], None),
                PIECEMOVED_INDEX,
            )),
            "piececrowned" => Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                PIECECROWNED_INDEX,
            )),
            _ => panic!("env::{} not found", file_name),
        }
    }
}
