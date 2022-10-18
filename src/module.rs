use std::{ops::Deref, ptr};
use wamr_sys::wasm_module_t;

pub struct Module(wasm_module_t);

impl Module {
    pub fn create(buf: &mut [u8]) -> Self {
        Module(unsafe {
            wamr_sys::wasm_runtime_load(
                buf.as_mut_ptr() as *mut u8,
                buf.len() as u32,
                ptr::null_mut(),
                0,
            )
        })
    }
}

impl Deref for Module {
    type Target = wasm_module_t;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            wamr_sys::wasm_runtime_unload(self.0);
        }
    }
}
