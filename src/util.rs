#[inline]
/// linear 0.0-1.0 floating point to srgb 0-255 integer conversion.
pub fn linear_to_srgb(value: f32) -> u32 {
    let v = f32::max(0., f32::min(1., value));
    if v <= 0.003_130_8 {
        (v * 12.92 * 255. + 0.5).round() as u32
    } else {
        ((1.055 * f32::powf(v, 1. / 2.4) - 0.055) * 255. + 0.5).round() as u32
    }
}

#[inline]
/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
pub fn srgb_to_linear(value: f32) -> f32 {
    let v = value / 255.;
    if v <= 0.04045 {
        v / 12.92
    } else {
        f32::powf((v + 0.055) / 1.055, 2.4)
    }
}

fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

pub fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}

#[inline]
fn build_lut() -> [f32; 256] {
    let mut data = [0.0; 256];

    for (index, value) in data.iter_mut().enumerate() {
        *value = srgb_to_linear(index as f32);
    }

    data
}

pub fn pre_compute_data_linear(rgb: &[u8]) -> Vec<f32> {
    let lookup: [f32; 256] = build_lut();

    rgb.iter().map(|data| lookup[*data as usize]).collect()
}

#[test]
fn do_map() {
    use std::time::Instant;

    let incoming = vec![100; 400_000_000];
    let now = Instant::now();

    let res = pre_compute_data_linear(&incoming);

    println!("It took {:?} ", now.elapsed());

    for i in res {
        assert!(i < 100.);
    }
}
