#![allow(non_snake_case)]

use libc::{uint8_t, uint64_t, int64_t, size_t, ssize_t, c_int, c_uint, c_char};
use std::os::raw::c_void;
use std::slice::from_raw_parts;

pub use ffi::{vlc_module_properties,vlc_Log,vlc_object_t, va_list, block_t, mtime_t, es_format_t,
                vlc_fourcc_t, es_out_t, es_out_id_t, input_thread_t, module_t, libvlc_int_t};

use ffi::{self, es_format_category_e};
use ffi::stream_t;

pub const VLC_TS_0: mtime_t = 1;

#[macro_export]
macro_rules! vlc_fourcc (
  ($a: expr, $b: expr, $c: expr, $d: expr) => {
    $a as uint32_t | (($b as uint32_t) << 8) |
      (($c as uint32_t) << 16) | (($d as uint32_t) << 24)
  }
);

pub fn stream_Peek<'a>(stream: *mut stream_t, size: size_t) -> &'a[u8] {
  let mut buf = 0 as *const uint8_t;
  unsafe {
    let sz = ffi::vlc_stream_Peek(stream, &mut buf, size);
    // FIXME: what if returned sz is negative? (error)
    if sz > 0 {
      from_raw_parts(buf, sz as usize)
    } else {
      &[]
    }
  }
}

pub fn stream_Read(stream: *mut stream_t, buf: &mut [u8]) -> ssize_t {
  unsafe {
    ffi::vlc_stream_Read(stream, buf.as_mut_ptr() as *mut c_void, buf.len())
  }
}

// FIXME: _stream_Tell and _stream_Seek are not exported by libvlccore, why?
/*
pub fn stream_Tell(stream: *mut stream_t) -> uint64_t {
  unsafe {
    ffi_Tell(stream)
  }
}
*/

pub fn stream_Seek(stream: *mut stream_t, index: uint64_t) -> bool {
  unsafe {
    ffi::vlc_stream_Read(stream, 0 as *mut c_void, index as size_t) == index as ssize_t
  }
}

pub fn stream_Block(stream: *mut stream_t, size: size_t) -> *mut block_t {
  unsafe {
    ffi::vlc_stream_Block(stream, size)
  }
}

pub fn es_format_Init(format: *mut es_format_t, i_cat: es_format_category_e,
                      i_codec: vlc_fourcc_t) {
  unsafe { ffi::es_format_Init(format, i_cat as i32, i_codec) }
}

pub fn es_out_Send(out: *mut es_out_t, id: *mut es_out_id_t, p_block: *mut block_t) -> c_int {
  // FIXME: should not unwrap without checks
  unsafe { ((*out).pf_send.as_ref().unwrap())( out, id, p_block ) }
}

pub fn es_out_Add(out: *mut es_out_t, fmt: *mut es_format_t) -> *mut es_out_id_t {
  unsafe { ((*out).pf_add.as_ref().unwrap())( out, fmt ) }
}

pub fn demux_vaControlHelper(stream: *mut stream_t, i_start: int64_t, i_end: int64_t,
                             i_bitrate: int64_t, i_align: c_int, i_query: c_int,
                             args: va_list) -> c_int {
  unsafe {
    ffi::demux_vaControlHelper(stream, i_start, i_end, i_bitrate, i_align, i_query, args)
  }
}

#[repr(C)]
pub struct demux_t<T> {
  //VLC_COMMON_MEMBERS
  pub psz_object_type: *const c_char,
  pub psz_header:      *mut c_char,
  pub i_flags:         c_int,
  pub b_force:         bool,
  pub p_libvlc:        *mut libvlc_int_t,
  pub p_parent:        *mut vlc_object_t,

  pub p_module:        *mut module_t,

  pub psz_access:      *mut c_char,
  pub psz_demux:       *mut c_char,
  pub psz_location:    *mut c_char,
  pub psz_file:        *mut c_char,

  pub s:               *mut stream_t,
  pub out:             *mut es_out_t,
  pub pf_demux:        Option<extern "C" fn(*mut demux_t<T>) -> c_int>,
  pub pf_control:      Option<extern "C" fn(*mut demux_t<T>, c_int, va_list) -> c_int>,

  // 'info' nested struct. Can we do that with Rust FFI?
  pub i_update:        c_uint,
  pub i_title:         c_int,
  pub i_seekpoint:     c_int,

  //FIXME: p_sys contains a pointer to a module specific structure, make it generic?
  pub p_sys:           *mut T,

  pub p_input:         *mut input_thread_t,
}

pub trait VLCObject {}
impl<T> VLCObject for demux_t<T> {}

use traits::ToC;

pub enum LogType {
  Info,
  Error,
  Warning,
  Debug,
}

pub fn Log<T:VLCObject>(object: &mut T, priority: LogType, module: &[u8], format: &str) {
  unsafe {
    let ptr = object as *mut T;
    vlc_Log(ptr as *mut vlc_object_t, priority.to_c(), module.as_ptr(), format.to_c() as *const uint8_t)
  }
}

#[macro_export]
macro_rules! vlc_Log {
  ($demux:expr, $priority:expr, $module:expr, $format:expr) => {{
    $crate::vlc::Log($demux, $priority, $module, concat!($format, "\0"))
  }};
  ($demux:expr, $priority:expr, $module:expr, $format:expr, $($args:expr),*) => {{
    let formatted = fmt::format(format_args!(concat!($format, "\0"),$($args),*));
    $crate::vlc::Log($demux, $priority, $module, &formatted)
  }};
}
