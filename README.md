# pngcmp - compare two PNG images exactly

This utility works like `cmp` command. It takes two PNG files, and
exits with a return code indicating whether two bitmaps are exactly
the same.

`pngcmp` always extract 32-bit RGBA bitmaps from PNG files, and
compare only the bitmaps. It is insensitive to color format, metadata,
etc.

## Usage

```sh
$ pngcmp foo.png bar.png
```

| return code | meaning               |
| :---------: | -------               |
| 0           | bitmaps are the same  |
| 1           | bitmaps are different |
| 2           | error                 |
