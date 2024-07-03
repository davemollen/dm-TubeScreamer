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
    let upsampled = self.upsample(input);
    let processed = self.run_upsampled_process(upsampled, callback);
    self.downsample(processed)
  }

  fn upsample(&mut self, input: f32) -> [f32; 8] {
    self.upsample_fir.process([input * OVERSAMPLE_FACTOR; 8])
  }

  fn run_upsampled_process<F>(&mut self, input: [f32; 8], callback: F) -> [f32; 8]
  where
    F: Fn([f32; 8]) -> [f32; 8],
  {
    callback(input)
  }

  fn downsample(&mut self, input: [f32; 8]) -> f32 {
    self.downsample_fir.process(input).into_iter().sum()
  }
}
