// src/lib.rs

#![crate_type = "cdylib"]
#![crate_name = "ichor"]

use std::ffi::c_int;
use std::ffi::c_float;
use std::ffi::c_double;
use std::ffi::c_char;
use std::ffi::CString;
//use num_traits::Num;



/* ALL is just for test for now */

#[repr(C)]
pub struct Node { pub x: f32, pub y: f32 }

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Node { x, y }
    }
}

#[no_mangle]
pub extern "C" fn node_new(x: c_float, y: c_float) -> *mut Node {
    /* The function to create a new node on the Rust side */
    let node = Node::new(x as f32, y as f32);
    Box::into_raw(Box::new(node))
}

#[no_mangle]
pub extern "C" fn node_free(ptr: *mut Node) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn node_string(a: Node) -> *const c_char {
    let nodeString = format!("nodex = {} nodey = {}", a.x, a.y);
    CString::new(nodeString).unwrap().into_raw()
}



