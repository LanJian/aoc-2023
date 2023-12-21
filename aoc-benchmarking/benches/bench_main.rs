use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use camel_cards::CamelCards;
use clumsy_crucible::ClumsyCrucible;
use cosmic_expansion::CosmicExpansion;
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use haunted_wasteland::HauntedWasteland;
use hot_springs::HotSprings;
use if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer;
use lens_library::LensLibrary;
use mirage_maintenance::MirageMaintenance;
use parabolic_reflector_dish::ParabolicReflectorDish;
use pipe_maze::PipeMaze;
use point_of_incidence::PointOfIncidence;
use scratchcards::Scratchcards;
use the_floor_will_be_lava::TheFloorWillBeLava;
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
    (
        day_012,
        "../day-012-hot-springs/input.txt",
        HotSprings,
        "Part 1",
        "Part 2"
    ),
    (
        day_013,
        "../day-013-point-of-incidence/input.txt",
        PointOfIncidence,
        "Combined (including parsing)"
    ),
    (
        day_014,
        "../day-014-parabolic-reflector-dish/input.txt",
        ParabolicReflectorDish,
        "Combined (including parsing)"
    ),
    (
        day_015,
        "../day-015-lens-library/input.txt",
        LensLibrary,
        "Part 1",
        "Part 2"
    ),
    (
        day_016,
        "../day-016-the-floor-will-be-lava/input.txt",
        TheFloorWillBeLava,
        "Combined (including parsing)"
    ),
    (
        day_017,
        "../day-017-clumsy-crucible/input.txt",
        ClumsyCrucible,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
