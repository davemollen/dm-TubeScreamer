#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use tube_screamer::TubeScreamer;
use utils::generate_signal_stream;

fn tube_screamer_bench(c: &mut Criterion) {
  let mut tube_screamer = TubeScreamer::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("tube_screamer", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        tube_screamer.process(*signal, 0.5, 0.5, 0.5);
      }
    })
  });
}

criterion_group!(benches, tube_screamer_bench);
criterion_main!(benches);
