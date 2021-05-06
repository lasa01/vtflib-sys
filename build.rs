fn main() {
    let mut build_config = cmake::Config::new("vendor");
    build_config.define("BUILD_SHARED_LIBS", "OFF");
    build_config.pic(true);
    // todo: a feature for this
    build_config.define("USE_LIBTXC_DXTN", "OFF");
    build_config.define("USE_NVDXT", "OFF");

    let dest = build_config.build().join("lib");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=VTFLib13");
}
