use canbench::{benchmark, macros::bench, BenchResult};

#[link(wasm_import_module = "ic0")]
extern "C" {
    pub fn stable64_grow(additional_pages: u64) -> i64;
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that no change is reported.
#[bench]
fn no_changes_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that a noisy change is reported.
#[bench]
fn noisy_change_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that regression is reported.
#[bench]
fn regression_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that an improvement is reported.
#[bench]
fn improvement_test() -> BenchResult {
    benchmark(|| {})
}

// The values of the benchmark are persisted such that a regression from zero
// is reported.
#[bench]
fn stable_memory_increase() -> BenchResult {
    benchmark(|| unsafe { stable64_grow(123) })
}

// A benchmark to check that only the _delta_ in stable memory is reported, not
// the total stable memory.
#[bench]
fn stable_memory_delta() -> BenchResult {
    unsafe { stable64_grow(123) };

    // Since only the delta is reported, the benchmark should return a delta
    // of 456 (and ignore the stable memory allocation above).
    benchmark(|| unsafe { stable64_grow(456) })
}

fn main() {}
