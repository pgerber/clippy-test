#![feature(plugin)]
#![plugin(clippy)]
#![feature(const_fn)]


use std::fs::{DirBuilder, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

fn main() {
   // DirBuilder::new()
   //     .recursive(true)
   //     .mode(0o700)
   //     .create("/home/peter")
   //     .unwrap();

   // DirBuilder::new()
   //     .recursive(true)
   //     .mode(0o777)
   //     .create("/home/peter")
   //     .unwrap();

   // #[warn(world_readable_file_mode)]
   // DirBuilder::new()
   //     .recursive(true)
   //     .mode(0o775)
   //     .create("/home/peter")
   //     .unwrap();

   // DirBuilder::new()
   //     .recursive(true)
   //     .mode(0o0)
   //     .create("/home/peter")
   //     .unwrap();

   // DirBuilder::new()
   //     .recursive(true)
   //     .mode(488) // 0o750
   //     .create("/home/peter")
   //     .unwrap();

   // OpenOptions::new().mode(0o777);

   // OpenOptions::new()
   //     .mode(0o77777)
   //     .open("abc")
   //     .unwrap();

   // OpenOptions::new()
   //     .mode(0o4750)
   //     .open("abc")
   //     .unwrap();

   // OpenOptions::new()
   //     .mode(0o2750)
   //     .open("abc")
   //     .unwrap();

   // let mode = 0o2000;
   // OpenOptions::new()
   //     .mode(mode)
   //     .open("abc")
   //     .unwrap();

    const MODE: u32 = 0o4000;
    OpenOptions::new()
        .mode(MODE)
        .open("abc")
        .unwrap();

    OpenOptions::new()
        .mode(get_mode())
        .open("abc")
        .unwrap();

    OpenOptions::new()
        .mode(get_mode_const(0o700))
        .open("abc")
        .unwrap();

    S.mode(0o007);
}

struct S;

impl S {
    fn mode(&mut self, _mode: u32) {}
}

#[inline(always)]
fn get_mode() -> u32 {
    0o3000
}

const fn get_mode_const(base: u32) -> u32 {
    base | 0o2000
}
