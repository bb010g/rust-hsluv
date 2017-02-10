#[cfg(test)]
extern crate json;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub enum RgbBounds {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    red: f64,
    green: f64,
    blue: f64,
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
            Ok(Rgb {
                red: red,
                green: green,
                blue: blue,
            })
        }
    }

    pub fn red(&self) -> f64 {
        self.red
    }
    pub fn red_ref(&self) -> &f64 {
        &self.red
    }
    pub fn red_mut(&mut self) -> &mut f64 {
        &mut self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }
    pub fn green_ref(&self) -> &f64 {
        &self.green
    }
    pub fn green_mut(&mut self) -> &mut f64 {
        &mut self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }
    pub fn blue_ref(&self) -> &f64 {
        &self.blue
    }
    pub fn blue_mut(&mut self) -> &mut f64 {
        &mut self.blue
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HslBounds {
    Hue,
    Saturation,
    Lightness,
}

pub trait Hsl: From<Rgb> {
    fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Self, HslBounds>;

    fn hue(&self) -> f64;
    fn hue_ref(&self) -> &f64;
    fn hue_mut(&mut self) -> &mut f64;

    fn saturation(&self) -> f64;
    fn saturation_ref(&self) -> &f64;
    fn saturation_mut(&mut self) -> &mut f64;

    fn lightness(&self) -> f64;
    fn lightness_ref(&self) -> &f64;
    fn lightness_mut(&mut self) -> &mut f64;
}

#[derive(Debug, Clone, Copy)]
pub struct Hsluv {
    hue: f64,
    saturation: f64,
    lightness: f64,
}
impl Hsl for Hsluv {
    fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hsluv, HslBounds> {
        if hue < 0.0 || hue > 360.0 {
            Err(HslBounds::Hue)
        } else if saturation < 0.0 || saturation > 100.0 {
            Err(HslBounds::Saturation)
        } else if lightness < 0.0 || saturation > 100.0 {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hsluv {
                hue: hue,
                saturation: saturation,
                lightness: lightness,
            })
        }
    }

    fn hue(&self) -> f64 {
        self.hue
    }
    fn hue_ref(&self) -> &f64 {
        &self.hue
    }
    fn hue_mut(&mut self) -> &mut f64 {
        &mut self.hue
    }

    fn saturation(&self) -> f64 {
        self.saturation
    }
    fn saturation_ref(&self) -> &f64 {
        &self.saturation
    }
    fn saturation_mut(&mut self) -> &mut f64 {
        &mut self.saturation
    }

    fn lightness(&self) -> f64 {
        self.lightness
    }
    fn lightness_ref(&self) -> &f64 {
        &self.lightness
    }
    fn lightness_mut(&mut self) -> &mut f64 {
        &mut self.lightness
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hpluv {
    hue: f64,
    saturation: f64,
    lightness: f64,
}
impl Hsl for Hpluv {
    fn new(hue: f64, saturation: f64, lightness: f64) -> Result<Hpluv, HslBounds> {
        if hue < 0.0 || hue > 360.0 {
            Err(HslBounds::Hue)
        } else if saturation < 0.0 || saturation > 100.0 {
            Err(HslBounds::Saturation)
        } else if lightness < 0.0 || saturation > 100.0 {
            Err(HslBounds::Lightness)
        } else {
            Ok(Hpluv {
                hue: hue,
                saturation: saturation,
                lightness: lightness,
            })
        }
    }

    fn hue(&self) -> f64 {
        self.hue
    }
    fn hue_ref(&self) -> &f64 {
        &self.hue
    }
    fn hue_mut(&mut self) -> &mut f64 {
        &mut self.hue
    }

    fn saturation(&self) -> f64 {
        self.saturation
    }
    fn saturation_ref(&self) -> &f64 {
        &self.saturation
    }
    fn saturation_mut(&mut self) -> &mut f64 {
        &mut self.saturation
    }

    fn lightness(&self) -> f64 {
        self.lightness
    }
    fn lightness_ref(&self) -> &f64 {
        &self.lightness
    }
    fn lightness_mut(&mut self) -> &mut f64 {
        &mut self.lightness
    }
}

pub fn hsluv_to_rgb((hue, saturation, lightness): (f64, f64, f64)) -> (f64, f64, f64) {
    match Rgb::from(Hsluv::new(hue, saturation, lightness)
        .expect("Invalid HSLuv; try using Hsluv & From")) {
        Rgb { red, green, blue } => (red, green, blue),
    }
}
pub fn hpluv_to_rgb((hue, saturation, lightness): (f64, f64, f64)) -> (f64, f64, f64) {
    match Rgb::from(Hpluv::new(hue, saturation, lightness)
        .expect("Invalid HPLuv; try using Hpluv & From")) {
        Rgb { red, green, blue } => (red, green, blue),
    }
}
pub fn rgb_to_hsluv((hue, saturation, lightness): (f64, f64, f64)) -> (f64, f64, f64) {
    match Hsluv::from(Rgb::new(hue, saturation, lightness)
        .expect("Invalid RGB; try using Rgb & From")) {
        Hsluv { hue, saturation, lightness } => (hue, saturation, lightness),
    }
}
pub fn rgb_to_hpluv((hue, saturation, lightness): (f64, f64, f64)) -> (f64, f64, f64) {
    match Hpluv::from(Rgb::new(hue, saturation, lightness)
        .expect("Invalid RGB; try using Rgb & From")) {
        Hpluv { hue, saturation, lightness } => (hue, saturation, lightness),
    }
}

// for RGB
static M: [(f64, f64, f64); 3] =
    [(3.24096994190452134377, -1.53738317757009345794, -0.49861076029300328366),
     (-0.96924363628087982613, 1.87596750150772066772, 0.04155505740717561247),
     (0.05563007969699360846, -0.20397695888897656435, 1.05697151424287856072)];

// for XYZ
static M_INV: [(f64, f64, f64); 3] =
    [(0.41239079926595948129, 0.35758433938387796373, 0.18048078840183428751),
     (0.21263900587151035754, 0.71516867876775592746, 0.07219231536073371500),
     (0.01933081871559185069, 0.11919477979462598791, 0.95053215224966058086)];

const REF_U: f64 = 0.19783000664283680764;
const REF_V: f64 = 0.46831999493879100370;

const KAPPA: f64 = 903.29629629629629629630;
const EPSILON: f64 = 0.00885645167903563082;

#[derive(Debug, Clone, Copy)]
struct Bounds(f64, f64);

struct BoundsIter<I> {
    iter: I,
    prev_pull: Option<(f64, f64, f64)>,

    lightness: f64,
    sub2: f64,
}

impl<I: fmt::Debug> fmt::Debug for BoundsIter<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoundsIter")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I: Iterator<Item = (f64, f64, f64)>> Iterator for BoundsIter<I> {
    type Item = Bounds;

    fn next(&mut self) -> Option<Bounds> {
        let (t, prev_pull): (u8, _) = match self.prev_pull {
            None => {
                match self.iter.next() {
                    None => return None,
                    Some(prev_pull) => (0, Some(prev_pull)),
                }
            }
            Some(_) => (1, None),
        };
        let m = self.prev_pull.or(prev_pull).expect("No return after 2 Nones from iter");
        self.prev_pull = prev_pull;

        let top1: f64 = (284517.0 * m.0 - 94839.0 * m.2) * self.sub2;
        let top2: f64 = (838422.0 * m.2 + 769860.0 * m.1 + 731718.0 * m.0) * self.lightness *
                        self.sub2 -
                        769860.0 * f64::from(t) * self.lightness;
        let bottom: f64 = (632260.0 * m.2 - 126452.0 * m.1) * self.sub2 + 126452.0 * f64::from(t);

        Some(Bounds(top1 / bottom, top2 / bottom))
    }
}

fn get_bounds(lightness: f64)
              -> BoundsIter<std::iter::Cloned<std::slice::Iter<'static, (f64, f64, f64)>>> {
    let tl: f64 = lightness + 16.0;
    let sub1: f64 = (tl * tl * tl) / 1560896.0;
    let sub2: f64 = if sub1 > EPSILON {
        sub1
    } else {
        lightness / KAPPA
    };

    BoundsIter {
        iter: M.into_iter().cloned(),
        prev_pull: None,

        lightness: lightness,
        sub2: sub2,
    }
}

