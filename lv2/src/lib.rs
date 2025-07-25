extern crate lv2;
extern crate tube_screamer;
use lv2::prelude::*;
use tube_screamer::{Params, TubeScreamer};

#[derive(PortCollection)]
struct Ports {
  drive: InputPort<InPlaceControl>,
  tone: InputPort<InPlaceControl>,
  level: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
}

#[uri("https://github.com/davemollen/dm-TubeScreamer")]
struct DmTubeScreamer {
  tube_screamer: TubeScreamer,
  params: Params,
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
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self
      .params
      .set(ports.drive.get(), ports.tone.get(), ports.level.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let tube_screamer_output = self.tube_screamer.process(input.get(), &mut self.params);
      output.set(tube_screamer_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmTubeScreamer);
