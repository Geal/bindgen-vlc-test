extern crate bindgen;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
  //look for libvlccore in the current directory while in development
  println!("cargo:rustc-link-search=native=.");
  let include_arg = concat!("-I", env!("INCLUDE_DIR"));


  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_stream.h"))
    .hide_type("block_t")
    .whitelist_recursively(true)
    .opaque_type("block_t")
    .whitelisted_type("stream_t")
    .whitelisted_function("stream_Read")
    .whitelisted_function("stream_Tell")
    .whitelisted_function("stream_Peek")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/stream.rs");
}
