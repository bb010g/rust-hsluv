#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RgbBounds {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
impl Rgb {
    pub fn new(red: f64, green: f64, blue: f64) -> Result<Rgb, RgbBounds> {
        if red < 0.0 || red > 1.0 {
            Err(RgbBounds::Red)
        } else if green < 0.0 || green > 1.0 {
            Err(RgbBounds::Green)
        } else if blue < 0.0 || blue > 1.0 {
            Err(RgbBounds::Blue)
        } else {
            Ok(Rgb { red, green, blue })
        }
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HslBounds {
    Hue,
    Saturation,
    Lightness,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hsluv {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}
impl Hsluv {
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hsluv, HslBounds> {
        if hue < 0.0 || hue > 360.0 {
            Err(HslBounds::Hue)
        } else if saturation < 0.0 || saturation > 100.0 {
            Err(HslBounds::Saturation)
        } else if lightness < 0.0 || saturation > 100.0 {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hsluv {
                hue,
                saturation,
                lightness,
            })
        }
    }

    pub fn hsl(&self) -> (f64, f64, f64) {
        (self.hue, self.saturation, self.lightness)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hpluv {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}
impl Hpluv {
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hpluv, HslBounds> {
        if hue < 0.0 || hue > 360.0 {
            Err(HslBounds::Hue)
        } else if saturation < 0.0 || saturation > 100.0 {
            Err(HslBounds::Saturation)
        } else if lightness < 0.0 || saturation > 100.0 {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hpluv {
                hue,
                saturation,
                lightness,
            })
        }
    }

    pub fn hsl(&self) -> (f64, f64, f64) {
        (self.hue, self.saturation, self.lightness)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum XyzBounds {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Result<Xyz, XyzBounds> {
        if x < 0.0 || x > 1.0 {
            Err(XyzBounds::X)
        } else if y < 0.0 || y > 1.0 {
            Err(XyzBounds::Y)
        } else if z < 0.0 || z > 1.0 {
            Err(XyzBounds::Z)
        } else {
            Ok(Xyz { x, y, z })
        }
    }

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

/*
 * http://en.wikipedia.org/wiki/CIELUV
 * In these formulas, Yn refers to the reference white point. We are using
 * illuminant D65, so Yn (see refY in Maxima file) equals 1. The formula is
 * simplified accordingly.
 */

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Luv {
    pub lightness: f64,
    pub u: f64,
    pub v: f64,
}

impl Luv {
    pub fn luv(&self) -> (f64, f64, f64) {
        (self.lightness, self.u, self.v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Lch {
    pub lightness: f64,
    pub chroma: f64,
    pub hue: f64,
}

impl Lch {
    pub fn lch(&self) -> (f64, f64, f64) {
        (self.lightness, self.chroma, self.hue)
    }
}
