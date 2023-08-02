use std::{env, path::PathBuf};

#[cfg(feature = "re2c")]
use std::{
    fs::File,
    process::{Command, Stdio},
};

use bindgen::builder;

fn main() {
    let bindings = builder()
        .header("ext/timelib/timelib.h")
        .allowlist_var("TIMELIB_ZONETYPE_ID")
        .allowlist_var("TIMELIB_NO_CLONE")
        .allowlist_function("timelib_builtin_db")
        .allowlist_function("timelib_error_container_dtor")
        .allowlist_function("timelib_fill_holes")
        .allowlist_function("timelib_parse_tzfile")
        .allowlist_function("timelib_strtotime")
        .allowlist_function("timelib_time_ctor")
        .allowlist_function("timelib_time_dtor")
        .allowlist_function("timelib_tzinfo_dtor")
        .allowlist_function("timelib_unixtime2local")
        .allowlist_function("timelib_update_ts")
        .header("shim/shim.h")
        .allowlist_function("timelib_tz_get_wrapper_cached")
        .generate()
        .expect("failed to run bindgen");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir.clone());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings.rs");

    // run re2c on 2 files
    #[cfg(feature = "re2c")]
    re2c("parse_date", &out_path);
    #[cfg(feature = "re2c")]
    re2c("parse_iso_intervals", &out_path);

    let src = [
        "ext/timelib/astro.c",
        "ext/timelib/dow.c",
        "ext/timelib/interval.c",
        "ext/timelib/parse_posix.c",
        "ext/timelib/parse_tz.c",
        "ext/timelib/parse_zoneinfo.c",
        "ext/timelib/timelib.c",
        "ext/timelib/tm2unixtime.c",
        "ext/timelib/unixtime2tm.c",
        // files generated from re2c:
        #[cfg(feature = "re2c")]
        &format!("{}/parse_date.c", out_dir.clone()),
        #[cfg(feature = "re2c")]
        &format!("{}/parse_iso_intervals.c", out_dir.clone()),
        #[cfg(not(feature = "re2c"))]
        "pregenerated/parse_date.c",
        #[cfg(not(feature = "re2c"))]
        "pregenerated/parse_iso_intervals.c",
        "shim/shim.c",
    ];

    let mut builder = cc::Build::new();
    let mut build = builder
        .files(src.iter())
        .include("ext/timelib")
        .include("ext/hashmap.h")
        // taken from Makefile
        .flag("-Wall")
        .define("HAVE_STDINT_H", None)
        .define("HAVE_GETTIMEOFDAY", None);

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        build = build
            .define("HAVE_DIRENT_H", Some("0"))
            .define("HAVE_UNISTD_H", Some("0"));
    } else {
        // extra parameters to use in non-Windows
        println!("cargo:rustc-link-lib=m");

        build = build
            .flag("-O0")
            .flag("-ggdb3")
            .flag("-fdiagnostics-show-option")
            .flag("-fno-exceptions")
            .flag("-fno-omit-frame-pointer")
            .flag("-fno-optimize-sibling-calls")
            //.flag("-fsanitize=address")
            //.flag("-fsanitize=undefined")
            .flag("-fstack-protector")
            .flag("-pedantic")
            .define("HAVE_DIRENT_H", None)
            .define("HAVE_UNISTD_H", None);
    }

    build.compile("timelib");
}

#[cfg(feature = "re2c")]
fn re2c(file: &str, out_path: &PathBuf) {
    let target_file = File::create(out_path.join(format!("{file}.c"))).unwrap();
    let stdio = Stdio::from(target_file);
    let mut cmd = Command::new("re2c")
        .current_dir("ext/timelib")
        .arg("-d")
        .arg("-b")
        .arg(format!("{file}.re"))
        .stdout(stdio)
        .spawn()
        .expect("re2c command failed to start");
    cmd.wait()
        .expect(format!("failed to generate {file}.c").as_str());
}
