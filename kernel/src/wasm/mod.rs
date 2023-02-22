mod module_env;
mod func_env;
mod module;
mod tunables;
mod vmoffsets;

/// WebAssembly page sizes are defined to be 64KiB.
pub const WASM_PAGE_SIZE: u32 = 0x10000;

/// The number of pages we can have before we run out of byte index space.
pub const WASM_MAX_PAGES: u32 = 0x10000;

use cranelift_codegen::isa::{self, TargetFrontendConfig};
use cranelift_codegen::settings::{self, Configurable};

use tunables::Tunables;
use module_env::ModuleEnvironment;

pub struct WasmProgram<'data> {
    env: ModuleEnvironment<'data>,
}

impl<'data> WasmProgram<'data> {
    pub fn from_wasm_bytes(name: &str, raw: &'data [u8]) -> Self {
        use core::str::FromStr;

        let target_isa = {
            let shared_builder = settings::builder();
            let shared_flags = settings::Flags::new(shared_builder);
            let isa_builder = isa::lookup(triple!("x86_64-unknown-unknown")).unwrap();
            isa_builder.finish(shared_flags)
        };
        let conf = target_isa.frontend_config();
        let tunables = Tunables::default();
        let mut obj = Self {
            env: ModuleEnvironment::new(conf, tunables)
        };

        cranelift_wasm::translate_module(raw, &mut obj.env);

        debug!("Functions found: {}", obj.env.result().module.functions.len());

        obj
    }
}
