use std::ops::Deref;
use wamr_sys::wasm_function_inst_t;

pub struct Function(pub(crate) wasm_function_inst_t);

impl Deref for Function {
    type Target = wasm_function_inst_t;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
