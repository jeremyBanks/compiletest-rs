// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use common::Config;
use std::ffi::OsStr;
use std::path::PathBuf;

/// Conversion table from triple OS name to Rust SYSNAME
const OS_TABLE: &'static [(&'static str, &'static str)] = &[
    ("android", "android"),
    ("bitrig", "bitrig"),
    ("darwin", "macos"),
    ("dragonfly", "dragonfly"),
    ("freebsd", "freebsd"),
    ("haiku", "haiku"),
    ("ios", "ios"),
    ("linux", "linux"),
    ("mingw32", "windows"),
    ("netbsd", "netbsd"),
    ("openbsd", "openbsd"),
    ("win32", "windows"),
    ("windows", "windows"),
    ("solaris", "solaris"),
    ("emscripten", "emscripten"),
];

const ARCH_TABLE: &'static [(&'static str, &'static str)] = &[
    ("aarch64", "aarch64"),
    ("amd64", "x86_64"),
    ("arm", "arm"),
    ("arm64", "aarch64"),
    ("hexagon", "hexagon"),
    ("i386", "x86"),
    ("i586", "x86"),
    ("i686", "x86"),
    ("mips", "mips"),
    ("msp430", "msp430"),
    ("nvptx64", "nvptx64"),
    ("powerpc", "powerpc"),
    ("powerpc64", "powerpc64"),
    ("s390x", "s390x"),
    ("sparc", "sparc"),
    ("x86_64", "x86_64"),
    ("xcore", "xcore"),
    ("asmjs", "asmjs"),
    ("wasm32", "wasm32"),
];

pub fn matches_os(triple: &str, name: &str) -> bool {
    // For the wasm32 bare target we ignore anything also ignored on emscripten
    // and then we also recognize `wasm32-bare` as the os for the target
    if triple == "wasm32-unknown-unknown" {
        return name == "emscripten" || name == "wasm32-bare"
    }
    for &(triple_os, os) in OS_TABLE {
        if triple.contains(triple_os) {
            return os == name;
        }
    }
    false
}
pub fn get_arch(triple: &str) -> &str {
    for &(triple_arch, arch) in ARCH_TABLE {
        if triple.contains(triple_arch) {
            return arch;
        }
    }
    triple.split('-').nth(0).unwrap()
}

pub fn get_env(triple: &str) -> Option<&str> {
    triple.split('-').nth(3)
}

pub fn get_pointer_width(triple: &str) -> &'static str {
    if (triple.contains("64") && !triple.ends_with("gnux32")) || triple.starts_with("s390x") {
        "64bit"
    } else {
        "32bit"
    }
}

pub fn make_new_path(path: &str) -> String {
    assert!(cfg!(windows));
    // Windows just uses PATH as the library search path, so we have to
    // maintain the current value while adding our own
    match env::var(lib_path_env_var()) {
        Ok(curr) => format!("{}{}{}", path, path_div(), curr),
        Err(..) => path.to_owned(),
    }
}

pub fn lib_path_env_var() -> &'static str {
    "PATH"
}
fn path_div() -> &'static str {
    ";"
}

pub fn logv(config: &Config, s: String) {
    debug!("{}", s);
    if config.verbose {
        println!("{}", s);
    }
}

pub trait PathBufExt {
    /// Append an extension to the path, even if it already has one.
    fn with_extra_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf;
}

impl PathBufExt for PathBuf {
    fn with_extra_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf {
        if extension.as_ref().is_empty() {
            self.clone()
        } else {
            let mut fname = self.file_name().unwrap().to_os_string();
            if !extension.as_ref().to_str().unwrap().starts_with('.') {
                fname.push(".");
            }
            fname.push(extension);
            self.with_file_name(fname)
        }
    }
}
