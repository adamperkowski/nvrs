use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::from_secs(10))
        .with_output_color(true)
}

fn bench_config_load(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("config_operations");

    group.bench_function("config_load", |b| {
        b.iter(|| rt.block_on(nvrs::config::load(None)))
    });

    group.finish();
}

fn bench_verfile_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("verfile_operations");

    let config = rt.block_on(nvrs::config::load(None)).unwrap();

    group.bench_function("verfile_load", |b| {
        b.iter(|| rt.block_on(nvrs::verfiles::load(config.0.__config__.clone())))
    });

    group.finish();
}

#[cfg(feature = "aur")]
fn bench_aur_requests(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("aur_operations");

    let mock_package = nvrs::config::Package::new(
        "aur".to_string(),
        "hyprland-git".to_string(),
        false,
        String::new(),
    )
    .unwrap();

    let client = reqwest::Client::new();

    group.bench_function("aur_request", |b| {
        b.iter(|| {
            rt.block_on(nvrs::run_source(
                ("hyprland-git".to_string(), mock_package.clone()),
                client.clone(),
                None,
            ))
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_config_load, bench_verfile_operations, bench_aur_requests
);
criterion_main!(benches);
