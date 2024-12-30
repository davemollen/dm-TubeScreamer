mod utils;
use tube_screamer::{Params, TubeScreamer};
use utils::generate_signal;

fn main() {
  let mut tube_screamer = TubeScreamer::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);

  loop {
    let input = generate_signal();
    tube_screamer.process(input, &mut params);
  }
}
