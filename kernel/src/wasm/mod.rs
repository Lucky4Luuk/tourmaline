mod module_env;
mod func_env;
mod module;
mod tunables;
mod vmoffsets;
mod address_map;
mod compilation;
mod cranelift;

/// WebAssembly page sizes are defined to be 64KiB.
pub const WASM_PAGE_SIZE: u32 = 0x10000;

/// The number of pages we can have before we run out of byte index space.
pub const WASM_MAX_PAGES: u32 = 0x10000;

use cranelift_codegen::isa::{self, TargetFrontendConfig};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_codegen::binemit::RelocSink;

use tunables::Tunables;
use module_env::ModuleEnvironment;
use compilation::{Compiler, Program};
use cranelift::Cranelift;

pub struct WasmProgram {
    prog: Program,
}

impl WasmProgram {
    pub fn from_wasm_bytes(name: &str, raw: &[u8]) -> Self {
        use core::str::FromStr;

        let target_isa = {
            let shared_builder = settings::builder();
            let shared_flags = settings::Flags::new(shared_builder);
            let isa_builder = isa::lookup(triple!("x86_64-unknown-unknown")).unwrap();
            isa_builder.finish(shared_flags)
        };
        let conf = target_isa.frontend_config();
        let tunables = Tunables::default();

        let (module, function_body_inputs) = {
            let mut env = ModuleEnvironment::new(conf, tunables);

            cranelift_wasm::translate_module(raw, &mut env);

            info!("Functions found: {}", env.result().module.functions.len());

            (env.result.module, env.result.function_body_inputs)
        };

        let result = Cranelift::compile_module(
            &module,
            function_body_inputs,
            &*target_isa,
            false,
        ).expect("Failed to compile!");

        Self {
            prog: Program::from_tuple(result, module.start_func.clone()),
        }
    }

    pub unsafe fn run_directly(self) -> ! {
        use core::arch::asm;
        asm!("jmp {value}", value = in(reg) self.prog.entry_point().unwrap().as_u64());
        unreachable!()
    }
}
