#![allow(non_snake_case)]
use std::{convert::TryInto, mem::MaybeUninit, rc::Rc};

use mrubyedge::yamrb::value::RObject;
use wasm_bindgen::prelude::*;

const MRB: &[u8] = include_bytes!("./mruby/fib.mrb");

static mut MRUBY_VM: MaybeUninit<mrubyedge::yamrb::vm::VM> = MaybeUninit::uninit();
static mut MRUBY_VM_LOADED: bool = false;

fn initVM() {
    let mut rite = mrubyedge::rite::load(MRB).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    vm.run().unwrap();

    unsafe {
        MRUBY_VM = core::mem::MaybeUninit::new(vm);
    }
}

#[allow(static_mut_refs)]
unsafe fn assume_initialized_VM() -> &'static mut mrubyedge::yamrb::vm::VM {
    if !MRUBY_VM_LOADED {
        initVM();
        MRUBY_VM_LOADED = true;
    }
    MRUBY_VM.assume_init_mut()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    let mut vm = unsafe { assume_initialized_VM() };
    let args = vec![Rc::new(RObject::integer(n as i64))];
    let retval: Result<Rc<RObject>, mrubyedge::Error> = 
        mrubyedge::yamrb::helpers::mrb_funcall(
            &mut vm,
            None,
            "fib",
            &args);
    let ret: i64 = match retval {
        Ok(obj) => obj.as_ref().try_into().expect("Failed to convert return value to i64"),
        Err(e) => panic!("{}", e.to_string()),
    };
    ret as u32
}