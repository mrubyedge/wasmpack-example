use std::path::Path;

extern crate mruby_compiler2_sys;

fn main() {
    // Rebuild mruby bytecode if the source file changes
    println!("cargo:rerun-if-changed=src/mruby/fib.rb");

    let input = Path::new("src/mruby/fib.rb");
    let output = Path::new("src/mruby/fib.mrb");
    let code = std::fs::read_to_string(input).expect("Failed to read mruby source file");

    unsafe {
        let mut ctx = mruby_compiler2_sys::MRubyCompiler2Context::new();
        ctx.compile_to_file(&code, output)
            .expect("Failed to compile mruby script");
    }
}
