use nih_plug::prelude::*;
use std::sync::Arc;
use tube_screamer::{Params as ProcessParams, TubeScreamer};
mod tube_screamer_parameters;
use tube_screamer_parameters::TubeScreamerParameters;
mod editor;

struct DmTubeScreamer {
  params: Arc<TubeScreamerParameters>,
  tube_screamer: TubeScreamer,
  process_params: ProcessParams,
}

impl Default for DmTubeScreamer {
  fn default() -> Self {
    let params = Arc::new(TubeScreamerParameters::default());
    Self {
      params: params.clone(),
      tube_screamer: TubeScreamer::new(44100.),
      process_params: ProcessParams::new(44100.),
    }
  }
}

impl Plugin for DmTubeScreamer {
  const NAME: &'static str = "dm-TubeScreamer";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-TubeScreamer";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.tube_screamer = TubeScreamer::new(buffer_config.sample_rate);
    self.process_params = ProcessParams::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    self.process_params.set(
      self.params.drive.value(),
      self.params.tone.value(),
      self.params.level.value(),
    );

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      *sample = self
        .tube_screamer
        .process(*sample, &mut self.process_params);
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmTubeScreamer {
  const CLAP_ID: &'static str = "dm-TubeScreamer";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("An overdrive plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Utility,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmTubeScreamer {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-TubeScreamer.";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmTubeScreamer);
nih_export_vst3!(DmTubeScreamer);
