use chrono::Local;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

fn make_sha256() -> String {
    let uuid = Uuid::new_v4();
    let timestamp = Local::now().timestamp_millis();
    let mut rng = thread_rng();
    let random = rng.gen_range(0..=999999);
    let key = format!("{}{}{:06}", uuid, timestamp, random);
    hex::encode(Sha256::digest(key))
}

fn hide_sha256_in_image(sha256: String) {
    let imgx = 4;
    let imgy = 2;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut chars = sha256.chars();
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let r = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
            let g = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
            let b = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
            let a = format!("{}{}", chars.next().unwrap(), chars.next().unwrap());
            let r = u8::from_str_radix(&r, 16).unwrap();
            let g = u8::from_str_radix(&g, 16).unwrap();
            let b = u8::from_str_radix(&b, 16).unwrap();
            let a = u8::from_str_radix(&a, 16).unwrap();
            *pixel = image::Rgba([r, g, b, a]);
        }
    }
    imgbuf.save("./data/sha256.png").unwrap();
}

fn get_sha256_from_image() -> String {
    let img = image::open("./data/sha256.png").unwrap();
    let mut sha256 = String::new();
    let imgbuf = img.to_rgba8();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = imgbuf.get_pixel(x, y);
            let str = format!(
                "{:02x}{:02x}{:02x}{:02x}",
                pixel[0], pixel[1], pixel[2], pixel[3]
            );
            sha256.push_str(&str);
        }
    }
    sha256
}

fn hide_file_in_image(path: &str) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let buf = reader.fill_buf().unwrap();
    let pixel_count = buf.len() / 4;
    let pixel_count = pixel_count as u32;
    let pixel_count = if (buf.len() % 4) == 0 {
        pixel_count
    } else {
        pixel_count + 1
    };
    let width = (pixel_count as f32).sqrt();
    let width = width as u32;
    let mut heigth = width;
    while (width * heigth) < pixel_count {
        heigth += 1;
    }
    let mut imgbuf = image::ImageBuffer::new(width, heigth);
    let mut i_buf = 0;
    for x in 0..width {
        for y in 0..heigth {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let mut rgba = [0u8; 4];
            for i in 0..rgba.len() {
                if i_buf < buf.len() {
                    rgba[i] = buf[i_buf];
                    i_buf += 1;
                }
            }
            *pixel = image::Rgba([rgba[0], rgba[1], rgba[2], rgba[3]]);
        }
    }
    imgbuf.save("./data/file.png").unwrap();
}

#[cfg(test)]
mod tests {
    use dev_util::log::log_init;

    use super::*;

    // cargo test tests::test_make_sha256
    #[test]
    fn test_make_sha256() {
        log_init();
        for _ in 0..1000 {
            let sha256 = make_sha256();
            log::info!("sha256: {}", sha256);
        }
    }
    // cargo test tests::test_hide_sha256_in_image
    #[test]
    fn test_hide_sha256_in_image() {
        log_init();
        let sha256 = make_sha256();
        log::info!("sha256: {}", sha256);
        hide_sha256_in_image(sha256);
    }
    // cargo test tests::test_get_sha256_from_image
    #[test]
    fn test_get_sha256_from_image() {
        log_init();
        let sha256 = make_sha256();
        log::info!("sha256 input : {}", sha256);
        hide_sha256_in_image(sha256);
        let sha256 = get_sha256_from_image();
        log::info!("sha256 output: {}", sha256);
    }

    // cargo test tests::test_hide_file_in_image
    #[test]
    fn test_hide_file_in_image() {
        log_init();
        let path = "./data/private.key";
        hide_file_in_image(path);
    }
}
