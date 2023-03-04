fn main() {
    abi_code_gen();
}

struct AbiValue {
    name: String,
    ty: String,
}

struct AbiCall {
    name: String,
    args: Vec<AbiValue>,
    rets: Vec<AbiValue>,
}

/// Generates code for the WASI abi
fn abi_code_gen() {
    static ABI_TRAIT_CODE: &'static str = include_str!("src/wasm/abi_trait.rs");
    let mut abi_calls: Vec<AbiCall> = Vec::new();
    for line in ABI_TRAIT_CODE.lines() {
        let line = line.trim().replace("async ", "");
        if line.starts_with("fn ") {
            let mut line = line[3..].to_string();
            if line.contains("{") {
                line = line.split("{").next().unwrap().to_string();
                line.push('@');
            }
            let line = line
                .replace(" ", "")
                .replace("->", "|")
                .replace(");", "|")
                .replace(")@", "|")
                .replace("(&", "|&")
                .replace(")|", "|")
                .replace("&self,", "");
            let name_args_ret_split: Vec<&str> = line.split("|").collect();
            let args_split: Vec<Vec<&str>> = name_args_ret_split[1]
                .split(",")
                .map(|s| s.split(":").collect::<Vec<&str>>())
                .collect();

            let name = name_args_ret_split[0].to_string();
            let name_stripped = name.split("<").collect::<Vec<&str>>()[0].to_string();
            let mut call = AbiCall {
                name: name_stripped,
                args: Vec::new(),
                rets: Vec::new(),
            };
            for arg in args_split {
                if arg.len() < 2 { continue; }
                let mut name = arg[0].to_string();
                if name.starts_with("_") {
                    name.remove(0);
                }
                let ty = arg[1].to_string();
                call.args.push(AbiValue {
                    name,
                    ty,
                });
            }
            abi_calls.push(call);
        }
    }

    let mut generated_code = String::new();
    generated_code.push_str("vec![\n");
    for call in abi_calls {
        let mut gen_args = String::new();
        let mut gen_args_def = String::new();
        if call.args.len() > 0 {
            gen_args_def.push_str(", ");
        }
        for arg in &call.args {
            let name = &arg.name;
            let ty = &arg.ty;
            gen_args.push_str(&format!("{name}, "));
            gen_args_def.push_str(&format!("{name}: {ty}, "));
        }
        gen_args.pop();
        gen_args.pop();
        gen_args_def.pop();
        gen_args_def.pop();
        let name = &call.name;
        let generated_call = format!(r#"AbiFunc::wrap("{name}", store, |mut caller: Caller<'_, ()>{gen_args_def}| self.{name}({gen_args})),"#);
        generated_code.push('\t');
        generated_code.push_str(&generated_call);
        generated_code.push('\n');
    }
    generated_code.push(']');
    std::fs::write("src/wasm/code_gen.rs", generated_code.bytes().collect::<Vec<u8>>());
}
