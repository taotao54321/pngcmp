use std::env;
use std::path::Path;

use lodepng::{ Bitmap, RGBA };

fn bitmap_is_equal(lhs: Bitmap<RGBA>, rhs: Bitmap<RGBA>) -> bool {
    (lhs.buffer, lhs.width, lhs.height) == (rhs.buffer, rhs.width, rhs.height)
}

enum ExitStatus {
    Same  = 0,
    Diff  = 1,
    Error = 2,
}

fn exit(status: ExitStatus) -> ! {
    std::process::exit(status as i32);
}

macro_rules! error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        exit(ExitStatus::Error);
    });
}

fn read_png_file<P: AsRef<Path>>(path: P) -> Bitmap<RGBA> {
    match lodepng::decode32_file(&path) {
        Ok(bitmap) => bitmap,
        Err(e)     => error!("failed to load '{}': {}", path.as_ref().display(), e),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 { error!("Usage: pngcmp <file1> <file2>"); }

    let bitmap1 = read_png_file(&args[1]);
    let bitmap2 = read_png_file(&args[2]);

    let status = if bitmap_is_equal(bitmap1, bitmap2) {
        ExitStatus::Same
    }
    else {
        ExitStatus::Diff
    };
    exit(status);
}
