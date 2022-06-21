use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct ColorStruct {
    colorId: u32,
    hexString: String,
    rgb: RGBStruct,
    hsl: HSLStruct,
}

#[derive(Deserialize)]
struct RGBStruct {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Deserialize)]
struct HSLStruct {
    h: f32,
    s: f32,
    l: f32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let find = args.iter().position(|x| x.eq("--hex"));
    match find {
        Some(i) => {
            find_similar(args[i + 1].clone());
        }
        None => {}
    }
}

fn find_similar(hex: String) {
    let color_data = fs::read_to_string("256-colors.json").expect("Could not read file");
    let colors: Vec<ColorStruct> =
        serde_json::from_str(&color_data).expect("JSON was not well-formatted");
    let mut scored: Vec<(i16, ColorStruct)> = Vec::new();
    let mut hex_iter = hex.chars();
    hex_iter.next();

    let mut temp = String::new();
    temp.push(hex_iter.next().unwrap());
    temp.push(hex_iter.next().unwrap());
    let r = hex::decode(temp).unwrap();

    temp = String::new();
    temp.push(hex_iter.next().unwrap());
    temp.push(hex_iter.next().unwrap());
    let g = hex::decode(temp).unwrap();

    temp = String::new();
    temp.push(hex_iter.next().unwrap());
    temp.push(hex_iter.next().unwrap());
    let b = hex::decode(temp).unwrap();

    println!("{:?} {:?} {:?}", r, g, b);

    let rgb = RGBStruct {
        r: r[0],
        g: g[0],
        b: b[0]
    };

    for color in colors {
        scored.push((compute_diff(&rgb, &color), color));
    }

    scored.sort_by_key(|k| k.0);

    println!("1. {}: {}", scored[0].1.colorId, scored[0].1.hexString);
    println!("2. {}: {}", scored[1].1.colorId, scored[1].1.hexString);
    println!("3. {}: {}", scored[2].1.colorId, scored[2].1.hexString);
}

fn compute_diff(rgb: &RGBStruct, color: &ColorStruct) -> i16 {
    let r = (i16::from(rgb.r) - i16::from(color.rgb.r)).abs();
    let g = (i16::from(rgb.g) - i16::from(color.rgb.g)).abs();
    let b = (i16::from(rgb.b) - i16::from(color.rgb.b)).abs();

    return r + g + b;
}