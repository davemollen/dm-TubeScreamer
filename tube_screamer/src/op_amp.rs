use crate::shared::non_inverting_op_amp::NonInvertingOpAmp;

const R1: f32 = 4700.;
const R2: f32 = 500000.;
const R3: f32 = 51000.;
const C1: f32 = 4.7e-8;
const C2: f32 = 5.1e-11;

pub struct OpAmp {
  op_amp: NonInvertingOpAmp,
}

impl OpAmp {
  const R1C1: f32 = R1 * C1;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: NonInvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, drive: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(drive);
    self.op_amp.process(input, s_domain_coefficients)
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
