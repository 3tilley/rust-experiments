#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

macro_rules! m { ($attr_path : path) => { #[$attr_path] fn f() {} } ; }
#[

inline<u8>]
fn f() {}
//~ ERROR: unexpected generic arguments in path

fn main() {}
