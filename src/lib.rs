use chrono::Local;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use uuid::Uuid;

fn make_sha256() -> String {
    let uuid = Uuid::new_v4();
    let timestamp = Local::now().timestamp_millis();
    let mut rng = thread_rng();
    let random = rng.gen_range(0..=999999);
    let key = format!("{}{}{:06}", uuid, timestamp, random);
    hex::encode(Sha256::digest(key))
}

fn hide_sha256_in_image(sha256: String) -> String {
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
    let output = "./data/sha256.png";
    imgbuf.save(output).unwrap();
    output.to_string()
}

fn get_sha256_from_image(path: &str) -> String {
    let img = image::open(path).unwrap();
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

fn hide_file_in_image(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let buf = reader.fill_buf().unwrap();
    log::info!("src:\n{:?}", &buf);
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
    let output = "./data/file.png";
    imgbuf.save(output).unwrap();
    output.to_string()
}

// 四角长度，外边儿数据，绕着放
fn hide_file_in_logo(file: &str, logo: &str) -> String {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let file_buf = reader.fill_buf().unwrap();
    let file_buf_len = format!("{:08x}", file_buf.len());
    let point_00 = &file_buf_len[0..2];
    let point_0w = &file_buf_len[2..4];
    let point_h0 = &file_buf_len[4..6];
    let point_hw = &file_buf_len[6..8];
    log::info!(
        "file buf size: {} {} {} {}",
        point_00,
        point_0w,
        point_h0,
        point_hw
    );
    let logo_img = image::open(logo).unwrap();
    let mut logo_buf = logo_img.to_rgba8();
    let w = logo_buf.width();
    let h = logo_buf.height();
    // 左上角
    let pixel = logo_buf.get_pixel_mut(0, 0);
    let a = u8::from_str_radix(&point_00, 16).unwrap();
    *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], a]);

    // 右上角
    let pixel = logo_buf.get_pixel_mut(0, w - 1);
    let a = u8::from_str_radix(&point_0w, 16).unwrap();
    *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], a]);

    // 左下角
    let pixel = logo_buf.get_pixel_mut(h - 1, 0);
    let a = u8::from_str_radix(&point_h0, 16).unwrap();
    *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], a]);

    // 右下角
    let pixel = logo_buf.get_pixel_mut(h - 1, w - 1);
    let a = u8::from_str_radix(&point_hw, 16).unwrap();
    *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], a]);

    let w = logo_buf.width();
    let h = logo_buf.height();

    let mut file_buf_i = 0;
    // 共计25圈
    'for_25: for i in 0..25 {
        // 上边，左-->右
        let y = i;
        for x in i..=(w - 1 - i) {
            if file_buf_i >= file_buf.len() {
                break 'for_25;
            }
            let pixel = logo_buf.get_pixel_mut(x, y);
            *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], file_buf[file_buf_i]]);
            file_buf_i += 1;
            // log::info!("({:03}, {:03})", x, y);
        }

        // 右边，上-->下
        let x = w - 1 - i;
        for y in (1 + i)..=(h - 1 - i) {
            if file_buf_i >= file_buf.len() {
                break 'for_25;
            }
            let pixel = logo_buf.get_pixel_mut(x, y);
            *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], file_buf[file_buf_i]]);
            file_buf_i += 1;
            // log::info!("({:03}, {:03})", x, y);
        }

        // 下边，右-->左
        let y = h - 1 - i;
        for x in i..=(w - 2 - i) {
            let x = w - 2 - x;
            if file_buf_i >= file_buf.len() {
                break 'for_25;
            }
            let pixel = logo_buf.get_pixel_mut(x, y);
            *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], file_buf[file_buf_i]]);
            file_buf_i += 1;
            // log::info!("({:03}, {:03})", x, y);
        }

        // 左边，下-->上
        let x = i;
        for y in (1 + i)..=(h - 2 - i) {
            let y = h - 1 - y;
            if file_buf_i >= file_buf.len() {
                break 'for_25;
            }
            let pixel = logo_buf.get_pixel_mut(x, y);
            *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], file_buf[file_buf_i]]);
            file_buf_i += 1;
            // log::info!("({:03}, {:03})", x, y);
        }
    }
    let output = "./data/file_in_logo.png";
    logo_buf.save(output).unwrap();
    output.to_string()
}

