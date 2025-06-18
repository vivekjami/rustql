use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rustql::Settings;
use rustql::graphql::create_schema;
use std::sync::Arc;

fn benchmark_schema_creation(c: &mut Criterion) {
    let settings = Arc::new(Settings::default());

    c.bench_function("schema_creation", |b| {
        b.iter(|| {
            let schema = create_schema(black_box(settings.clone()));
            black_box(schema);
        })
    });
}

fn benchmark_simple_query(c: &mut Criterion) {
    let settings = Arc::new(Settings::default());
    let schema = create_schema(settings);

    c.bench_function("simple_query", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let result = schema.execute(black_box("{ health }")).await;
                black_box(result);
            });
        })
    });
}

criterion_group!(benches, benchmark_schema_creation, benchmark_simple_query);
criterion_main!(benches);
