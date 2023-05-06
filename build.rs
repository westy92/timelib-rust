use std::{
    fs::File,
    process::{Command, Stdio},
};

fn main() {
    // run re2c on 2 files
    re2c("parse_date");
    re2c("parse_iso_intervals");

    println!("cargo:rustc-link-lib=m");

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
        .flag("-O0")
        .flag("-ggdb3")
        .flag("-Wall")
        //.flag("-Werror")
        .flag("-Wempty-body")
        .flag("-Wenum-compare")
        .flag("-Wformat-nonliteral")
        .flag("-Wformat-security")
        .flag("-Wimplicit-fallthrough")
        .flag("-Winit-self")
        .flag("-Wlogical-not-parentheses")
        //.flag("-Wlogical-op")
        //.flag("-Wmaybe-uninitialized")
        .flag("-Wmissing-field-initializers")
        .flag("-Wmissing-format-attribute")
        .flag("-Wno-unused-parameter")
        .flag("-Wparentheses")
        .flag("-Wpointer-arith")
        .flag("-Wshadow")
        .flag("-Wsign-compare")
        .flag("-Wsizeof-array-argument")
        .flag("-Wunused-but-set-variable")
        .flag("-Wvariadic-macros")
        .flag("-Wwrite-strings")
        .flag("-fdiagnostics-show-option")
        .flag("-fno-exceptions")
        .flag("-fno-omit-frame-pointer")
        .flag("-fno-optimize-sibling-calls")
        //.flag("-fsanitize=address")
        //.flag("-fsanitize=undefined")
        .flag("-fstack-protector")
        .flag("-pedantic")
        .define("HAVE_STDINT_H", None)
        .define("HAVE_GETTIMEOFDAY", None)
        .define("HAVE_UNISTD_H", None)
        .define("HAVE_DIRENT_H", None)
        .define("HAVE_STDINT_H", None)
        .define("HAVE_STDINT_H", None);

    if !std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // extra parameters to use in non-Windows
        build = build
            .flag("-Wextra");
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
