//! CLI tool to generate QR-Code from text

use std::{env, process::{Command, exit}};

use qrcodegen::{QrCode, QrCodeEcc};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut iter = args.iter().skip(1);
    let mut text: Option<String> = None;
    let mut output: Option<String> = None;
    loop {
        match iter.next() {
            Some(v) if v == "-o" => output = Some(iter.next().expect("missing output path").to_string()),
            Some(v) => text = Some(v.to_string()),
            _ => break
        }
    }
    
    if text.is_none() {
        if args.len() <= 1 {
            println!("error: missing input\n\nUSAGE: \tqr-code <text> [-o <path>]");
            exit(1);
        }
    }

    let outpath = output.unwrap_or("qr-code-output.png".to_string());
    let _ = args.iter().skip(1).next().map(|text| generate_qrcode(text, &outpath));
}

fn generate_qrcode(text: &str, outpath: &str) {
    // Simple operation
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).unwrap();
    let svg = qr.to_svg_string(4);

    let tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(200, 200).unwrap();
    resvg::render(&tree, usvg::FitTo::Width(200), pixmap.as_mut()).unwrap();
    pixmap.save_png(outpath).expect("failed to generate PNG file");

    // open if on macos
    if env::consts::OS.to_lowercase() == "macos" {
        let _ = Command::new("/usr/bin/open")
            .arg(outpath)
            .output();
    } else {
        println!("output to '{}'", outpath);
    }
}
