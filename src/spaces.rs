/// Error indicating which component of an [`Rgb`] triplet was out of bounds.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RgbBounds {
	/// Red component was out of bounds
    Red,
	/// Green component was out of bounds
    Green,
	/// Blue component was out of bounds
    Blue,
}

/// Point in the RGB color space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rgb {
	/// Red component, between 0.0 and 1.0 (inclusive).
    pub red: f64,
	/// Green component, between 0.0 and 1.0 (inclusive).
    pub green: f64,
	/// Blue component, between 0.0 and 1.0 (inclusive).
    pub blue: f64,
}
impl Rgb {
	/// Create a new [`Rgb`] while checking the ranges of the values.
    pub fn new(red: f64, green: f64, blue: f64) -> Result<Rgb, RgbBounds> {
        if !(0.0..=1.0).contains(&red) {
            Err(RgbBounds::Red)
        } else if !(0.0..=1.0).contains(&green) {
            Err(RgbBounds::Green)
        } else if !(0.0..=1.0).contains(&blue) {
            Err(RgbBounds::Blue)
        } else {
            Ok(Rgb { red, green, blue })
        }
    }

    /// Return the RGB components as tuple.
    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }
}

/// Error indicating which parameter of an hue-saturation-value triplet was out of bounds.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HslBounds {
	/// Hue parameter was out of bounds
    Hue,
	/// Saturation parameter was out of bounds
    Saturation,
	/// Lightness parameter was out of bounds
    Lightness,
}

/// Point in the HSLuv color space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hsluv {
	/// Hue component, between 0.0 and 360.0 (inclusive)
    pub hue: f64,
	/// Saturation component, between 0.0 and 100.0 (inclusive)
    pub saturation: f64,
	/// Lightness component, between 0.0 and 100.0 (inclusive)
    pub lightness: f64,
}
impl Hsluv {
	/// Create a new [`Hsluv`] while checking the ranges of the values.
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hsluv, HslBounds> {
        if !(0.0..=360.0).contains(&hue) {
            Err(HslBounds::Hue)
        } else if !(0.0..=100.0).contains(&saturation) {
            Err(HslBounds::Saturation)
        } else if !(0.0..=100.0).contains(&lightness) {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hsluv {
                hue,
                saturation,
                lightness,
            })
        }
    }

    /// Return the HSL components as tuple.
    pub fn hsl(&self) -> (f64, f64, f64) {
        (self.hue, self.saturation, self.lightness)
    }
}

/// Point in the HPLuv color space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hpluv {
	/// Hue component, between 0.0 and 360.0 (inclusive)
    pub hue: f64,
	/// Saturation component, between 0.0 and 100.0 (inclusive)
    pub saturation: f64,
	/// Lightness component, between 0.0 and 100.0 (inclusive)
    pub lightness: f64,
}
impl Hpluv {
	/// Create a new [`Hpluv`] while checking the ranges of the values.
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hpluv, HslBounds> {
        if !(0.0..=360.0).contains(&hue) {
            Err(HslBounds::Hue)
        } else if !(0.0..=100.0).contains(&saturation) {
            Err(HslBounds::Saturation)
        } else if !(0.0..=100.0).contains(&lightness) {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hpluv {
                hue,
                saturation,
                lightness,
            })
        }
    }

    /// Return the HSL components as tuple.
    pub fn hsl(&self) -> (f64, f64, f64) {
        (self.hue, self.saturation, self.lightness)
    }
}

/// Error indicating which component of an XYZ triplet was out of bounds.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum XyzBounds {
    X,
    Y,
    Z,
}

/// Point in the XYZ color space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Xyz {
	/// X component, between 0.0 and 1.0 (inclusive).
    pub x: f64,
	/// Y component, between 0.0 and 1.0 (inclusive).
    pub y: f64,
	/// Z component, between 0.0 and 1.0 (inclusive).
    pub z: f64,
}

impl Xyz {
	/// Create a new [`Xyz`] while checking the ranges of the values.
    pub fn new(x: f64, y: f64, z: f64) -> Result<Xyz, XyzBounds> {
        if !(0.0..=1.0).contains(&x) {
            Err(XyzBounds::X)
        } else if !(0.0..=1.0).contains(&y) {
            Err(XyzBounds::Y)
        } else if !(0.0..=1.0).contains(&z) {
            Err(XyzBounds::Z)
        } else {
            Ok(Xyz { x, y, z })
        }
    }

    /// Return the XYZ components as tuple.
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

/// Point in the [CIELUV](https://en.wikipedia.org/wiki/CIELUV) color space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Luv {
	/// Lightness component
    pub lightness: f64,
	/// U component
    pub u: f64,
	/// V component
    pub v: f64,
}

impl Luv {
    /// Return the Luv components as tuple.
    pub fn luv(&self) -> (f64, f64, f64) {
        (self.lightness, self.u, self.v)
    }
}

/// Point in the Lch color space.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Lch {
	/// Lightness component
    pub lightness: f64,
	/// Chroma component
    pub chroma: f64,
	/// Hue component
    pub hue: f64,
}

impl Lch {
    /// Return the Lch components as tuple.
    pub fn lch(&self) -> (f64, f64, f64) {
        (self.lightness, self.chroma, self.hue)
    }
}
