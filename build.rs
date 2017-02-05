extern crate bindgen;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
  //look for libvlccore in the current directory while in development
  println!("cargo:rustc-link-search=native=.");
  let include_arg = concat!("-I", env!("INCLUDE_DIR"));
  let vlc_common_path = concat!(env!("INCLUDE_DIR"), "/vlc_common.h");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .header(vlc_common_path)
    .whitelisted_type("vlc_fourcc_t")
    .whitelisted_type("mtime_t")
    .whitelisted_type("es_out_id_t")
    .whitelisted_type("libvlc_int_t")
    .whitelisted_type("module_t")
    .whitelisted_type("vlc_object_t")
    .hide_type("demux_t")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/common.rs");
  println!("wrote ffi/common.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_plugin.h"))
    .whitelisted_type("vlc_module_properties")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/plugin.rs");

/*
  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_block.h"))
    //.whitelisted_type("block_t")
    .opaque_type("mtime_t")
    .hide_type("va_list")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::definitions::va_list;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/block.rs");
  */

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_block.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .whitelist_recursively(true)
    .whitelisted_type("block_t")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::mtime_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/block.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_input.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("input_thread_t")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/input.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_stream.h"))
    .hide_type("block_t")
    .hide_type("module_t")
    .hide_type("input_thread_t")
    .hide_type("vlc_object_t")
    .hide_type("va_list")
    .hide_type("libvlc_int_t")
    .whitelist_recursively(true)
    .whitelisted_type("stream_t")
    .whitelisted_function("stream_Read")
    .whitelisted_function("stream_Tell")
    .whitelisted_function("stream_Peek")
    .whitelisted_function("stream_Block")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::block::block_t;")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::input::input_thread_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/stream.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_demux.h"))
    .whitelisted_function("demux_New")
    .whitelisted_function("demux_vaControlHelper")
    .hide_type("stream_t")
    .hide_type("vlc_fourcc_t")
    .hide_type("es_format_t")
    .hide_type("block_t")
    .hide_type("es_out_id_t")
    .hide_type("va_list")
    .hide_type("module_t")
    .hide_type("input_thread_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::stream::stream_t;")
    .raw_line("use ffi::common::es_out_id_t;")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::es::es_format_t;")
    .raw_line("use ffi::block::block_t;")
    .raw_line("use ffi::input::input_thread_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/demux.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_es.h"))
    .whitelisted_function("es_format_Init")
    .whitelisted_type("es_format_category_e")
    .hide_type("vlc_fourcc_t")
    .raw_line("use ffi::common::vlc_fourcc_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/es.rs");

  /*
  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_es_out.h"))
    .whitelisted_type("es_out_query_e")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/es_out.rs");
  */

  let dest_path = Path::new("src/macros.rs");
  let mut f = File::create(&dest_path).unwrap();

  let macros = concat!("#[macro_export]
macro_rules! vlc_module {
  (set_name($name:expr) set_description($desc:expr) set_capability($cap:expr, $score:expr) set_callbacks($open:expr, $close:expr)) => (
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern fn vlc_entry__", env!("VLC_VERSION"), "(vlc_set: unsafe extern fn(*mut c_void, *mut c_void, c_int, ...)
                                  -> c_int,
                                  opaque: *mut c_void) -> c_int {
      let module: *mut c_void = 0 as *mut c_void;

      if vlc_set(opaque, 0 as *mut c_void, $crate::vlc::vlc_module_properties::VLC_MODULE_CREATE as i32,
      &module) != 0 {
        return -1;
      }

      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_NAME as i32,
      concat!($name, \"\\0\").as_ptr()) != 0 {
        return -1;
      }

      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_DESCRIPTION as i32,
      concat!($desc, \"\\0\").as_ptr()) != 0 {
        return -1;
      }

      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_CAPABILITY as i32,
      concat!($cap, \"\\0\").as_ptr()) != 0 {
        return -1;
      }

      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_SCORE as i32, $score) != 0 {
        return -1;
      }

      let p_open: extern \"C\" fn(*mut demux_t<demux_sys_t>) -> c_int =
        transmute($open as extern \"C\" fn(_) -> c_int);
      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_CB_OPEN as i32, p_open) != 0 {
        return -1;
      }

      let p_close: extern \"C\" fn(*mut demux_t<demux_sys_t>) = transmute($close as extern \"C\" fn(_));
      if vlc_set(opaque, module, $crate::vlc::vlc_module_properties::VLC_MODULE_CB_CLOSE as i32, p_close) != 0 {
        return -1;
      }
      0
    }
  );
}
    ");
  f.write_all(macros.as_bytes()).unwrap();
}
