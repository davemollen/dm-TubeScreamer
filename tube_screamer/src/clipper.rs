// Diode values
const IS: f32 = 2.52e-9; // Saturation Current (N914)
const N: f32 = 1.752; // Diode ideality factor (N914)
const VT: f32 = 25.864e-3; // Thermal Voltage

// Component Values
const R1: f32 = 4.7e3;
const R2: f32 = 51e3;
const R3: f32 = 500e3;
const C1: f32 = 47e-9;
const C2: f32 = 51e-12;

pub struct Clipper {
  vin_n1: f32,
  v_hp_n1: f32,
  v_n1: f32,
  ts: f32,
}

impl Clipper {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      vin_n1: 0.,
      v_hp_n1: 0.,
      v_n1: 0.,
      ts: sample_rate.recip(),
    }
  }

  pub fn process(&mut self, vin: f32, drive: f32) -> f32 {
    let r2 = R2 + drive * R3;

    // High Pass Input Stage
    let v_hp = ((1. - self.ts / (2. * R1 * C1)) * self.v_hp_n1 + vin - self.vin_n1)
      / (1. + self.ts / (2. * R1 * C1));
    let i = v_hp / R1;

    // Newton Solver
    let mut v = self.v_n1;
    for _ in 1..100 {
      let f = v - self.v_n1 - (self.ts / C2) * i
        + (self.ts / (2. * r2 * C2)) * (v + self.v_n1)
        + (self.ts * IS / C2) * (v / (N * VT)).sinh()
        + (self.ts * IS / C2) * (self.v_n1 / (N * VT)).sinh();
      let d_f =
        1. + (self.ts / (2. * r2 * C2)) + ((self.ts * IS) / (N * VT * C2)) * (v / (N * VT)).cosh();
      let err = f / d_f;

      if err.abs() < 10e-10 {
        break;
      } else {
        v = v - err;
      }
    }

    // Read Output
    let vout = vin + v;

    // Update states
    self.vin_n1 = vin;
    self.v_hp_n1 = v_hp;
    self.v_n1 = v;

    vout
  }
}
