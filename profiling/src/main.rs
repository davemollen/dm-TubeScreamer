mod utils;
use tube_screamer::TubeScreamer;
use utils::generate_signal;

fn main() {
  let mut tube_screamer = TubeScreamer::new(44100.);

  loop {
    let input = generate_signal();
    tube_screamer.process(input, 0.5, 0.5, 0.5);
  }
}
