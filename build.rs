use std::{
    env,
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
        .allowlist_function("timelib_date_to_int")
        .allowlist_function("timelib_error_container_dtor")
        .allowlist_function("timelib_fill_holes")
        .allowlist_function("timelib_parse_tzfile")
        .allowlist_function("timelib_strtotime")
        .allowlist_function("timelib_time_ctor")
        .allowlist_function("timelib_time_dtor")
        .allowlist_function("timelib_tzinfo_dtor")
        .allowlist_function("timelib_unixtime2local")
        .allowlist_function("timelib_update_ts")
        .generate()
        .expect("failed to run bindgen");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("failed to write bindings.rs");

    // run re2c on 2 files
    re2c("parse_date");
    re2c("parse_iso_intervals");

    let src = [
        "ext/timelib/astro.c",
        "ext/timelib/dow.c",
        "ext/timelib/interval.c",
        "ext/timelib/parse_date.c",
        "ext/timelib/parse_iso_intervals.c",
        "ext/timelib/parse_posix.c",
        "ext/timelib/parse_tz.c",
        "ext/timelib/parse_zoneinfo.c",
        "ext/timelib/timelib.c",
        "ext/timelib/tm2unixtime.c",
        "ext/timelib/unixtime2tm.c",
    ];

    let mut builder = cc::Build::new();
    let mut build = builder
        .files(src.iter())
        .include("ext/timelib")
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

fn re2c(file: &str) {
    let target_file = File::create(format!("ext/timelib/{file}.c")).unwrap();
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