fn intersect_line_line(line1: &Bounds, line2: &Bounds) -> f64 {
    (line1.1 - line2.1) / (line2.0 - line1.0)
}

fn dist_from_pole(x: f64, y: f64) -> f64 {
    f64::sqrt(x * x + y * y)
}

fn ray_length_until_intersect(theta: f64, line: &Bounds) -> f64 {
    line.1 / (f64::sin(theta) - line.0 * f64::cos(theta))
}

fn max_safe_chroma_for_lightness(lightness: f64) -> f64 {
    let mut min_len = std::f64::MAX;

    for bounds in get_bounds(lightness) {
        let Bounds(m1, b1) = bounds;
        // x where line intersects with perpendicular running though (0, 0)
        let line2 = Bounds(-1.0 / m1, 0.0);
        let x: f64 = intersect_line_line(&bounds, &line2);
        let distance: f64 = dist_from_pole(x, b1 + x * m1);

        if distance >= 0.0 && distance < min_len {
            min_len = distance;
        }
    }

    min_len
}

fn max_chroma_for_lightness_hue(lightness: f64, hue: f64) -> f64 {
    let mut min_len = std::f64::MAX;
    let hue_rad: f64 = hue * 0.01745329251994329577; // (2 * pi / 260)

    for bounds in get_bounds(lightness) {
        let lightness: f64 = ray_length_until_intersect(hue_rad, &bounds);

        if lightness >= 0.0 && lightness < min_len {
            min_len = lightness;
        }
    }

    min_len
}

