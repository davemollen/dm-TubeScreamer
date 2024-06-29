extern crate lv2;
extern crate tube_screamer;
use lv2::prelude::*;
use tube_screamer::TubeScreamer;

#[derive(PortCollection)]
struct Ports {
  drive: InputPort<Control>,
  tone: InputPort<Control>,
  level: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-TubeScreamer")]
struct DmTubeScreamer {
  tube_screamer: TubeScreamer,
  is_active: bool,
}

impl Plugin for DmTubeScreamer {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      tube_screamer: TubeScreamer::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (drive, tone, level) =
      self
        .tube_screamer
        .map_params(*ports.drive, *ports.tone, *ports.level);

    if !self.is_active {
      self.tube_screamer.initialize_params(drive, tone, level);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.tube_screamer.process(*input, drive, tone, level);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmTubeScreamer);
