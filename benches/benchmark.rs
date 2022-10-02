use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use reverse_geocoding::config::Config;
use reverse_geocoding::geocoding::Geocoding;

const MAX_LATITUDE: f64 = 90.0000000;
const MIN_LATITUDE: f64 = -90.0000000;
const MAX_LONGITUDE: f64 = 180.0000000;
const MIN_LONGITUDE: f64 = -180.0000000;

const DATA_PATH: &'static str = "data/";
const OSM_EXTENSION: &'static str = ".bin";

const OSM_FILE_PATHS: &'static [&str; 4] = &[
    "planet-220926.osm.1",
    "planet-220926.osm.0_1",
    "planet-220926.osm.0_01",
    "planet-220926.osm.0_0005",
];

fn bench_lookup(c: &mut Criterion) {
    let mut grouped_benchmarks = c.benchmark_group("Benchmark lookup for each resolution");
    grouped_benchmarks.sample_size(10_000);
    let osm_file_paths: Vec<String> = OSM_FILE_PATHS
        .iter()
        .map(|&path| {
            format!(
                "{}{}{}",
                DATA_PATH.to_string(),
                path.to_string(),
                OSM_EXTENSION.to_string()
            )
        })
        .collect::<Vec<String>>();
    for path in osm_file_paths {
        let config = Config {
            file_path: path.to_string(),
            port: 4020,
        };
        let mut geocoding = Geocoding::new(&config);
        grouped_benchmarks.bench_function(BenchmarkId::new("Cache::Lookup_uncached", &path), |b| {
            b.iter(|| {
                let longitude = rand::thread_rng().gen_range(MIN_LONGITUDE..MAX_LONGITUDE) as f32;
                let latitude = rand::thread_rng().gen_range(MIN_LATITUDE..MAX_LATITUDE) as f32;
                geocoding.lookup(black_box(longitude), black_box(latitude));
            });
        });
        grouped_benchmarks.bench_function(BenchmarkId::new("Cache::Lookup_cached", &path), |b| {
            let longitude = rand::thread_rng().gen_range(MIN_LONGITUDE..MAX_LONGITUDE) as f32;
            let latitude = rand::thread_rng().gen_range(MIN_LATITUDE..MAX_LATITUDE) as f32;
            b.iter(|| {
                geocoding.lookup(black_box(longitude), black_box(latitude));
            });
        });
    }
    grouped_benchmarks.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
