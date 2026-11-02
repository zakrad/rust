//! Regression test for <https://github.com/rust-lang/rust/issues/64521>.
//!
//! Adding an offset to a function address used to fail on wasm32 with
//! "LLVM ERROR: Function addresses with offsets not supported".

//@ only-wasm32
//@ assembly-output: emit-asm
//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]

unsafe extern "C" {
    fn call_guest_fn(f: u32) -> u32;
}

#[no_mangle]
pub fn test_callback() -> u32 {
    42
}

// CHECK-LABEL: call_with_offset:
// CHECK: {{i32.const|global.get}} test_callback
// CHECK-NEXT: i32.const 1
// CHECK-NEXT: i32.add
// CHECK-NEXT: call call_guest_fn
#[no_mangle]
pub fn call_with_offset() {
    unsafe { call_guest_fn(test_callback as fn() -> u32 as usize as u32 + 1) };
}
