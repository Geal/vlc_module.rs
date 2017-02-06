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
    .whitelisted_type("vlc_set_cb")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/plugin.rs");

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
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_access.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .hide_type("input_thread_t")
    .hide_type("va_list")
    .whitelist_recursively(true)
    .whitelisted_type("access_query_e")
    .whitelisted_type("access_t")
    .whitelisted_type("access_fsdir")
    .whitelisted_function("vlc_access_NewMRL")
    .whitelisted_function("vlc_access_Delete")
    .whitelisted_function("vlc_access_Eof")
    .whitelisted_function("vlc_access_Read")
    .whitelisted_function("vlc_access_Block")
    .whitelisted_function("access_vaControl")
    .whitelisted_function("access_Control")
    .whitelisted_function("access_GetSize")
    .whitelisted_function("access_InitFields")
    .whitelisted_function("access_vaDirectoryControlHelper")
    .whitelisted_function("access_fsdir_init")
    .whitelisted_function("access_fsdir_finish")
    .whitelisted_function("access_fsdir_additem")
    .raw_line("use ffi::common::{vlc_object_t,module_t,libvlc_int_t};")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::block::block_t;")
    .raw_line("use ffi::input::input_thread_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/access.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_addons.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("addon_type_t")
    .whitelisted_type("addon_state_t")
    .whitelisted_type("addon_flags_t")
    .whitelisted_type("addon_uuid_t")
    .whitelisted_type("addon_file_t")
    .whitelisted_type("addon_entry_t")
    .whitelisted_type("addons_finder_t")
    .whitelisted_type("addons_finder_sys_t")
    .whitelisted_type("addons_storage_t")
    .whitelisted_type("addons_storage_sys_t")
    .whitelisted_type("addons_manager_t")
    .whitelisted_function("addon_entry_New")
    .whitelisted_function("addon_entry_Hold")
    .whitelisted_function("addon_entry_Release")
    .whitelisted_function("addons_manager_New")
    .whitelisted_function("addons_manager_Delete")
    .whitelisted_function("addons_manager_LoadCatalog")
    .whitelisted_function("addons_manager_Gather")
    .whitelisted_function("addons_manager_Install")
    .whitelisted_function("addons_manager_Remove")
    .whitelisted_function("addons_uuid_read")
    .whitelisted_function("addons_uuid_to_psz")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/addons.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_aout.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .hide_type("mtime_t")
    .hide_type("vlc_fourcc_t")
    .whitelist_recursively(true)
    .whitelisted_type("audio_output")
    .whitelisted_type("audio_filters_t")
    .whitelisted_type("audio_request_vout_t")
    .whitelisted_function("aout_CheckChannelReorder")
    .whitelisted_function("aout_ChannelReorder")
    .whitelisted_function("aout_Interleave")
    .whitelisted_function("aout_Deinterleave")
    .whitelisted_function("aout_CheckChannelExtraction")
    .whitelisted_function("aout_ChannelExtract")
    .whitelisted_function("aout_FormatNbChannels")
    .whitelisted_function("aout_BitsPerSample")
    .whitelisted_function("aout_FormatPrepare")
    .whitelisted_function("aout_FormatPrint")
    .whitelisted_function("aout_FormatPrintChannels")
    .whitelisted_function("aout_VolumeGet")
    .whitelisted_function("aout_VolumeSet")
    .whitelisted_function("aout_MuteGet")
    .whitelisted_function("aout_MuteSet")
    .whitelisted_function("aout_DevicesGet")
    .whitelisted_function("aout_DevicesSet")
    .whitelisted_function("aout_DevicesList")
    .whitelisted_function("aout_VolumeReport")
    .whitelisted_function("aout_MuteReport")
    .whitelisted_function("aout_PolicyReport")
    .whitelisted_function("aout_DeviceReport")
    .whitelisted_function("aout_HotplugReport")
    .whitelisted_function("aout_GainRequest")
    .whitelisted_function("aout_RestartRequest")
    .whitelisted_function("aout_ChannelsRestart")
    .whitelisted_function("aout_FiltersNew")
    .whitelisted_function("aout_FiltersDelete")
    .whitelisted_function("aout_FiltersAdjustResampling")
    .whitelisted_function("aout_FiltersPlay")
    .whitelisted_function("aout_FiltersDrain")
    .whitelisted_function("aout_FiltersFlush")
    .whitelisted_function("aout_filter_RequestVout")
    .whitelisted_var("pi_vlc_chan_order_wg4")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::{mtime_t,vlc_fourcc_t};")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/aout.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_aout_volume.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .hide_type("vlc_fourcc_t")
    .whitelist_recursively(true)
    .whitelisted_type("audio_volume_t")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::vlc_fourcc_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/aout_volume.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_arrays.h"))
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("vlc_array_t")
    .whitelisted_type("vlc_dictionary_entry_t")
    .whitelisted_type("vlc_dictionary_t")
    .whitelisted_function("realloc_down")
    .whitelisted_function("vlc_array_init")
    .whitelisted_function("vlc_array_clear")
    .whitelisted_function("vlc_array_new")
    .whitelisted_function("vlc_array_destroy")
    .whitelisted_function("vlc_array_count")
    .whitelisted_function("vlc_array_item_at_index")
    .whitelisted_function("vlc_array_index_of_item")
    .whitelisted_function("vlc_array_insert")
    .whitelisted_function("vlc_array_append")
    .whitelisted_function("vlc_array_remove")
    .whitelisted_function("DictHash")
    .whitelisted_function("vlc_dictionary_init")
    .whitelisted_function("vlc_dictionary_clear")
    .whitelisted_function("vlc_dictionary_has_key")
    .whitelisted_function("vlc_dictionary_value_for_key")
    .whitelisted_function("vlc_dictionary_keys_count")
    .whitelisted_function("vlc_dictionary_all_keys")
    .whitelisted_function("vlc_dictionary_insert")
    .whitelisted_function("vlc_dictionary_remove_value_for_key")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/arrays.rs");

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
