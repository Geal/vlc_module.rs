extern crate libbindgen;

fn main() {
  //look for libvlccore in the current directory while in development
  println!("cargo:rustc-link-search=native=.");
  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_common.h")
    .whitelisted_type("vlc_fourcc_t")
    .whitelisted_type("mtime_t")
    .whitelisted_type("es_out_id_t")
    .hide_type("demux_t")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/common.rs");

  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_plugin.h")
    .whitelisted_type("vlc_module_properties")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/plugin.rs");

/*
  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_block.h")
    //.whitelisted_type("block_t")
    .opaque_type("mtime_t")
    .hide_type("va_list")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::definitions::va_list;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/block.rs");
  */


  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_stream.h")
    .whitelisted_type("stream_t")
    .whitelisted_function("stream_Read")
    .whitelisted_function("stream_Tell")
    .whitelisted_function("stream_Peek")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/stream.rs");

  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_demux.h")
    .whitelisted_function("demux_New")
    .whitelisted_function("demux_vaControlHelper")
    .hide_type("stream_t")
    .hide_type("vlc_fourcc_t")
    .hide_type("es_format_t")
    .hide_type("block_t")
    .hide_type("es_out_id_t")
    .hide_type("va_list")
    .raw_line("use ffi::definitions::va_list;")
    .raw_line("use ffi::stream::stream_t;")
    .raw_line("use ffi::common::{vlc_fourcc_t,es_out_id_t};")
    .raw_line("use ffi::es::es_format_t;")
    .raw_line("use ffi::definitions::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/demux.rs");

  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_es.h")
    .whitelisted_function("es_format_Init")
    .whitelisted_type("es_format_category_e")
    .hide_type("vlc_fourcc_t")
    .raw_line("use ffi::common::vlc_fourcc_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/es.rs");

  let _ = libbindgen::builder()
    .clang_arg("-I include/")
    .header("include/vlc_es_out.h")
    .whitelisted_type("es_out_query_e")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/es_out.rs");
}
