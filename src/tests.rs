use super::*;

#[derive(Debug, Clone)]
struct ColorValues {
    lch: Lch,
    luv: Luv,
    rgb: Rgb,
    xyz: Xyz,
    hpluv: Hpluv,
    hsluv: Hsluv,
}

include!(concat!(env!("OUT_DIR"), "/snapshot-rev4.rs"));

const MAX_DIFF: f64 = 0.00000001;

fn check_eq(color: &str, format: &'static str, expected: f64, actual: f64) {
    if Float::abs(expected - actual) < MAX_DIFF {
        ()
    } else {
        panic!(
            "{} {}: expected {}, got {}",
            color, format, expected, actual
        )
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
