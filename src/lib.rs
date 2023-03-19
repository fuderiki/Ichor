// src/lib.rs
use std::os::raw::c_int;
use std::os::raw::c_float;
use num_traits::Num;

#[repr(C)]
pub struct Node<T> {
    pub x: T,
    pub y: T,
}
