#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::excessive_precision)]

use num_traits::float::Float;

mod spaces;
pub use spaces::*;

// for RGB
static M: [(f64, f64, f64); 3] = [
    (
        3.24096994190452134377,
        -1.53738317757009345794,
        -0.49861076029300328366,
    ),
    (
        -0.96924363628087982613,
        1.87596750150772066772,
        0.04155505740717561247,
    ),
    (
        0.05563007969699360846,
        -0.20397695888897656435,
        1.05697151424287856072,
    ),
];

// for XYZ
static M_INV: [(f64, f64, f64); 3] = [
    (
        0.41239079926595948129,
        0.35758433938387796373,
        0.18048078840183428751,
    ),
    (
        0.21263900587151035754,
        0.71516867876775592746,
        0.07219231536073371500,
    ),
    (
        0.01933081871559185069,
        0.11919477979462598791,
        0.95053215224966058086,
    ),
];

const REF_U: f64 = 0.19783000664283680764;
const REF_V: f64 = 0.46831999493879100370;

const KAPPA: f64 = 903.29629629629629629630;
const EPSILON: f64 = 0.00885645167903563082;

fn get_bounds(lightness: f64) -> [Line; 6] {
    let mut bounds = [Line {
        slope: 0.0,
        intercept: 0.0,
    }; 6];

    let tl = lightness + 16.0;
    let sub1 = (tl * tl * tl) / 1560896.0;
    let sub2 = if sub1 > EPSILON {
        sub1
    } else {
        lightness / KAPPA
    };

    for (channel, m) in M.iter().enumerate() {
        for t in 0u8..2u8 {
            let t_f64 = f64::from(t);
            let top1 = (284517.0 * m.0 - 94839.0 * m.2) * sub2;
            let top2 = (838422.0 * m.2 + 769860.0 * m.1 + 731718.0 * m.0) * lightness * sub2
                - 769860.0 * t_f64 * lightness;
            let bottom = (632260.0 * m.2 - 126452.0 * m.1) * sub2 + 126452.0 * t_f64;

            bounds[channel * 2 + usize::from(t)] = Line {
                slope: top1 / bottom,
                intercept: top2 / bottom,
            };
        }
    }

    bounds
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    pub slope: f64,
    pub intercept: f64,
}

impl Line {
    pub fn distance_from_origin(&self) -> f64 {
        Float::abs(self.intercept) / Float::sqrt(self.slope * self.slope + 1.0)
    }

    pub fn ray_length_until_intersect(&self, theta: f64) -> f64 {
        self.intercept / (Float::sin(theta) - self.slope * Float::cos(theta))
    }
}

pub fn max_safe_chroma_for_lightness(lightness: f64) -> f64 {
    get_bounds(lightness)
        .iter()
        .map(Line::distance_from_origin)
        .fold(f64::MAX, f64::min)
}

pub fn max_chroma_for_lightness_hue(lightness: f64, hue: f64) -> f64 {
    // (2 * pi / 260)
    let hue_rad = hue * 0.01745329251994329577;

    get_bounds(lightness)
        .iter()
        .map(|l| l.ray_length_until_intersect(hue_rad))
        .filter(|length| length >= &0.0)
        .fold(f64::MAX, f64::min)
}

fn dot_product<T, U, V>(lhs: (T, T, T), rhs: (U, U, U)) -> V
where
    T: core::ops::Mul<U, Output = V>,
    V: core::ops::Add<Output = V>,
{
    lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
}

impl From<Xyz> for Rgb {
    fn from(xyz: Xyz) -> Rgb {
        fn from_linear(c: f64) -> f64 {
            if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * Float::powf(c, 1.0 / 2.4) - 0.055
            }
        }

        let xyz = xyz.xyz();
        Rgb {
            red: from_linear(dot_product(M[0], xyz)),
            green: from_linear(dot_product(M[1], xyz)),
            blue: from_linear(dot_product(M[2], xyz)),
        }
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Xyz {
        fn to_linear(c: f64) -> f64 {
            if c > 0.04045 {
                Float::powf((c + 0.055) / 1.055, 2.4)
            } else {
                c / 12.92
            }
        }

        let rgb_linear = (
            to_linear(rgb.red),
            to_linear(rgb.green),
            to_linear(rgb.blue),
        );
        Xyz {
            x: dot_product(M_INV[0], rgb_linear),
            y: dot_product(M_INV[1], rgb_linear),
            z: dot_product(M_INV[2], rgb_linear),
        }
    }
}

pub fn y_to_lightness(y: f64) -> f64 {
    if y <= EPSILON {
        y * KAPPA
    } else {
        116.0 * Float::cbrt(y) - 16.0
    }
}

pub fn lightness_to_y(lightness: f64) -> f64 {
    if lightness <= 8.0 {
        lightness / KAPPA
    } else {
        let x = (lightness + 16.0) / 116.0;
        x * x * x
    }
}

