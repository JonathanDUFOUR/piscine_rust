use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ex04::{prime_decomposition, Integer, Prime};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("Prime::next()", |b| {
		b.iter(|| {
			let mut prime = Prime::new(0);

			for _ in 0..55 {
				black_box(prime.next());
			}
		})
	});
	c.bench_function("prime_decomposition()", |b| {
		b.iter(|| {
			for n in 0..55 {
				black_box(prime_decomposition(n as Integer));
			}
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
