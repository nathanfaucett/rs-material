#![feature(alloc)]
#![feature(collections)]
#![no_std]


extern crate alloc;
extern crate collections;

extern crate shared;
extern crate gl_context;
extern crate shader;


mod material;


pub use material::Material;
