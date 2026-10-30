use crate::shared::{bilinear_transform::BilinearTransform, biquad_filter::BiquadFilter};

const R1: f32 = 4700.;
const R2: f32 = 500000.;
const R3: f32 = 51000.;
const C1: f32 = 4.7e-8;
const C2: f32 = 5.1e-11;

pub struct OpAmp {
  op_amp: BiquadFilter,
  bilinear_transform: BilinearTransform,
  prev_drive: Option<f32>,
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp {
  const R1C1: f32 = R1 * C1;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: BiquadFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
      prev_drive: None,
      z_domain_coefficients: ([0.; 3], [0.; 3]),
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
  }

  pub fn process(&mut self, input: f32, drive: f32) -> f32 {
    if self.prev_drive.map_or(true, |value| value != drive) {
      let s_domain_coefficients = self.get_s_domain_coefficients(drive);
      self.z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
      self.prev_drive = Some(drive);
    }
    self.op_amp.process(input, self.z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, drive: f32) -> ([f32; 3], [f32; 3]) {
    let r2 = drive * R2 + R3;

    let r2c2: f32 = r2 * C2;

    let a0 = Self::R1C1 * r2c2;
    let a1 = Self::R1C1 + r2c2;
    let b1 = r2 * C1 + a1;

    ([0., b1, 1.], [a0, a1, 1.])
  }
}
