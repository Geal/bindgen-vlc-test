build with:

```
INCLUDE_DIR=`pwd`/include cargo build
```

Expected: file `src/stream.rs` generated with definitions for stream_t, stream_Read, stream_Tell, stream_Peek

Output:

```
error: failed to run custom build command for `bindgen-vlc-test v0.1.0 (file:///Users/geal/dev/videolan/bindgen-vlc-test)`
process didn't exit successfully: `/Users/geal/dev/videolan/bindgen-vlc-test/target/debug/build/bindgen-vlc-test-b23f79ec58e723dd/build-script-build` (exit code: 101)
--- stdout
cargo:rustc-link-search=native=.
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:101:31: error: unknown type name 'block_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:105:5: error: must use 'struct' tag to refer to type 'block_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:107:5: error: unknown type name 'uint8_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:109:5: error: unknown type name 'uint8_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:112:5: error: unknown type name 'uint32_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:115:5: error: unknown type name 'mtime_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:116:5: error: unknown type name 'mtime_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:117:5: error: unknown type name 'mtime_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:139:1: error: unknown type name 'VLC_API', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:139:9: error: expected identifier or '(', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:1: error: unknown type name 'VLC_API', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:16: error: expected ';' after top level declarator, err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:1: error: must use 'struct' tag to refer to type 'block_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:27: error: must use 'struct' tag to refer to type 'block_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:64: error: expected function body after function declarator, err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:142:1: error: unknown type name 'VLC_API', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:142:16: error: expected ';' after top level declarator, err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:144:42: error: must use 'struct' tag to refer to type 'block_t', err: true
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:144:56: error: must use 'struct' tag to refer to type 'block_t', err: true
fatal error: too many errors emitted, stopping now [-ferror-limit=], err: true

--- stderr
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:101:31: error: unknown type name 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:105:5: error: must use 'struct' tag to refer to type 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:107:5: error: unknown type name 'uint8_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:109:5: error: unknown type name 'uint8_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:112:5: error: unknown type name 'uint32_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:115:5: error: unknown type name 'mtime_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:116:5: error: unknown type name 'mtime_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:117:5: error: unknown type name 'mtime_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:139:1: error: unknown type name 'VLC_API'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:139:9: error: expected identifier or '('
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:1: error: unknown type name 'VLC_API'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:16: error: expected ';' after top level declarator
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:1: error: must use 'struct' tag to refer to type 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:9: note: struct 'block_t' is hidden by a non-type declaration of 'block_t' here
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:27: error: must use 'struct' tag to refer to type 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:9: note: struct 'block_t' is hidden by a non-type declaration of 'block_t' here
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:141:64: error: expected function body after function declarator
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:142:1: error: unknown type name 'VLC_API'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:142:16: error: expected ';' after top level declarator
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:144:42: error: must use 'struct' tag to refer to type 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:9: note: struct 'block_t' is hidden by a non-type declaration of 'block_t' here
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:144:56: error: must use 'struct' tag to refer to type 'block_t'
/Users/geal/dev/videolan/bindgen-vlc-test/vlc_block.h:140:9: note: struct 'block_t' is hidden by a non-type declaration of 'block_t' here
fatal error: too many errors emitted, stopping now [-ferror-limit=]
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ()', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libcore/result.rs:870
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
