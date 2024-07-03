mod lookup_table;
mod oversample;
use {lookup_table::DIODE_TABLE, oversample::Oversample};

pub struct Clipper {
  oversample: Oversample,
}

impl Clipper {
  const SIZE: usize = DIODE_TABLE.len() - 1;
  const HALF_SIZE: f32 = DIODE_TABLE.len() as f32 * 0.5;

  pub fn new() -> Self {
    Self {
      oversample: Oversample::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.oversample.process(input, |x| {
      x.map(|x| {
        let x = (x / 24.) * Self::HALF_SIZE + Self::HALF_SIZE;
        let index = x.trunc();
        let frac = x - index;
        let i = index as usize;

        DIODE_TABLE[i.clamp(0, Self::SIZE)] * (1. - frac)
          + DIODE_TABLE[(i + 1).clamp(0, Self::SIZE)] * frac
      })
    })
  }
}
