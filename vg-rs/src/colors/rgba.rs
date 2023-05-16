//a Imports
use serde::Serialize;

//a Rgba
//tp Rgba
/// A 32-bit color consisting of a 24-bit RGB color with an 8-bit alpha.
///
/// When stored the alpha of 255 is transparent, 0 opaque
///
/// Stored as a u32 with (255-alpha) in top 8 bits, then R, then G, then B in bottom 8 bits
///
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Rgba(u32);

//ip From<u32> for Rgba
impl From<u32> for Rgba {
    #[inline]
    fn from(rgb: u32) -> Self {
        Self(rgb)
    }
}

//ip From<(u8, u8, u8)> for Rgba
impl From<(u8, u8, u8)> for Rgba {
    #[inline]
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self::from_tuple_rgb(rgb)
    }
}

//ip From<(u8, u8, u8, u8)> for Rgba
impl From<(u8, u8, u8, u8)> for Rgba {
    #[inline]
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        Self::from_tuple_rgba(rgba)
    }
}

//ip From<(f32, f32, f32)> for Rgba
impl From<(f32, f32, f32)> for Rgba {
    #[inline]
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        let r = (r * 255.9).floor() as u8;
        let g = (g * 255.9).floor() as u8;
        let b = (b * 255.9).floor() as u8;
        (r, g, b).into()
    }
}

//ip From<(f32, f32, f32, f32)> for Rgba
impl From<(f32, f32, f32, f32)> for Rgba {
    #[inline]
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        let r = (r * 255.9).floor() as u8;
        let g = (g * 255.9).floor() as u8;
        let b = (b * 255.9).floor() as u8;
        let a = (a * 255.9).floor() as u8;
        (r, g, b, a).into()
    }
}

//ip From<&Rgba> for (u8, u8, u8, u8)
impl From<&Rgba> for (u8, u8, u8, u8) {
    fn from(rgba: &Rgba) -> (u8, u8, u8, u8) {
        rgba.as_tuple_rgba()
    }
}

//ip From<Rgba> for (u8, u8, u8, u8)
impl From<Rgba> for (u8, u8, u8, u8) {
    fn from(rgba: Rgba) -> (u8, u8, u8, u8) {
        rgba.as_tuple_rgba()
    }
}

//ip From<&Rgba> for String
//ip From<Rgba> for String
// An SVG-compatible string is generated
impl From<Rgba> for String {
    fn from(rgba: Rgba) -> String {
        let (r, g, b, alpha) = rgba.as_tuple_rgba();
        if alpha == 255 {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else {
            format!("rgba({},{},{},{})", r, g, b, alpha)
        }
    }
}

//ip Rgba
impl Rgba {
    //cp of_rgba
    /// Create an [Rgba] from a u32 containing AARRGGBB as the nybbles of the 32-bit value
    pub fn of_rgba(rgba: u32) -> Self {
        let r: Self = (rgba & 0xffffff).into();
        r.set_alpha((rgba >> 24) as u8)
    }

    //ci from_tuple_rgb
    /// Create an opaque Rgba from a tuple (R, G, B) of u8
    ///
    /// This is accessed from the 'From' trait (or, therefore, Into)
    fn from_tuple_rgb((r, g, b): (u8, u8, u8)) -> Self {
        let rgb = (b as u32) | ((g as u32) << 8) | ((r as u32) << 16);
        dbg!(r, g, b, &rgb);
        Self(rgb)
    }

