//@ edition: 2021
// regression test for #119316: a closure inside an array-length constant in a
// generic function used to ICE with "expected type of closure to be a closure";
// it should instead emit ordinary errors.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

async fn foo<const N: usize>() {
    let _data = &mut [0u8; { N + (|| 42)() }];
    //~^ ERROR overly complex generic constant
    //~| ERROR cannot call non-const closure in constants
}

fn main() {}
