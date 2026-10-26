extern crate lv2;
extern crate tube_screamer;
use lv2::prelude::*;
use tube_screamer::{Bypass, Params, TubeScreamer};

#[derive(PortCollection)]
struct Ports {
  drive: InputPort<InPlaceControl>,
  tone: InputPort<InPlaceControl>,
  level: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-TubeScreamer")]
struct DmTubeScreamer {
  tube_screamer: TubeScreamer,
  params: Params,
  bypass: Bypass,
}

impl Plugin for DmTubeScreamer {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      tube_screamer: TubeScreamer::new(sample_rate),
      params: Params::new(sample_rate),
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.tube_screamer.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.tube_screamer.reset();
      self.params.reset();
    });

    if self.bypass.is_done_activating() {
      ports.output.copy_from(&ports.input, sample_count);
      return;
    }

    self
      .params
      .set(ports.drive.get(), ports.tone.get(), ports.level.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self.tube_screamer.process(dry, &mut self.params);
      let tube_screamer_output = self.bypass.process(dry, wet);
      output.set(tube_screamer_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmTubeScreamer);