impl From<Xyz> for Luv {
    fn from(xyz: Xyz) -> Luv {
        let var_u = (4.0 * xyz.x) / (xyz.x + (15.0 * xyz.y) + (3.0 * xyz.z));
        let var_v = (9.0 * xyz.y) / (xyz.x + (15.0 * xyz.y) + (3.0 * xyz.z));
        let lightness = y_to_lightness(xyz.y);

        if lightness < 0.00000001 {
            Luv {
                lightness,
                u: 0.0,
                v: 0.0,
            }
        } else {
            Luv {
                lightness,
                u: 13.0 * lightness * (var_u - REF_U),
                v: 13.0 * lightness * (var_v - REF_V),
            }
        }
    }
}

impl From<Luv> for Xyz {
    fn from(luv: Luv) -> Xyz {
        if luv.lightness <= 0.00000001 {
            // Black will create a divide-by-zero error.
            Xyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let var_u = luv.u / (13.0 * luv.lightness) + REF_U;
            let var_v = luv.v / (13.0 * luv.lightness) + REF_V;
            let y = lightness_to_y(luv.lightness);
            let x = -(9.0 * y * var_u) / ((var_u - 4.0) * var_v - var_u * var_v);
            Xyz {
                x,
                y,
                z: (9.0 * y - (15.0 * var_v * y) - (var_v * x)) / (3.0 * var_v),
            }
        }
    }
}

impl From<Luv> for Lch {
    fn from(luv: Luv) -> Lch {
        let chroma = Float::sqrt(luv.u * luv.u + luv.v * luv.v);

        Lch {
            lightness: luv.lightness,
            chroma,
            hue: if chroma < 0.00000001 {
                0.0
            } else {
                // (180 / pi)
                match Float::atan2(luv.v, luv.u) * 57.29577951308232087680 {
                    hue if hue < 0.0 => hue + 360.0,
                    hue => hue,
                }
            },
        }
    }
}

impl From<Lch> for Luv {
    fn from(lch: Lch) -> Luv {
        // (pi / 180)
        let hue_rad = lch.hue * 0.01745329251994329577;

        Luv {
            lightness: lch.lightness,
            u: Float::cos(hue_rad) * lch.chroma,
            v: Float::sin(hue_rad) * lch.chroma,
        }
    }
}

impl From<Hsluv> for Lch {
    fn from(hsluv: Hsluv) -> Lch {
        Lch {
            lightness: hsluv.lightness,
            // White and black: disambiguate chroma
            chroma: if hsluv.lightness > 99.9999999 || hsluv.lightness < 0.00000001 {
                0.0
            } else {
                max_chroma_for_lightness_hue(hsluv.lightness, hsluv.hue) / 100.0 * hsluv.saturation
            },
            // Grays: disambiguate hue
            hue: if hsluv.saturation < 0.00000001 {
                0.0
            } else {
                hsluv.hue
            },
        }
    }
}

impl From<Lch> for Hsluv {
    fn from(lch: Lch) -> Hsluv {
        Hsluv {
            // Grays: disambiguate hue
            hue: if lch.chroma < 0.00000001 {
                0.0
            } else {
                lch.hue
            },
            // White and black: disambiguate saturation
            saturation: if lch.lightness > 99.9999999 || lch.lightness < 0.00000001 {
                0.0
            } else {
                lch.chroma / max_chroma_for_lightness_hue(lch.lightness, lch.hue) * 100.0
            },
            lightness: lch.lightness,
        }
    }
}

impl From<Hpluv> for Lch {
    fn from(hpluv: Hpluv) -> Lch {
        Lch {
            lightness: hpluv.lightness,
            // White and black: disambiguate chroma
            chroma: if hpluv.lightness > 99.9999999 || hpluv.lightness < 0.00000001 {
                0.0
            } else {
                max_safe_chroma_for_lightness(hpluv.lightness) / 100.0 * hpluv.saturation
            },
            // Grays: disambiguate hue
            hue: if hpluv.saturation < 0.00000001 {
                0.0
            } else {
                hpluv.hue
            },
        }
    }
}

impl From<Lch> for Hpluv {
    fn from(lch: Lch) -> Hpluv {
        Hpluv {
            // Grays: disambiguate hue
            hue: if lch.chroma < 0.00000001 {
                0.0
            } else {
                lch.hue
            },
            // White and black: disambiguate saturation
            saturation: if lch.lightness > 99.9999999 || lch.lightness < 0.00000001 {
                0.0
            } else {
                lch.chroma / max_safe_chroma_for_lightness(lch.lightness) * 100.0
            },
            lightness: lch.lightness,
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

pub fn hsluv_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (f64, f64, f64) {
    Rgb::from(Hsluv {
        hue,
        saturation,
        lightness,
    })
    .rgb()
}
pub fn hpluv_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (f64, f64, f64) {
    Rgb::from(Hpluv {
        hue,
        saturation,
        lightness,
    })
    .rgb()
}
pub fn rgb_to_hsluv(red: f64, green: f64, blue: f64) -> (f64, f64, f64) {
    Hsluv::from(Rgb { red, green, blue }).hsl()
}
pub fn rgb_to_hpluv(red: f64, green: f64, blue: f64) -> (f64, f64, f64) {
    Hpluv::from(Rgb { red, green, blue }).hsl()
}

#[cfg(test)]
mod tests;