    //ci from_tuple_rgba
    /// Create an Rgba from a tuple (R, G, B, A) of u8
    ///
    /// This is accessed from the 'From' trait (or, therefore, Into)
    fn from_tuple_rgba((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        let rgba = (b as u32) | ((g as u32) << 8) | ((r as u32) << 16) | (((255 - a) as u32) << 24);
        Self(rgba)
    }

    //cp of_str
    /// Create an Rgba by paarsing a string
    ///
    /// The string must be #xxx or #xxxxxx (really)
    pub fn of_str(s: &str) -> Option<Rgba> {
        if s.as_bytes().first() != Some(&b'#') {
            None
        } else {
            let short_rgb = s.len() < 7;
            match u32::from_str_radix(s.split_at(1).1, 16) {
                Ok(rgb) => {
                    if short_rgb {
                        let b = (rgb >> 8) & 0xf;
                        let g = (rgb >> 4) & 0xf;
                        let r = rgb & 0xf;
                        let r = (r | (r << 4)) as u8;
                        let g = (g | (g << 4)) as u8;
                        let b = (b | (b << 4)) as u8;
                        Some((r, g, b).into())
                    } else {
                        let b = ((rgb >> 16) & 0xff) as u8;
                        let g = ((rgb >> 8) & 0xff) as u8;
                        let r = (rgb & 0xff) as u8;
                        eprintln!("{:x}, {}, {}, {}", rgb, r, g, b);
                        Some((r, g, b).into())
                    }
                }
                _ => None,
            }
        }
    }

    //dp as_tuple_rgba
    /// Convert to an (R, G, B, A) tuple of u8 values
    ///
    /// This is accessed from the 'From' trait (or, therefore, Into)
    pub fn as_tuple_rgba(self) -> (u8, u8, u8, u8) {
        (
            ((self.0 >> 16) & 0xff) as u8,         // r
            ((self.0 >> 8) & 0xff) as u8,          // g
            (self.0 & 0xff) as u8,                 // b
            255 - (((self.0 >> 24) & 0xff) as u8), // alpha
        )
    }

    //bp set_alpha
    /// Builder method that sets the alpha of the Rgba (0 transparent,
    /// 255 opaque)
    pub fn set_alpha(mut self, alpha: u8) -> Self {
        self.0 = (self.0 & 0xffffff) | (((255 - alpha) as u32) << 24);
        self
    }

    //ap alpha
    /// Extract the alpha value (0 transparent, 255 opaque) of the RGB
    pub fn alpha(&self) -> u8 {
        255 - (((self.0 >> 24) & 0xff) as u8)
    }

    //ap is_transparent
    /// Return true if the color is fully transparent
    ///
    /// Note that externally 0 is transparent, 255 is opaque
    pub fn is_transparent(&self) -> bool {
        self.alpha() == 0
    }

    //ap is_opaque
    /// Return true if the color is fully opaque
    ///
    /// Note that externally 0 is transparent, 255 is opaque
    pub fn is_opaque(&self) -> bool {
        self.alpha() == 255
    }

    //mp as_tuple_rgba_f32
    #[inline]
    pub fn as_tuple_rgba_f32(&self) -> (f32, f32, f32, f32) {
        let (r, g, b, a): (u8, u8, u8, u8) = self.into();
        let r = (r as f32) / 255.0;
        let g = (g as f32) / 255.0;
        let b = (b as f32) / 255.0;
        let a = (a as f32) / 255.0;
        (r, g, b, a)
    }

    //mp as_tuple_rgb_f32
    #[inline]
    pub fn as_tuple_rgb_f32(&self) -> (f32, f32, f32) {
        let (r, g, b, _a): (u8, u8, u8, u8) = self.into();
        let r = (r as f32) / 255.0;
        let g = (g as f32) / 255.0;
        let b = (b as f32) / 255.0;
        (r, g, b)
    }

    //mi mmrrgb
    #[inline]
    fn mmrrgb(&self) -> (f32, f32, f32, f32, f32, f32) {
        let (r, g, b, _) = self.as_tuple_rgba_f32();
        let c_min = r.min(g.min(b));
        let c_max = r.max(g.max(b));
        let c_r = c_max - c_min;
        (c_max, c_min, c_r, r, g, b)
    }

    //fi hue_of_mrrgb
    #[inline]
    fn hue_of_mrrgb(c_max: f32, c_r: f32, r: f32, g: f32, b: f32) -> f32 {
        let h = {
            if c_max == r {
                360.0 + 60.0 * (g - b) / c_r
            } else if c_max == g {
                120.0 + 60.0 * (b - r) / c_r
            } else {
                // must be blue
                240.0 + 60.0 * (r - g) / c_r
            }
        };
        if h >= 360.0 {
            h - 360.0
        } else {
            h
        }
    }

    //ap as_hsv
    /// Get the HSV as three f32 values
    pub fn as_hsv(&self) -> (f32, f32, f32) {
        let (c_max, _c_min, c_r, r, g, b) = self.mmrrgb();
        // For a grey, c_r is 0; this covers c_max is 0 too
        if c_r == 0. {
            (0., 0., r)
        } else {
            let s = c_r / c_max;
            let h = Self::hue_of_mrrgb(c_max, c_r, r, g, b);
            (h, s, c_max)
        }
    }

    //ap as_hsl
    /// Get the HSL as three f32 values
    pub fn as_hsl(&self) -> (f32, f32, f32) {
        let (c_max, c_min, c_r, r, g, b) = self.mmrrgb();
        // For a grey, c_r is 0; this covers c_max is 0 too
        if c_r == 0. {
            (0., 0., r)
        } else {
            let l = (c_max + c_min) / 2.0;
            let s = c_r / (1.0 - (2.0 * l - 1.0).abs());
            let h = Self::hue_of_mrrgb(c_max, c_r, r, g, b);
            (h, s, l)
        }
    }

    //ap as_cmyk
    /// Get the CMYK as four f32 values
    pub fn as_cmyk(&self) -> (f32, f32, f32, f32) {
        let (c_max, _c_min, _c_r, r, g, b) = self.mmrrgb();
        let k = 1.0 - c_max;
        let nk = 1.0 - k;
        let c = (nk - r) / nk;
        let m = (nk - g) / nk;
        let y = (nk - b) / nk;
        (c, m, y, k)
    }

    //ap cie_xyz_d6500_to_srgb
    /// Get the CIE XYZ if the color space is sRGB and a white-point of 6500K (?)
    pub fn cie_xyz_d6500_to_srgb((x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        fn cie_l_d6500(c: f32) -> f32 {
            if c > 0.0031308 {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            } else {
                c * 12.92
            }
        }
        let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
        let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
        let b = 0.0557 * x - 0.2040 * y + 1.0570 * z;
        let r = cie_l_d6500(r);
        let g = cie_l_d6500(g);
        let b = cie_l_d6500(b);
        (r, g, b)
    }

    //ap srgb_to_cie_xyz_d6500
    /// Get the CIE XYZ if the color space is sRGB and a white-point of 6500K (?)
    pub fn srgb_to_cie_xyz_d6500(&self) -> (f32, f32, f32) {
        let (r, g, b, _) = self.as_tuple_rgba_f32();
        fn cie_c_d6500(c: f32) -> f32 {
            if c > 0.04045 {
                ((c + 0.055) / 1.055).powf(2.4)
            } else {
                c / 12.92
            }
        }
        let r = cie_c_d6500(r);
        let g = cie_c_d6500(g);
        let b = cie_c_d6500(b);
        let x = 0.4124 * r + 0.3576 * g + 0.1805 * b;
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let z = 0.0193 * r + 0.1192 * g + 0.9505 * b;
        (x, y, z)
    }

    //ap srgb_to_cie_lab_d6500
    /// Get the CIE XYZ if the color space is sRGB and a white-point of 6500K
    pub fn srgb_to_cie_lab_d6500(&self) -> (f32, f32, f32) {
        let (x, y, z) = self.srgb_to_cie_xyz_d6500();
        fn f(t: f32) -> f32 {
            const DELTA: f32 = 6.0 / 29.0;
            const DELTA_CUBED: f32 = DELTA * DELTA * DELTA;
            const DELTA_SQUARED: f32 = DELTA * DELTA;
            if t > DELTA_CUBED {
                t.powf(1.0 / 3.0)
            } else {
                t / (3.0 * DELTA_SQUARED) + 4.0 / 29.0
            }
        }
        let f_x = f(x / 0.950489);
        let f_y = f(y / 1.000);
        let f_z = f(z / 1.088840);
        let l = 116.0 * f_y - 16.0;
        let a = 500.0 * (f_x - f_y);
        let b = 200.0 * (f_y - f_z);
        (l, a, b)
    }

    //ap srgb_to_cie_lch_ab_d6500
    /// Get the CIE HLC if the color space is sRGB and a white-point of 6500K
    pub fn srgb_to_cie_lch_ab_d6500(&self) -> (f32, f32, f32) {
        let (l, a, b) = self.srgb_to_cie_lab_d6500();
        let c = (a * a + b * b).sqrt();
        let h = b.atan2(a) * 180. / 3.14159265;
        (l, c, h)
    }
    //ap srgb_to_cie_luv_d6500
    /// Get the CIE XYZ if the color space is sRGB and a white-point of 6500K
    pub fn srgb_to_cie_luv_d6500(&self) -> (f32, f32, f32) {
        const DELTA: f32 = 6.0 / 29.0;
        const DELTA_CUBED: f32 = DELTA * DELTA * DELTA;
        let (x, y, z) = self.srgb_to_cie_xyz_d6500();
        let _x_n = 0.950489;
        let y_n = 1.000;
        let _z_n = 1.088840;
        let y_y_n = y / y_n;
        let l = {
            if y > DELTA_CUBED {
                116.0 * y_y_n.powf(1.0 / 3.0) - 16.0
            } else {
                (29.0_f32 / 3.0).powf(3.0) * y_y_n
            }
        };
        let u = 4.0 * x / (x + 15.0 * y + 3.0 * z);
        let v = 9.0 * y / (x + 15.0 * y + 3.0 * z);
        let u = 13.0 * l * (u - 0.2009);
        let v = 13.0 * l * (v - 0.4610);
        (l, u, v)
    }

    //ap cie_luv_d6500_to_srgb
    /// Convert a CIE LUV to an sRGB float tuple
    pub fn cie_luv_d6500_to_srgb((l, u, v): (f32, f32, f32)) -> (f32, f32, f32) {
        let u = u / 13.0 / l + 0.2009;
        let v = v / 13.0 / l + 0.4610;
        let _x_n = 0.950489;
        let y_n = 1.000;
        let _z_n = 1.088840;
        let y = {
            if l > 8.0 {
                ((l + 16.0) / 116.0).powf(3.0) * y_n
            } else {
                (3.0_f32 / 29.0_f32).powf(3.0) * l * y_n
            }
        };
        let x = y * 9.0 * u / (4.0 * v);
        let z = y * (12.0 - 3.0 * u - 20.0 * v) / (4.0 * v);
        (x, y, z)
    }

    //ap srgb_to_cie_lch_uv_d6500
    /// Get the CIE HLC if the color space is sRGB and a white-point of 6500K
    pub fn srgb_to_cie_lch_uv_d6500(&self) -> (f32, f32, f32) {
        let (l, u, v) = self.srgb_to_cie_luv_d6500();
        let c = (u * u + v * v).sqrt();
        let h = v.atan2(u) * 180. / 3.14159265;
        (l, c, h)
    }
    //zz All done
}
//a Tests
#[test]
fn convert() {
    #[track_caller]
    fn approx_eq(v: f32, e: f32, m: &str) {
        let d = (e - v).abs();
        let min_diff = (v * 0.003).abs() + 1E-6;
        assert!(
            d < min_diff,
            "{} difference should be < {} (v-e = {} - {})",
            d,
            min_diff,
            v,
            e
        );
    }
    #[track_caller]
    fn approx_eq_t3((v0, v1, v2): (f32, f32, f32), (e0, e1, e2): (f32, f32, f32), m: &str) {
        approx_eq(v0, e0, m);
        approx_eq(v1, e1, m);
        approx_eq(v2, e2, m);
    }
    #[track_caller]
    fn approx_eq_t4(
        (v0, v1, v2, v3): (f32, f32, f32, f32),
        (e0, e1, e2, e3): (f32, f32, f32, f32),
        m: &str,
    ) {
        approx_eq(v0, e0, m);
        approx_eq(v1, e1, m);
        approx_eq(v2, e2, m);
        approx_eq(v3, e3, m);
    }

    let rgb: Rgba = (127, 231, 75).into();
    approx_eq_t4(
        rgb.as_tuple_rgba_f32(),
        (0.498, 0.906, 0.294, 1.),
        "RGB as f32",
    );
    approx_eq_t3(rgb.as_hsv(), (100.0, 0.675, 0.906), "RGB as hsv");
    approx_eq_t3(rgb.as_hsl(), (100.0, 0.765, 0.600), "RGB as hsl");
    approx_eq_t4(rgb.as_cmyk(), (0.450, 0.0, 0.675, 0.094), "RGB as cmyk");
    approx_eq_t3(
        Rgba::cie_xyz_d6500_to_srgb(rgb.srgb_to_cie_xyz_d6500()),
        rgb.as_tuple_rgb_f32(),
        "CIE XYZ and back",
    );
    approx_eq_t3(
        rgb.srgb_to_cie_xyz_d6500(),
        (0.386, 0.622, 0.166),
        "RGB as sRGBB as CIE XYZ at D6500",
    );
    approx_eq_t3(
        rgb.srgb_to_cie_lab_d6500(),
        (83.004, -56.481, 63.807), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has -56.476 and 63.808
        "RGB as sRGBB as CIE XYZ at D6500",
    );
    approx_eq_t3(
        rgb.srgb_to_cie_lch_ab_d6500(),
        (83.004, 85.214, 131.515), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has 120.332 and 99.618
        "RGB as sRGBB as CIE XYZ at D6500",
    );

    // approx_eq_t3(
    // Rgba::cie_luv_d6500_to_srgb((l, u, v))
    // rgb.as_tuple_rgb_f32(),
    // "CIE XYZ and back",
    // );

    //    approx_eq_t3(
    //        rgb.srgb_to_cie_luv_d6500(),
    //        (83.004, -50.307, 85.982), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has -56.476 and 63.808
    //        "RGB as sRGBB as CIE XYZ at D6500",
    //    );
    approx_eq_t3(
        rgb.srgb_to_cie_lch_uv_d6500(),
        (83.004, 108.124, 119.728), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has 120.332 and 99.618
        "RGB as sRGBB as CIE XYZ at D6500",
    );

    let rgb: Rgba = (143, 235, 102).into();
    approx_eq_t4(
        rgb.as_tuple_rgba_f32(),
        (0.560, 0.920, 0.400, 1.),
        "RGB as f32",
    );
    approx_eq_t3(rgb.as_hsv(), (101.525, 0.566, 0.920), "RGB as hsv");
    approx_eq_t3(rgb.as_hsl(), (101.525, 0.769, 0.660), "RGB as hsl");
    approx_eq_t4(rgb.as_cmyk(), (0.391, 0.0, 0.566, 0.0784), "RGB as cmyk");
    approx_eq_t3(
        rgb.srgb_to_cie_xyz_d6500(),
        (0.434, 0.662, 0.231),
        "RGB as sRGBB as CIE XYZ at D6500",
    );
    approx_eq_t3(
        rgb.srgb_to_cie_lab_d6500(),
        (85.11, -50.68, 55.103),
        "RGB as sRGBB as CIE XYZ at D6500",
    );
    approx_eq_t3(
        rgb.srgb_to_cie_lch_ab_d6500(),
        (85.11, 74.866, 132.61), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has 120.332 and 99.618
        "RGB as sRGBB as CIE XYZ at D6500",
    );
    //    approx_eq_t3(
    //        rgb.srgb_to_cie_luv_d6500(),
    //        (83.004, -50.307, 85.982), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has -56.476 and 63.808
    //        "RGB as sRGBB as CIE XYZ at D6500",
    //    );
    approx_eq_t3(
        rgb.srgb_to_cie_lch_uv_d6500(),
        (85.11, 98.869, 119.344), // http://mkweb.bcgsc.ca/brewer/talks/color-palettes-brewer.pdf has 120.332 and 99.618
        "RGB as sRGBB as CIE XYZ at D6500",
    );
}
