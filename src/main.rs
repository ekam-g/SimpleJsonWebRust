#![feature(proc_macro_hygiene, decl_macro)]


mod https;
use crate::https::{Start, Web};


fn main() {
    Web{}.start();
}
