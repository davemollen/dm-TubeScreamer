mod coefficients;
use coefficients::Coefficients;

pub struct FirFilter {
  buffer: Vec<[f32; 8]>,
  coefficients: Vec<[f32; 8]>,
  index: usize,
  mask: usize,
}

impl FirFilter {
  pub fn new() -> Self {
    let coefficients = Coefficients::new();
    let length = coefficients.len();
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![[0.; 8]; length],
      coefficients,
      index: 0,
      mask: length - 1,
    }
  }

  pub fn process(&mut self, input: [f32; 8]) -> [f32; 8] {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: [f32; 8]) {
    self.buffer[self.index] = input;
    self.index = self.index + 1 & self.mask;
  }

  fn convolve(&self) -> [f32; 8] {
    let coefficients = &self.coefficients;

    let (front, back) = self.buffer.split_at(self.index);
    back
      .iter()
      .chain(front)
      .zip(coefficients)
      .fold([0.; 8], |mut result, (input, coeff)| {
        result[0] += input[0] * coeff[0];
        result[1] += input[1] * coeff[1];
        result[2] += input[2] * coeff[2];
        result[3] += input[3] * coeff[3];
        result[4] += input[4] * coeff[4];
        result[5] += input[5] * coeff[5];
        result[6] += input[6] * coeff[6];
        result[7] += input[7] * coeff[7];

        result
      })
  }
}
