#![feature(portable_simd)]
mod clipper;
mod tone;
mod shared {
  pub mod float_ext;
  pub mod non_inverting_op_amp;
}
mod op_amp;
mod smooth_parameters;
use {clipper::Clipper, op_amp::OpAmp, smooth_parameters::SmoothParameters, tone::Tone};

pub struct TubeScreamer {
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl TubeScreamer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn map_params(&self, drive: f32, tone: f32, level: f32) -> (f32, f32, f32) {
    (
      self.apply_log_curve(drive),
      self.apply_s_taper_curve(tone),
      self.apply_log_curve(level),
    )
  }

  pub fn initialize_params(&mut self, drive: f32, tone: f32, level: f32) {
    self.smooth_parameters.initialize(drive, tone, level);
  }

  pub fn process(&mut self, input: f32, drive: f32, tone: f32, level: f32) -> f32 {
    let (drive, tone, level) = self.smooth_parameters.process(drive, tone, level);
    let op_amp_output = self.op_amp.process(input, drive);
    let clip_output = self.clipper.process(op_amp_output) * 0.4 + input * 0.8;
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }

  fn apply_s_taper_curve(&self, input: f32) -> f32 {
    let inv_input = 1. - input;
    let squared_input = input * input;
    let squared_inv_input = inv_input * inv_input;
    (1. - squared_inv_input * squared_inv_input) * 0.5 + squared_input * squared_input * 0.5
  }

  fn apply_log_curve(&self, input: f32) -> f32 {
    input * input * input
  }
}
