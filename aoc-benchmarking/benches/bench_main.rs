use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use camel_cards::CamelCards;
use cosmic_expansion::CosmicExpansion;
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use haunted_wasteland::HauntedWasteland;
use if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer;
use mirage_maintenance::MirageMaintenance;
use pipe_maze::PipeMaze;
use scratchcards::Scratchcards;
use trebuchet::Trebuchet;
use wait_for_it::WaitForIt;
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
    (
        day_004,
        "../day-004-scratchcards/input.txt",
        Scratchcards,
        "Part 1",
        "Part 2"
    ),
    (
        day_005,
        "../day-005-if-you-give-a-seed-a-fertilizer/input.txt",
        IfYouGiveASeedAFertilizer,
        "Combined (including parsing)"
    ),
    (
        day_006,
        "../day-006-wait-for-it/input.txt",
        WaitForIt,
        "Part 1",
        "Part 2"
    ),
    (
        day_007,
        "../day-007-camel-cards/input.txt",
        CamelCards,
        "Combined (including parsing)"
    ),
    (
        day_008,
        "../day-008-haunted-wasteland/input.txt",
        HauntedWasteland,
        "Part 1",
        "Part 2"
    ),
    (
        day_009,
        "../day-009-mirage-maintenance/input.txt",
        MirageMaintenance,
        "Combined (including parsing)"
    ),
    (
        day_010,
        "../day-010-pipe-maze/input.txt",
        PipeMaze,
        "Part 1",
        "Part 2"
    ),
    (
        day_011,
        "../day-011-cosmic-expansion/input.txt",
        CosmicExpansion,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
