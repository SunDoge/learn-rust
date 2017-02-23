extern crate image;

use std::fs::File;
use std::path::Path;

pub static mut X: usize = 1234567890987654321;
pub static mut Y: usize = 362436362436362436;
pub static mut Z: usize = 1066149217761810;
pub static mut C: usize = 123456123456123456;

pub const WIDTH:  u32 = 700;
pub const HEIGHT: u32 = 700;


fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let pixel = img[(0, 0)];

    for x in 0 .. WIDTH {
        for y in 0 .. HEIGHT {
            if kiss() % 2 == 0 {
                img.put_pixel(x, y, pixel);
            }
        }
    }

    let ref mut fout = File::create(&Path::new("static.png")).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}

pub fn kiss() -> usize {
    unsafe {
        Z = 6906969069 * Z + 1234567;
        Y ^= Y << 13;
        Y ^= Y >> 17;
        Y ^= Y << 43;

        let t: usize = (X << 58) + C;
        C = X >> 6;
        X += t;
        C += X << t;
        X + Y + Z
    }
}