fn get_file_from_image(path: &str) -> String {
    let img = image::open(path).unwrap();
    let mut filebuf = Vec::new();
    let imgbuf = img.to_rgba8();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = imgbuf.get_pixel(x, y);
            filebuf.push(pixel[0]);
            filebuf.push(pixel[1]);
            filebuf.push(pixel[2]);
            filebuf.push(pixel[3]);
        }
    }
    while filebuf[filebuf.len() - 1] == 0 {
        filebuf.remove(filebuf.len() - 1);
    }
    log::info!("get:\n{:?}", &filebuf);
    let output = "./data/file_src.png";
    let mut writer = BufWriter::new(File::create(output).unwrap());
    writer.write_all(&filebuf).unwrap();
    output.to_string()
}

fn read_image(path: &str) {
    let img = image::open(path).unwrap();
    let imgbuf = img.to_rgba8();
    let mut pixel_map = HashMap::new();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = imgbuf.get_pixel(x, y);
            let pixel_key = format!("{},{},{},{}", pixel[0], pixel[1], pixel[2], pixel[3]);
            if x < 25 || x >= 475 || y < 25 || y >= 475 {
                log::info!("({},{}) rgbr: [{}]", x, y, &pixel_key);
                let count = match pixel_map.get(&pixel_key) {
                    Some(count) => *count + 1,
                    None => 1,
                };
                pixel_map.insert(pixel_key, count);
            }
        }
    }
    for (key, count) in pixel_map {
        log::info!("{}--{}", key, count);
    }
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
        let path = hide_sha256_in_image(sha256);
        log::info!("hide in file: {}", path);
    }
    // cargo test tests::test_get_sha256_from_image
    #[test]
    fn test_get_sha256_from_image() {
        log_init();
        let sha256 = make_sha256();
        log::info!("sha256 input : {}", sha256);
        let path = hide_sha256_in_image(sha256);
        log::info!("hide in file: {}", &path);
        let sha256 = get_sha256_from_image(&path);
        log::info!("sha256 output: {}", sha256);
    }

    // cargo test tests::test_hide_file_in_image
    #[test]
    fn test_hide_file_in_image() {
        log_init();
        let path = hide_file_in_image("./data/sha256.png");
        log::info!("hide in file: {}", &path);
    }
    // cargo test tests::test_get_file_from_image
    #[test]
    fn test_get_file_from_image() {
        log_init();
        let sha256 = make_sha256();
        log::info!("sha256 input : {}", sha256);
        let path = hide_sha256_in_image(sha256);
        log::info!("hide sha256 in file: {}", &path);
        let path = hide_file_in_image(&path);
        log::info!("hide file in path: {}", &path);
        let path = get_file_from_image(&path);
        log::info!("get file: {}", &path);
        let sha256 = get_sha256_from_image(&path);
        log::info!("get    sha256: {}", sha256);
    }

    // cargo test tests::test_it_works
    #[test]
    fn test_it_works() {
        log_init();
        let path = "./src/lib.rs";
        log::info!("hide file in image: {}", &path);
        let path = hide_file_in_image(&path);
        log::info!("hide file in path: {}", &path);
        let path = get_file_from_image(&path);
        log::info!("get file: {}", &path);
    }

    // cargo test tests::test_read_image
    #[test]
    fn test_read_image() {
        log_init();
        let path = "./data/cyberkl.png";
        log::info!("image: {}", &path);
        read_image(&path);
    }

    // cargo test tests::test_hide_file_in_logo
    #[test]
    fn test_hide_file_in_logo() {
        log_init();
        let logo = "./data/logo.png";
        log::info!("logo: {}", &logo);
        let file = "./src/lib.rs";
        log::info!("file: {}", &file);
        hide_file_in_logo(file, logo);
    }
}
