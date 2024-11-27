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
    let mut group = c.benchmark_group("config");

    group.bench_function("config_loading", |b| {
        b.iter(|| rt.block_on(nvrs::config::load(None)))
    });

    group.finish();
}

fn bench_verfiles(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("verfiles");

    let config = rt.block_on(nvrs::config::load(None)).unwrap();

    group.bench_function("verfile_loading", |b| {
        b.iter(|| rt.block_on(nvrs::verfiles::load(config.0.__config__.clone())))
    });

    group.finish();
}

fn bench_aur_requests(c: &mut Criterion) {
    #[cfg(feature = "aur")]
    {
        let rt = Runtime::new().unwrap();
        let mut group = c.benchmark_group("requests");

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
}

fn bench_github_requests(c: &mut Criterion) {
    #[cfg(feature = "github")]
    {
        let rt = Runtime::new().unwrap();
        let mut group = c.benchmark_group("requests");

        let mock_package = nvrs::config::Package::new(
            "github".to_string(),
            "orhun/git-cliff".to_string(),
            false,
            "v".to_string(),
        )
        .unwrap();

        let client = reqwest::Client::new();

        group.bench_function("github_request", |b| {
            b.iter(|| {
                rt.block_on(nvrs::run_source(
                    ("git-cliff".to_string(), mock_package.clone()),
                    client.clone(),
                    None,
                ))
            })
        });

        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_config_load, bench_verfiles, bench_aur_requests, bench_github_requests
);
criterion_main!(benches);
