use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::from_secs(10))
        .with_output_color(true)
}

fn bench_config_load(c: &mut Criterion) {
    c.bench_function("config_load", |b| {
        b.iter(|| nvrs::config::load(black_box(None)))
    });
}

fn bench_verfile_operations(c: &mut Criterion) {
    let (config_content, _, _) = nvrs::config::load(None);

    c.bench_function("verfile_load", |b| {
        b.iter(|| nvrs::verfiles::load(black_box(config_content.__config__.clone())))
    });
}

fn bench_api_requests(c: &mut Criterion) {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();

    let mock_package = nvrs::config::Package {
        source: "aur".to_string(),
        aur: "hyprland-git".to_string(),
        github: String::new(),
        gitlab: String::new(),
        host: String::new(),
        prefix: String::new(),
    };

    c.bench_function("api_request", |b| {
        b.iter(|| {
            rt.block_on(nvrs::run_source(
                ("hyprland-git".to_string(), mock_package.clone()),
                None,
            ))
        })
    });
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_config_load, bench_verfile_operations, bench_api_requests
}
criterion_main!(benches);
