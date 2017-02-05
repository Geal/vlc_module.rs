use libc::{uint8_t, uint32_t, uint64_t, size_t, ssize_t, c_void, c_int, c_uint};

use ffi::stream::stream_t;
use ffi::common::{vlc_object_t,mtime_t};

pub type __va_list_tag = c_void;
pub type va_list = *mut __va_list_tag;

/*
#[derive(Debug,Clone,Copy)]
#[repr(C)]
pub struct block_t {
  pub p_next:       *mut block_t,
  pub p_buffer:     *mut uint8_t,
  pub i_buffer:     size_t,
  pub p_start:      *mut uint8_t,
  pub i_size:       size_t,
  pub i_flags:      uint32_t,
  pub i_nb_samples: c_uint,
  pub i_pts:        mtime_t,
  pub i_dts:        mtime_t,
  pub i_length:     mtime_t,
  pub pf_release:   Option<unsafe extern "C" fn(*mut block_t)>,
}
*/

#[cfg_attr(feature = "link-vlc", link(name = "vlccore"))]
extern {
  /*
  pub fn stream_Peek(stream: *mut stream_t, buf: *mut *const uint8_t, size: size_t) -> ssize_t;

  // https://www.videolan.org/developers/vlc/doc/doxygen/html/group__stream.html
  pub fn stream_Read(stream: *mut stream_t, buf: *const c_void, size: size_t) -> ssize_t;
  // https://www.videolan.org/developers/vlc/doc/doxygen/html/group__stream.html
  pub fn stream_Tell(stream: *mut stream_t) -> uint64_t;

  pub fn stream_Block(stream: *mut stream_t, size: size_t) -> *mut block_t;
  */

  pub fn vlc_Log(obj: *mut vlc_object_t, priority: c_int, module: *const uint8_t, format: *const uint8_t, ...);
}
