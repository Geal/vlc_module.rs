extern crate bindgen;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
  //look for libvlccore in the current directory while in development
  println!("cargo:rustc-link-search=native=.");
  let include_arg = concat!("-I", env!("INCLUDE_DIR"));
  let vlc_common_path = concat!(env!("INCLUDE_DIR"), "/vlc_common.h");

  /*
  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .header(vlc_common_path)
    //.enable_cxx_namespaces()
    .layout_tests(false)
    .whitelisted_type("mtime_t")
    .whitelisted_type("vlc_fourcc_t")
    .whitelisted_type("vlc_fourcc_to_char")
    .whitelisted_type("vlc_list_t")
    .whitelisted_type("vlc_object_t")
    .whitelisted_type("libvlc_int_t")
    .whitelisted_type("date_t")
    .whitelisted_type("module_t")
    .whitelisted_type("module_config_t")
    .whitelisted_type("vlc_value_t")
    .whitelisted_type("vlc_common_members")
    .whitelisted_function("GCD")
    .whitelisted_function("clip_uint8_vlc")
    .whitelisted_function("clz")
    .whitelisted_function("popcount")
    .whitelisted_function("popcountll")
    .whitelisted_type("parity")
    .whitelisted_type("bswap16")
    .whitelisted_type("bswap32")
    .whitelisted_type("bswap64")
    .whitelisted_type("U16_AT")
    .whitelisted_type("U32_AT")
    .whitelisted_type("U64_AT")
    .whitelisted_type("GetWLE")
    .whitelisted_type("GetDWLE")
    .whitelisted_type("GetQWLE")
    .whitelisted_type("SetWBE")
    .whitelisted_type("SetDWBE")
    .whitelisted_type("SetQWBE")
    .whitelisted_type("SetWLE")
    .whitelisted_type("SetDWLE")
    .whitelisted_type("SetQWLE")
    .whitelisted_type("vlc_rational_t")
    .whitelisted_function("vlc_memalign")
    .whitelisted_function("vlc_gettext")
    .whitelisted_function("vlc_ngettext")
    .hide_type("demux_t")
    .hide_type("es_out_t")
    .hide_type("es_out_id_t")
    .hide_type("es_format_t")
    .hide_type("va_list")
    .hide_type("__va_list_tag")
    .hide_type("input_thread_t")
    .hide_type("stream_t")
    .hide_type("block_t")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::es_out::{es_out_t, es_out_id_t};")
    .raw_line("use ffi::input::input_thread_t;")
    .raw_line("use ffi::stream::stream_t;")
    .raw_line("use ffi::es::es_format_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/common.rs");
  println!("wrote ffi/common.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_access.h"))
    .layout_tests(false)
    .whitelist_recursively(true)
    .whitelisted_function("vlc_access_NewMRL")
    .whitelisted_function("access_vaDirectoryControlHelper")
    .whitelisted_function("access_fsdir_init")
    .whitelisted_function("access_fsdir_finish")
    .whitelisted_function("access_fsdir_additem")
    .whitelisted_type("access_fsdir")
    .hide_type("vlc_common_members ")
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .hide_type("input_thread_t")
    .hide_type("va_list")
    .hide_type("__va_list_tag")
    .hide_type("stream_t")
    .hide_type("mtime_t")
    .hide_type("vlc_fourcc_t")
    .hide_type("es_out_t")
    .hide_type("es_out_id_t")
    .hide_type("es_format_t")
    .hide_type("vlc_stream_Block")
    .hide_type("vlc_stream_Read")
    .hide_type("vlc_stream_Peek")
    .hide_type("vlc_Log")
    .raw_line("use ffi::common::{vlc_object_t,vlc_fourcc_t,module_t,libvlc_int_t,mtime_t};")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::es_out::{es_out_t, es_out_id_t};")
    .raw_line("use ffi::es::{es_format_t};")
    .raw_line("use ffi::block::block_t;")
    .raw_line("use ffi::input::input_thread_t;")
    .raw_line("use ffi::stream::stream_t;")
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
    //.enable_cxx_namespaces()
    .layout_tests(false)
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
    .whitelisted_type("addons_manager_owner")
    .whitelisted_type("addons_manager_private_t")
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
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .hide_type("mtime_t")
    .hide_type("vlc_fourcc_t")
    .whitelist_recursively(true)
    .whitelisted_type("audio_output")
    .whitelisted_type("pi_vlc_chan_order_wg4")
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
    .layout_tests(false)
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
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("vlc_array_t")
    .whitelisted_type("vlc_dictionary_entry_t")
    .whitelisted_type("vlc_dictionary_t")
    .whitelisted_type("kVLCDictionaryNotFound")
    .whitelisted_function("realloc_down")
    .whitelisted_function("vlc_array_init")
    .whitelisted_function("vlc_array_clear")
    .whitelisted_function("vlc_array_count")
    .whitelisted_function("vlc_array_item_at_index")
    .whitelisted_function("vlc_array_index_of_item")
    .whitelisted_function("vlc_array_insert")
    .whitelisted_function("vlc_array_append")
    .whitelisted_function("vlc_array_remove")
    .whitelisted_function("vlc_array_new")
    .whitelisted_function("vlc_array_destroy")
    .whitelisted_function("DictHash")
    .whitelisted_function("vlc_dictionary_init")
    .whitelisted_function("vlc_dictionary_clear")
    .whitelisted_function("vlc_dictionary_has_key")
    .whitelisted_function("vlc_dictionary_value_for_key")
    .whitelisted_function("vlc_dictionary_keys_count")
    .whitelisted_function("vlc_dictionary_all_keys")
    .whitelisted_function("vlc_dictionary_insert")
    .whitelisted_function("vlc_dictionary_remove_value_for_key")
    .whitelisted_function("vlc_delete_all")
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
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_atomic.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("atomic_flag")
    .whitelisted_type("atomic_bool")
    .whitelisted_type("atomic_char")
    .whitelisted_type("atomic_schar")
    .whitelisted_type("atomic_uchar")
    .whitelisted_type("atomic_short")
    .whitelisted_type("atomic_ushort")
    .whitelisted_type("atomic_int")
    .whitelisted_type("atomic_uint")
    .whitelisted_type("atomic_long")
    .whitelisted_type("atomic_ulong")
    .whitelisted_type("atomic_llong")
    .whitelisted_type("atomic_ullong")
    .whitelisted_type("atomic_wchar_t")
    .whitelisted_type("atomic_int_least8_t")
    .whitelisted_type("atomic_8int_least8_t")
    .whitelisted_type("atomic_int_least16_t")
    .whitelisted_type("atomic_uint_least16_t")
    .whitelisted_type("atomic_int_least32_t")
    .whitelisted_type("atomic_uint_least32_t")
    .whitelisted_type("atomic_int_least64_t")
    .whitelisted_type("atomic_uint_least64_t")
    .whitelisted_type("atomic_int_fast8_t")
    .whitelisted_type("atomic_uint_fast8_t")
    .whitelisted_type("atomic_int_fast16_t")
    .whitelisted_type("atomic_uint_fast16_t")
    .whitelisted_type("atomic_int_fast32_t")
    .whitelisted_type("atomic_uint_fast32_t")
    .whitelisted_type("atomic_int_fast64_t")
    .whitelisted_type("atomic_uint_fast64_t")
    .whitelisted_type("atomic_intptr_t")
    .whitelisted_type("atomic_uintptr_t")
    .whitelisted_type("atomic_size_t")
    .whitelisted_type("atomic_ptrdiff_t")
    .whitelisted_type("atomic_intmax_t")
    .whitelisted_type("atomic_uintmax_t")
    .whitelisted_type("vlc_atomic_float")
    .whitelisted_function("vlc_atomic_init_float")
    .whitelisted_function("vlc_atomic_load_flot")
    .whitelisted_function("vlc_atomic_store_float")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/atomic.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_avcodec.h"))
    .layout_tests(false)
    .whitelist_recursively(true)
    .whitelisted_function("vlc_avcodec_lock")
    .whitelisted_function("vlc_avcodec_unlock")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/avcodec.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_bits.h"))
    .layout_tests(false)
    //.whitelist_recursively(true)
    .whitelisted_type("bs_t")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_function("bs_write_init")
    .whitelisted_function("bs_init")
    .whitelisted_function("bs_pos")
    .whitelisted_function("bs_remain")
    .whitelisted_function("bs_read")
    .whitelisted_function("bs_read1")
    .whitelisted_function("bs_show")
    .whitelisted_function("bs_skip")
    .whitelisted_function("bs_write")
    .whitelisted_function("bs_aligned")
    .whitelisted_function("bs_align")
    .whitelisted_function("bs_align_0")
    .whitelisted_function("bs_align_1")
    .whitelisted_function("bs_read_ue")
    .whitelisted_function("bs_read_se")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/bits.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_block.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .whitelist_recursively(true)
    .whitelisted_type("block_t")
    .whitelisted_type("block_Alloc")
    .whitelisted_type("block_TryRealloc")
    .whitelisted_type("block_Realloc")
    .whitelisted_type("block_Release")
    .whitelisted_type("block_CopyProperties")
    .whitelisted_type("block_Duplicate")
    .whitelisted_type("block_heap_Alloc")
    .whitelisted_type("block_mmap_Alloc")
    .whitelisted_type("block_shm_Alloc")
    .whitelisted_type("block_File")
    .whitelisted_type("block_FilePath")
    .whitelisted_type("block_FifoNew")
    .whitelisted_type("block_FifoRelease")
    .whitelisted_type("block_FifoEmpty")
    .whitelisted_type("block_FifoPut")
    .whitelisted_type("block_FifoGet")
    .whitelisted_type("block_FifoShow")
    .whitelisted_type("vlc_fifo_t")
    .whitelisted_type("vlc_fifo_DequeueUnlocked")
    .whitelisted_type("vlc_fifo_DequeueAllUnlocked")
    .whitelisted_function("block_Init")
    .whitelisted_function("block_Cleanup")
    .whitelisted_function("block_ChainAppend")
    .whitelisted_function("block_ChainLastAppend")
    .whitelisted_function("block_ChainRelease")
    .whitelisted_function("block_ChainExtract")
    .whitelisted_function("block_ChainProperties")
    .whitelisted_function("block_ChainGather")
    .whitelisted_function("block_FifoSize")
    .whitelisted_function("block_FifoCount")
    .whitelisted_function("vlc_fifo_Lock")
    .whitelisted_function("vlc_fifo_Unlock")
    .whitelisted_function("vlc_fifo_Signal")
    .whitelisted_function("vlc_fifo_Wait")
    .whitelisted_function("vlc_fifo_WaitCond")
    .whitelisted_function("vlc_fifo_TimedWaitCond")
    .whitelisted_function("vlc_fifo_QueueUnlocked")
    .whitelisted_function("vlc_fifo_GetCount")
    .whitelisted_function("vlc_fifo_GetBytes")
    .whitelisted_function("vlc_fifo_IsEmpty")
    .whitelisted_function("vlc_fifo_Cleanup")
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
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_block_helper.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("block_bytestream_t")
    .whitelisted_type("block_startcode_helper_t")
    .whitelisted_type("block_startcode_matcher_t")
    .whitelisted_function("block_BytestreamInit")
    .whitelisted_function("block_BytestreamRelease")
    .whitelisted_function("block_BytestreamEmpty")
    .whitelisted_function("block_BytestreamFlush")
    .whitelisted_function("block_BytestreamPush")
    .whitelisted_function("block_BytestreamRemaining")
    .whitelisted_function("block_BytestreamPop")
    .whitelisted_function("block_WaitBytes")
    .whitelisted_function("block_PeekBytes")
    .whitelisted_function("block_GetBytes")
    .whitelisted_function("block_SkipBytes")
    .whitelisted_function("block_SkipByte")
    .whitelisted_function("block_PeekOffsetBytes")
    .whitelisted_function("block_FindStartcodeFromOffset")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/block_helper.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_boxes.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("bo_t")
    .whitelisted_function("bo_init")
    .whitelisted_function("bo_deinit")
    .whitelisted_function("bo_free")
    .whitelisted_function("bo_extend")
    .whitelisted_function("bo_set8")
    .whitelisted_function("bo_add8")
    .whitelisted_function("bo_add24be")
    .whitelisted_function("bo_swap_32be")
    .whitelisted_function("bo_add_mem")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/boxes.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_charset.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("vlc_iconv_t")
    .whitelisted_function("vlc_towc")
    .whitelisted_function("IsUTF8")
    .whitelisted_function("EnsureUTF8")
    .whitelisted_function("vlc_iconv_open")
    .whitelisted_function("vlc_iconv")
    .whitelisted_function("vlc_iconv_close")
    .whitelisted_function("utf8_vfprintf")
    .whitelisted_function("utf8_fprintf")
    .whitelisted_function("vlc_strcasestr")
    .whitelisted_function("FromCharset")
    .whitelisted_function("ToCharset")
    .whitelisted_function("FromWide")
    .whitelisted_function("ToWide")
    .whitelisted_function("ToCodePage")
    .whitelisted_function("FromCodePage")
    .whitelisted_function("FromANSI")
    .whitelisted_function("ToANSI")
    .whitelisted_function("FromLatin1")
    .whitelisted_function("us_strtod")
    .whitelisted_function("us_strtof")
    .whitelisted_function("us_atof")
    .whitelisted_function("us_vasprintf")
    .whitelisted_function("us_asprintf")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/charset.rs");
  */
  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_codec.h"))
    .layout_tests(false)
    .hide_type("module_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .hide_type("block_t")
    .whitelist_recursively(true)
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_type("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .whitelisted_function("")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::common::mtime_t;")
    .raw_line("use ffi::block::block_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/codec.rs");
  /*
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
    .hide_type("__va_list_tag")
    .hide_type("libvlc_int_t")
    .hide_type("mtime_t")
    .whitelist_recursively(true)
    .whitelisted_type("stream_t")
    .whitelisted_function("vlc_stream_Read")
    .whitelisted_function("vlc_stream_Tell")
    .whitelisted_function("vlc_stream_Peek")
    .whitelisted_function("vlc_stream_Block")
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
    .hide_type("__va_list_tag")
    .hide_type("module_t")
    .hide_type("input_thread_t")
    .hide_type("vlc_object_t")
    .hide_type("libvlc_int_t")
    .hide_type("es_out_t")
    .hide_type("mtime_t")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::stream::stream_t;")
    .raw_line("use ffi::common::vlc_object_t;")
    .raw_line("use ffi::common::vlc_fourcc_t;")
    .raw_line("use ffi::common::module_t;")
    .raw_line("use ffi::common::libvlc_int_t;")
    .raw_line("use ffi::es::es_format_t;")
    .raw_line("use ffi::block::block_t;")
    .raw_line("use ffi::input::input_thread_t;")
    .raw_line("use ffi::es_out::{es_out_t, es_out_id_t};")
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
    .whitelisted_type("es_format_t")
    .hide_type("vlc_fourcc_t")
    .raw_line("use ffi::common::vlc_fourcc_t;")
    .use_core()
    .generate().unwrap()
    .write_to_file("src/ffi/es.rs");

  let _ = bindgen::builder()
    .clang_arg(include_arg)
    .clang_arg("-include")
    .clang_arg(vlc_common_path)
    .header(concat!(env!("INCLUDE_DIR"), "/vlc_es_out.h"))
    .whitelisted_type("es_out_t")
    .whitelisted_type("es_out_query_e")
    .whitelisted_type("es_out_id_t")
    .hide_type("es_format_t")
    .hide_type("block_t")
    .hide_type("va_list")
    .hide_type("__va_list_tag")
    .raw_line("use ffi::definitions::{va_list,__va_list_tag};")
    .raw_line("use ffi::es::es_format_t;")
    .raw_line("use ffi::block::block_t;")
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
