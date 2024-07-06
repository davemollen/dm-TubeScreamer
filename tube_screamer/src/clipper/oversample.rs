mod fir_filter;
use fir_filter::FirFilter;

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct Oversample {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Oversample {
  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(),
      downsample_fir: FirFilter::new(),
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
  where
    F: Fn([f32; 8]) -> [f32; 8],
  {
    let upsampled = self.upsample_fir.process([input * OVERSAMPLE_FACTOR; 8]);
    let processed = callback(upsampled);
    self.downsample_fir.process(processed).into_iter().sum()
  }
}
