check-all:
    cargo check --all --all-targets --no-default-features
    cargo check --all --all-targets --features="harness"

run *argv:
    cargo run --features="harness" --release -- {{ argv }}

run-benchmarks:
    cargo bench --features="harness" --bench="aoc-bench"
