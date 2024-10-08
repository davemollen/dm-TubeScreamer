mod fir_filter;
use {
  fir_filter::FirFilter,
  std::simd::{f32x8, num::SimdFloat, StdFloat},
};

pub struct Clipper {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(),
      downsample_fir: FirFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let upsampled = self.upsample_fir.upsample(input * 1.877);
    let clipped = Self::clip(upsampled);
    self.downsample_fir.downsample(clipped) * 0.3204805 // 0.640961 * 0.5
  }

  fn clip(x: f32x8) -> f32x8 {
    let x_abs = x.abs();
    let x2 = x_abs * x_abs;
    let x4 = x2 * x2;
    let a = f32x8::splat(1.) + x4;

    x / a.sqrt().sqrt()
  }
}
