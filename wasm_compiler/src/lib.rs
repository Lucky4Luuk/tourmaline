use cranelift_module::ModuleDeclarations;
use cranelift_wasm::{
    WasmResult,
    ModuleEnvironment,
    WasmFuncType,
    TypeIndex,
};

pub struct ModEnv {
    mod_decl: ModuleDeclarations,
}

impl<'data> ModuleEnvironment<'data> for ModEnv {
    fn declare_type_func(&mut self, wasm_func_type: WasmFuncType) -> WasmResult<()> { todo!() }
    fn declare_func_import(&mut self, index: TypeIndex, modules: &'a data str, field: &'data str) -> WasmResult<()> { todo!() }
}

pub fn compile_wasm(wasm: String) {

}
