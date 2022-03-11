#![allow(non_upper_case_globals)]

use nix::unistd::{geteuid, Uid};
use std::io::{self, Write};
use std::path::PathBuf;

const cyellow: &[u8; 9] = b"\x1B[1m\x1B[33m"; // set_color yellow --bold
const cred: &[u8; 9] = b"\x1B[1m\x1B[31m"; // set_color red --bold
const cgreen: &[u8; 9] = b"\x1B[1m\x1B[32m"; // set_color green --bold
const cwhite: &[u8; 9] = b"\x1B[1m\x1B[37m"; // set_color white --bold
const cnormal: &[u8; 11] = b"\x1B[30m\x1B(B\x1B[m"; // set_color normal

static mut out: Vec<u8> = Vec::new();

fn color(c: &[u8]) {
    unsafe {
        out.extend_from_slice(c);
    }
}

fn status(s: String) {
    if let Ok(status) = s.parse::<i64>() {
        if status == 0 {
            return;
        }
        color(cred);
        unsafe {
            let _ = write!(&mut out, "{} ", status);
        }
    }
}

fn cmd_duration(s: String) {
    if let Ok(msecs) = s.parse::<u64>() {
        let mut i = msecs;

        if i < 100 {
            return;
        }

        let ms: u64 = i % 1000;
        i /= 1000;
        let seconds = i % 60;
        i /= 60;
        let minutes = i % 60;
        i /= 60;
        let hours = i % 24;
        i /= 24;
        let days: u64 = i;
        let mut full: bool = false;

        color(cyellow);

        unsafe {
            let _ = write!(out, "[");
        }

        if days > 0 {
            full = true;
            unsafe {
                let _ = write!(out, "{}d", days);
            }
        }

        if full || hours > 0 {
            full = true;
            unsafe {
                let _ = write!(out, "{:02}h", hours);
            }
        }

        if full || minutes > 0 {
            unsafe {
                let _ = write!(out, "{:02}m", minutes);
            }
        }

        unsafe {
            let _ = write!(out, "{:02}.{:03}s] ", seconds, ms);
        }
    }
}

fn cwd(uid: Uid) {
    use std::env;
    use std::os::unix::ffi::OsStrExt;
    use std::str::Chars;

    let mut cwd: PathBuf = match env::current_dir() {
        Ok(_cwd) => _cwd, //.to_string_lossy().to_string(),
        Err(_) => return,
    };
    if uid.is_root() {
        color(cred);
    } else {
        color(cgreen);
    }

    if let Some(home) = env::var_os("HOME") {
        let home: PathBuf = PathBuf::from(home);
        if cwd.starts_with(home.as_path()) {
            let c = cwd.strip_prefix(home).unwrap().to_path_buf();
            unsafe {
                let _ = write!(out, "~");
            }
            cwd = PathBuf::from("/");
            cwd.push(c);
        }
    };

    let dir: String = String::from_utf8_lossy(cwd.as_os_str().as_bytes()).to_string();
    let mut s = dir.split('/');

    while let Some(d) = s.next() {
        if let Some(c) = d.chars().next() {
            unsafe {
                let _ = write!(out, "/{}", c);
            }
        }
    }

    if let Some(i) = dir.rfind('/') {
        let mut rest: Chars = dir[i..].chars();
        let _ = rest.next(); // '/'
        let _ = rest.next(); // first char is already written

        unsafe {
            let _ = write!(out, "{}", rest.collect::<String>());
        }
    }
}

fn prompt(uid: Uid) {
    color(cwhite);
    if uid.is_root() {
        unsafe {
            let _ = write!(out, " # ");
        }
    } else {
        unsafe {
            let _ = write!(out, "> ");
        }
    }
    color(cnormal);
}

fn main() {
    unsafe {
        out.reserve(32768);
    }
    let mut args = std::env::args();
    let _ = args.next();
    if let Some(st) = args.next() {
        status(st);
    }
    if let Some(d) = args.next() {
        cmd_duration(d);
    }

    let uid: Uid = geteuid();
    cwd(uid);
    prompt(uid);

    let sl: &[u8] = unsafe { out.as_slice() };
    let _ = io::stdout().write(sl);
    let _ = io::stdout().flush();
}
