# Helper library to write VLC modules in Rust

This work was extracted from https://github.com/Geal/rust-vlc-demux to provide general tools to make VLC modules in Rust.

It provides some of the struct definitions, some of the functions exported by libVLCCore, and macros to declare a module and call `vlc_Log`.

it currently requires the content of the `vlc/include` directory available at `include/`
environment variable requirements:

- INCLUDE_DIR points to the VLC include directory
- VLC_VERSION is the version number used for entry points. Set VLC_VERSION=3_0_0a for a vlc_entry__3_0_0a
