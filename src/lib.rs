#![feature(collections, alloc)]
#![no_std]


extern crate alloc;
extern crate collections;

extern crate gl_context;
extern crate shader;


mod material;


pub use material::Material;
