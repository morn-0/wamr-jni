use crate::function::Function;
use std::{
    ffi::CString,
    ptr::{self},
};
use wamr_sys::{
    wasm_exec_env_t, wasm_function_inst_t, wasm_module_inst_t, wasm_module_t, wasm_val_t,
    wasm_val_t__bindgen_ty_1,
};

pub struct Instance {
    inst: wasm_module_inst_t,
    env: wasm_exec_env_t,
}

impl Instance {
    pub fn create(module: wasm_module_t, stack_size: u32, heap_size: u32) -> Self {
        let inst = unsafe {
            wamr_sys::wasm_runtime_instantiate(module, stack_size, heap_size, ptr::null_mut(), 0)
        };
        let env = unsafe { wamr_sys::wasm_runtime_create_exec_env(inst, stack_size) };

        Instance { inst, env }
    }

    pub fn lookup_function(&self, name: &str) -> Function {
        let c_name = CString::new(name).unwrap();
        let c_ptr = c_name.as_ptr();
        Function(unsafe { wamr_sys::wasm_runtime_lookup_function(self.inst, c_ptr, ptr::null()) })
    }

    pub fn call(&self, func: wasm_function_inst_t, data: &[u8]) -> Vec<u8> {
        let mut result = wasm_val_t {
            kind: wamr_sys::wasm_valkind_enum_WASM_I64 as _,
            of: wasm_val_t__bindgen_ty_1 { i64_: 0 },
        };

        let size = data.len();
        let ptr = unsafe {
            wamr_sys::wasm_runtime_module_dup_data(
                self.inst,
                data.as_ptr() as *const _,
                size as u32,
            )
        };
        unsafe {
            wamr_sys::wasm_runtime_call_wasm_v(
                self.env,
                func,
                1,
                ptr::addr_of_mut!(result),
                2,
                ptr,
                size,
            );
            wamr_sys::wasm_runtime_module_free(self.inst, ptr);
        }

        let value = unsafe { result.of.i64_ } as i64;
        let (ptr, len) = (value as u32, (value >> 32) as usize);

        let mut bytes = vec![0u8; len];
        unsafe {
            ptr::copy(
                wamr_sys::wasm_runtime_addr_app_to_native(self.inst, ptr) as *mut u8,
                bytes.as_mut_ptr(),
                len,
            );
            wamr_sys::wasm_runtime_module_free(self.inst, ptr);
        }

        bytes
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            wamr_sys::wasm_runtime_destroy_exec_env(self.env);
            wamr_sys::wasm_runtime_deinstantiate(self.inst);
        }
    }
}
