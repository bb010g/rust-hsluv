use super::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct ColorValues {
    lch: Lch,
    luv: Luv,
    rgb: Rgb,
    xyz: Xyz,
    hpluv: Hpluv,
    hsluv: Hsluv,
}

lazy_static! {
    static ref SNAPSHOT: HashMap<String, ColorValues> = snapshot_init("snapshot-rev4.json");
}

fn snapshot_init(path: &str) -> HashMap<String, ColorValues> {
    let mut s = String::new();
    File::open(path)
        .expect("Snapshot file not present")
        .read_to_string(&mut s)
        .expect("Can't record snapshot file");
    let json = json::parse(&s).expect("Can't parse snapshot");
    if !json.is_object() {
        panic!("Snapshot isn't an object");
    }
    let mut out = HashMap::new();
    for (color, values) in json.entries() {
        let lch = &values["lch"];
        let luv = &values["luv"];
        let rgb = &values["rgb"];
        let xyz = &values["xyz"];
        let hpluv = &values["hpluv"];
        let hsluv = &values["hsluv"];

        out.insert(
            color.to_owned(),
            ColorValues {
                lch: Lch {
                    lightness: lch[0].as_f64().expect("LCH missing lightness"),
                    chroma: lch[1].as_f64().expect("LCH missing chroma"),
                    hue: lch[2].as_f64().expect("LCH missing hue"),
                },
                luv: Luv {
                    lightness: luv[0].as_f64().expect("LUV missing lightness"),
                    u: luv[1].as_f64().expect("LUV missing u"),
                    v: luv[2].as_f64().expect("LUV missing v"),
                },
                rgb: Rgb {
                    red: rgb[0].as_f64().expect("RGB missing red"),
                    green: rgb[1].as_f64().expect("RGB missing green"),
                    blue: rgb[2].as_f64().expect("RGB missing blue"),
                },
                xyz: Xyz {
                    x: xyz[0].as_f64().expect("XYZ missing X"),
                    y: xyz[1].as_f64().expect("XYZ missing Y"),
                    z: xyz[2].as_f64().expect("XYZ missing Z"),
                },
                hpluv: Hpluv {
                    hue: hpluv[0].as_f64().expect("HPLuv missing hue"),
                    saturation: hpluv[1].as_f64().expect("HPLuv missing saturation"),
                    lightness: hpluv[2].as_f64().expect("HPLuv missing lightness"),
                },
                hsluv: Hsluv {
                    hue: hsluv[0].as_f64().expect("HSLuv missing lightness"),
                    saturation: hsluv[1].as_f64().expect("HSLuv missing lightness"),
                    lightness: hsluv[2].as_f64().expect("HSLuv missing lightness"),
                },
            },
        );
    }
    out
}

const MAX_DIFF: f64 = 0.00000001;

fn check_eq(color: &str, format: &'static str, expected: f64, actual: f64) {
    if f64::abs(expected - actual) < MAX_DIFF {
        ()
    } else {
        panic!(format!(
            "{} {}: expected {}, got {}",
            color, format, expected, actual
        ))
    }
}

#[test]
fn test_rgb_to_lch() {
    for (color, values) in SNAPSHOT.iter() {
        let xyz = Xyz::from(values.rgb);
        check_eq(color, "Xyz.x", values.xyz.x, xyz.x);
        check_eq(color, "Xyz.y", values.xyz.y, xyz.y);
        check_eq(color, "Xyz.z", values.xyz.z, xyz.z);

        let luv = Luv::from(xyz);
        check_eq(color, "Luv.lightness", values.luv.lightness, luv.lightness);
        check_eq(color, "Luv.u", values.luv.u, luv.u);
        check_eq(color, "Luv.v", values.luv.v, luv.v);

        let lch = Lch::from(luv);
        check_eq(color, "Lch.lightness", values.lch.lightness, lch.lightness);
        check_eq(color, "Lch.chroma", values.lch.chroma, lch.chroma);
        check_eq(color, "Lch.hue", values.lch.hue, lch.hue);
    }
}

#[test]
fn test_lch_to_rgb() {
    for (color, values) in SNAPSHOT.iter() {
        let luv = Luv::from(values.lch);
        check_eq(color, "Luv.lightness", values.luv.lightness, luv.lightness);
        check_eq(color, "Luv.u", values.luv.u, luv.u);
        check_eq(color, "Luv.v", values.luv.v, luv.v);

        let xyz = Xyz::from(luv);
        check_eq(color, "Xyz.x", values.xyz.x, xyz.x);
        check_eq(color, "Xyz.y", values.xyz.y, xyz.y);
        check_eq(color, "Xyz.z", values.xyz.z, xyz.z);

        let rgb = Rgb::from(xyz);
        check_eq(color, "Rgb.red", values.rgb.red, rgb.red);
        check_eq(color, "Rgb.green", values.rgb.green, rgb.green);
        check_eq(color, "Rgb.blue", values.rgb.blue, rgb.blue);
    }
}

#[test]
fn test_lch_to_hsluv() {
    for (color, values) in SNAPSHOT.iter() {
        let hsluv = Hsluv::from(values.lch);
        check_eq(color, "Hsluv.hue", values.hsluv.hue, hsluv.hue);
        check_eq(
            color,
            "Hsluv.saturation",
            values.hsluv.saturation,
            hsluv.saturation,
        );
        check_eq(
            color,
            "Hsluv.lightness",
            values.hsluv.lightness,
            hsluv.lightness,
        );
    }
}

#[test]
fn test_lch_to_hpluv() {
    for (color, values) in SNAPSHOT.iter() {
        let hpluv = Hpluv::from(values.lch);
        check_eq(color, "Hpluv.hue", values.hpluv.hue, hpluv.hue);
        check_eq(
            color,
            "Hpluv.saturation",
            values.hpluv.saturation,
            hpluv.saturation,
        );
        check_eq(
            color,
            "Hpluv.lightness",
            values.hpluv.lightness,
            hpluv.lightness,
        );
    }
}

#[test]
fn test_hsluv_to_lch() {
    for (color, values) in SNAPSHOT.iter() {
        let lch = Lch::from(values.hsluv);
        check_eq(color, "Lch.lightness", values.lch.lightness, lch.lightness);
        check_eq(color, "Lch.chroma", values.lch.chroma, lch.chroma);
        check_eq(color, "Lch.hue", values.lch.hue, lch.hue);
    }
}

#[test]
fn test_hpluv_to_lch() {
    for (color, values) in SNAPSHOT.iter() {
        let lch = Lch::from(values.hpluv);
        check_eq(color, "Lch.lightness", values.lch.lightness, lch.lightness);
        check_eq(color, "Lch.chroma", values.lch.chroma, lch.chroma);
        check_eq(color, "Lch.hue", values.lch.hue, lch.hue);
    }
}
