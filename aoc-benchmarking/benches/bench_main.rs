use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use trebuchet::Trebuchet;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-trebuchet/input.txt",
        Trebuchet,
        "Part 1",
        "Part 2"
    ),
    (
        day_002,
        "../day-002-cube-conundrum/input.txt",
        CubeConundrum,
        "Part 1",
        "Part 2"
    ),
    (
        day_003,
        "../day-003-gear-ratios/input.txt",
        GearRatios,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
