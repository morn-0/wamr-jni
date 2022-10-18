use std::sync::Mutex;

static __: Mutex<bool> = Mutex::new(false);

pub struct Runtime;

impl Runtime {
    pub fn init() {
        let mut lock = __.lock().unwrap();

        if !*lock {
            unsafe {
                *lock = wamr_sys::wasm_runtime_init();
            }
        }
    }

    pub fn destroy() {
        let mut lock = __.lock().unwrap();

        if *lock {
            unsafe {
                wamr_sys::wasm_runtime_destroy();
            }
        }

        *lock = false;
    }
}
