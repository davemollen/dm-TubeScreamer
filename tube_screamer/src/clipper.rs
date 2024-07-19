mod fir_filter;
use {
  fir_filter::FirFilter,
  std::simd::{f32x8, num::SimdFloat, StdFloat},
};

const OVERSAMPLE_FACTOR: f32 = 8.;

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
    let upsampled = self
      .upsample_fir
      .process(f32x8::splat(input * OVERSAMPLE_FACTOR));
    let clipped = Self::clip(upsampled);
    /*
      clipping gain is 0.3333 -> 0.66666 * 0.5
      and the input signal is mixed with the clipped signal
    */
    self.downsample_fir.process(clipped).reduce_sum() * 0.33333 + input
  }

  fn clip(x: f32x8) -> f32x8 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let a = x + f32x8::splat(0.16489087) * x3 + f32x8::splat(0.00985468) * x5;
    a / (f32x8::splat(1.0) + a * a).sqrt()
  }
}
