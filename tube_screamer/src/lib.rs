#![feature(portable_simd)]
mod bypass;
mod clipper;
mod params;
mod tone;
mod shared {
  pub mod bilinear_transform;
  pub mod biquad_filter;
  pub mod float_ext;
}
mod op_amp;
pub use {bypass::Bypass, params::Params};
use {clipper::Clipper, op_amp::OpAmp, params::Smoother, tone::Tone};

pub struct TubeScreamer {
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
}

impl TubeScreamer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
    self.clipper.reset();
    self.tone.reset();
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let drive = params.drive.next();
    let tone = params.tone.next();
    let level = params.level.next();

    let op_amp_output = self.op_amp.process(input, drive);
    let clip_output = self.clipper.process(op_amp_output) + input;
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
