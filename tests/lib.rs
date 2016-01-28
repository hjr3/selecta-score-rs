/*

Note: Benchmarking only works on nightly.

extern crate score;
extern crate test;

mod paths;

use score::*;
use test::Bencher;
use paths::PATHS;

#[cfg(test)]

#[bench]
fn bench_non_matching(b: &mut Bencher) {
    b.iter(|| {
        let _ = score("xxxxxxxxxxxxxxxxx", "yyyyyyyyyyyyyyy");
    });
}

#[bench]
fn bench_match_exactly(b: &mut Bencher) {
    b.iter(|| {
        let _ = score("xxxxxxxxxxxxxxxxx", "xxxxxxxxxxxxxxxxx");
    });
}

#[bench]
fn bench_paths_non_matching(b: &mut Bencher) {
    b.iter(|| {
        for choice in PATHS.iter() {
            let _ = score(choice.as_ref(), "xxxxxxxxxxxxxxx");
        };
    });
}

#[bench]
fn bench_paths_empty_query(b: &mut Bencher) {
    b.iter(|| {
        for choice in PATHS.iter() {
            let _ = score(choice.as_ref(), "");
        };
    });
}

#[bench]
fn bench_paths_trivial_query(b: &mut Bencher) {
    b.iter(|| {
        for choice in PATHS.iter() {
            let _ = score(choice.as_ref(), "a");
        };
    });
}
*/
