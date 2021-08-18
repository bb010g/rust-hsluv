use std::{
    error::Error,
    fs::{self, File},
    io::Read,
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=snapshot-rev4.json");

    let out_dir = std::env::var_os("OUT_DIR").expect("No OUT_DIR provided");
    let dest_path = Path::new(&out_dir).join("snapshot-rev4.rs");

    let mut s = String::new();
    File::open("snapshot-rev4.json")
        .expect("Snapshot file not present")
        .read_to_string(&mut s)
        .expect("Can't record snapshot file");
    let json = json::parse(&s).expect("Can't parse snapshot");
    if !json.is_object() {
        panic!("Snapshot isn't an object");
    }

    let mut out = String::new();

    out.push_str("static SNAPSHOT: &'static [(&'static str, ColorValues)] = &[\n");
    for (color, values) in json.entries() {
        let lch = &values["lch"];
        let luv = &values["luv"];
        let rgb = &values["rgb"];
        let xyz = &values["xyz"];
        let hpluv = &values["hpluv"];
        let hsluv = &values["hsluv"];

        let color_values = format!(
            concat!(
                "ColorValues {{ ",
                "lch: Lch {{ lightness: {:?}, chroma: {:?}, hue: {:?} }}, ",
                "luv: Luv {{ lightness: {:?}, u: {:?}, v: {:?} }}, ",
                "rgb: Rgb {{ red: {:?}, green: {:?}, blue: {:?} }}, ",
                "xyz: Xyz {{ x: {:?}, y: {:?}, z: {:?} }}, ",
                "hpluv: Hpluv {{ hue: {:?}, saturation: {:?}, lightness: {:?} }}, ",
                "hsluv: Hsluv {{ hue: {:?}, saturation: {:?}, lightness: {:?} }}, ",
                "}}",
            ),
            lch[0].as_f64().expect("LCH missing lightness"),
            lch[1].as_f64().expect("LCH missing chroma"),
            lch[2].as_f64().expect("LCH missing hue"),
            luv[0].as_f64().expect("LUV missing lightness"),
            luv[1].as_f64().expect("LUV missing u"),
            luv[2].as_f64().expect("LUV missing v"),
            rgb[0].as_f64().expect("RGB missing red"),
            rgb[1].as_f64().expect("RGB missing green"),
            rgb[2].as_f64().expect("RGB missing blue"),
            xyz[0].as_f64().expect("XYZ missing X"),
            xyz[1].as_f64().expect("XYZ missing Y"),
            xyz[2].as_f64().expect("XYZ missing Z"),
            hpluv[0].as_f64().expect("HPLuv missing hue"),
            hpluv[1].as_f64().expect("HPLuv missing saturation"),
            hpluv[2].as_f64().expect("HPLuv missing lightness"),
            hsluv[0].as_f64().expect("HSLuv missing lightness"),
            hsluv[1].as_f64().expect("HSLuv missing lightness"),
            hsluv[2].as_f64().expect("HSLuv missing lightness"),
        );
        out.push_str(&format!("    ({:?}, {}),\n", color, color_values));
    }
    out.push_str("];\n");

    fs::write(&dest_path, out)?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
