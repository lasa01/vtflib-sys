# vtflib-sys

FFI bindings for VTFLib.

`pkg-config` is used to find VTFLib.
Defaults to dynamic linking, static linking can be enabled
either with the `static` feature or with the `VTFLIB_STATIC` environment variable.
If linking statically, keep in mind that VTFLib is LGPL-licensed.

If the library is not found and static linking is enabled, VTFLib is automatically built.
This requires cmake and a C++ compiler.

The library path can be overridden with the environment variable `VTFLIB_PATH` or `VTFLIB13_PATH`.