fn dot_product<T, U, V>(lhs: (T, T, T), rhs: (U, U, U)) -> V
    where T: ops::Mul<U, Output = V>,
          V: ops::Add<Output = V>
{
    lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
}

// used for RGB conversions
fn from_linear(c: f64) -> f64 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * f64::powf(c, 1.0 / 2.4) - 0.055
    }
}

fn to_linear(c: f64) -> f64 {
    if c > 0.04045 {
        f64::powf((c + 0.055) / 1.055, 2.4)
    } else {
        c / 12.92
    }
}

#[derive(Debug, Copy, Clone)]
struct Xyz {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Xyz> for Rgb {
    fn from(Xyz { x, y, z }: Xyz) -> Rgb {
        let xyz: (f64, f64, f64) = (x, y, z);
        Rgb {
            red: from_linear(dot_product(M[0], xyz)),
            green: from_linear(dot_product(M[1], xyz)),
            blue: from_linear(dot_product(M[2], xyz)),
        }
    }
}

impl From<Rgb> for Xyz {
    fn from(Rgb { red, green, blue }: Rgb) -> Xyz {
        let rgb_linear: (f64, f64, f64) = (to_linear(red), to_linear(green), to_linear(blue));
        Xyz {
            x: dot_product(M_INV[0], rgb_linear),
            y: dot_product(M_INV[1], rgb_linear),
            z: dot_product(M_INV[2], rgb_linear),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Luv {
    lightness: f64,
    u: f64,
    v: f64,
}

/*
 * http://en.wikipedia.org/wiki/CIELUV
 * In these formulas, Yn refers to the reference white point. We are using
 * illuminant D65, so Yn (see refY in Maxima file) equals 1. The formula is
 * simplified accordingly.
 */

fn y_to_lightness(y: f64) -> f64 {
    if y <= EPSILON {
        y * KAPPA
    } else {
        116.0 * f64::cbrt(y) - 16.0
    }
}

fn lightness_to_y(lightness: f64) -> f64 {
    if lightness <= 8.0 {
        lightness / KAPPA
    } else {
        let x: f64 = (lightness + 16.0) / 116.0;
        x * x * x
    }
}

impl From<Xyz> for Luv {
    fn from(Xyz { x, y, z }: Xyz) -> Luv {
        let var_u: f64 = (4.0 * x) / (x + (15.0 * y) + (3.0 * z));
        let var_v: f64 = (9.0 * y) / (x + (15.0 * y) + (3.0 * z));
        let lightness: f64 = y_to_lightness(y);

        if lightness < 0.00000001 {
            Luv {
                lightness: lightness,
                u: 0.0,
                v: 0.0,
            }
        } else {
            Luv {
                lightness: lightness,
                u: 13.0 * lightness * (var_u - REF_U),
                v: 13.0 * lightness * (var_v - REF_V),
            }
        }
    }
}

impl From<Luv> for Xyz {
    fn from(Luv { lightness, u, v }: Luv) -> Xyz {
        if lightness <= 0.00000001 {
            // Black will create a divide-by-zero error.
            Xyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let var_u: f64 = u / (13.0 * lightness) + REF_U;
            let var_v: f64 = v / (13.0 * lightness) + REF_V;
            let y: f64 = lightness_to_y(lightness);
            let x: f64 = -(9.0 * y * var_u) / ((var_u - 4.0) * var_v - var_u * var_v);
            Xyz {
                x: x,
                y: y,
                z: (9.0 * y - (15.0 * var_v * y) - (var_v * x)) / (3.0 * var_v),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Lch {
    lightness: f64,
    chroma: f64,
    hue: f64,
}

impl From<Luv> for Lch {
    fn from(Luv { lightness, u, v }: Luv) -> Lch {
        let chroma: f64 = f64::sqrt(u * u + v * v);

        Lch {
            lightness: lightness,
            chroma: chroma,
            hue: if chroma < 0.00000001 {
                0.0
            } else {
                let mut hue: f64 = f64::atan2(v, u) * 57.29577951308232087680; // (180 / pi)
                if hue < 0.0 {
                    hue += 360.0
                }
                hue
            },
        }
    }
}

impl From<Lch> for Luv {
    fn from(Lch { lightness, chroma, hue }: Lch) -> Luv {
        let hrad: f64 = hue * 0.01745329251994329577; // (pi / 180.0)

        Luv {
            lightness: lightness,
            u: f64::cos(hrad) * chroma,
            v: f64::sin(hrad) * chroma,
        }
    }
}

impl From<Hsluv> for Lch {
    fn from(Hsluv { hue, saturation, lightness }: Hsluv) -> Lch {
        Lch {
            lightness: lightness,
            // White and black: disambiguate chroma
            chroma: if lightness > 99.9999999 || lightness < 0.00000001 {
                0.0
            } else {
                max_chroma_for_lightness_hue(lightness, hue) / 100.0 * saturation
            },
            // Grays: disambiguate hue
            hue: if saturation < 0.00000001 { 0.0 } else { hue },
        }
    }
}

impl From<Lch> for Hsluv {
    fn from(Lch { lightness, chroma, hue }: Lch) -> Hsluv {
        Hsluv {
            // Grays: disambiguate hue
            hue: if chroma < 0.00000001 { 0.0 } else { hue },
            // White and black: disambiguate saturation
            saturation: if lightness > 99.9999999 || lightness < 0.00000001 {
                0.0
            } else {
                chroma / max_chroma_for_lightness_hue(lightness, hue) * 100.0
            },
            lightness: lightness,
        }
    }
}

impl From<Hpluv> for Lch {
    fn from(Hpluv { hue, saturation, lightness }: Hpluv) -> Lch {
        Lch {
            lightness: lightness,
            // White and black: disambiguate chroma
            chroma: if lightness > 99.9999999 || lightness < 0.00000001 {
                0.0
            } else {
                max_safe_chroma_for_lightness(lightness) / 100.0 * saturation
            },
            // Grays: disambiguate hue
            hue: if saturation < 0.00000001 { 0.0 } else { hue },
        }
    }
}

impl From<Lch> for Hpluv {
    fn from(Lch { lightness, chroma, hue }: Lch) -> Hpluv {
        Hpluv {
            // Grays: disambiguate hue
            hue: if chroma < 0.00000001 { 0.0 } else { hue },
            // White and black: disambiguate saturation
            saturation: if lightness > 99.9999999 || lightness < 0.00000001 {
                0.0
            } else {
                chroma / max_safe_chroma_for_lightness(lightness) * 100.0
            },
            lightness: lightness,
        }
    }
}

impl From<Hsluv> for Rgb {
    fn from(hsluv: Hsluv) -> Rgb {
        Rgb::from(Xyz::from(Luv::from(Lch::from(hsluv))))
    }
}
impl From<Hpluv> for Rgb {
    fn from(hpluv: Hpluv) -> Rgb {
        Rgb::from(Xyz::from(Luv::from(Lch::from(hpluv))))
    }
}
impl From<Rgb> for Hsluv {
    fn from(rgb: Rgb) -> Hsluv {
        Hsluv::from(Lch::from(Luv::from(Xyz::from(rgb))))
    }
}
impl From<Rgb> for Hpluv {
    fn from(rgb: Rgb) -> Hpluv {
        Hpluv::from(Lch::from(Luv::from(Xyz::from(rgb))))
    }
}

#[cfg(test)]
mod tests;
