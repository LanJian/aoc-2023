use criterion::criterion_main;

use a_long_walk::ALongWalk;
use aoc_benchmarking::aoc_benches;
use aplenty::Aplenty;
use camel_cards::CamelCards;
use clumsy_crucible::ClumsyCrucible;
use cosmic_expansion::CosmicExpansion;
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use haunted_wasteland::HauntedWasteland;
use hot_springs::HotSprings;
use if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer;
use lavaduct_lagoon::LavaductLagoon;
use lens_library::LensLibrary;
use mirage_maintenance::MirageMaintenance;
use never_tell_me_the_odds::NeverTellMeTheOdds;
use parabolic_reflector_dish::ParabolicReflectorDish;
use pipe_maze::PipeMaze;
use point_of_incidence::PointOfIncidence;
use pulse_propagation::PulsePropagation;
use sand_slabs::SandSlabs;
use scratchcards::Scratchcards;
use snowverload::Snowverload;
use step_counter::StepCounter;
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
    (
        day_018,
        "../day-018-lavaduct-lagoon/input.txt",
        LavaductLagoon,
        "Part 1",
        "Part 2"
    ),
    (
        day_019,
        "../day-019-aplenty/input.txt",
        Aplenty,
        "Part 1",
        "Part 2"
    ),
    (
        day_020,
        "../day-020-pulse-propagation/input.txt",
        PulsePropagation,
        "Part 1",
        "Part 2"
    ),
    (
        day_021,
        "../day-021-step-counter/input.txt",
        StepCounter,
        "Part 1",
        "Part 2"
    ),
    (
        day_022,
        "../day-022-sand-slabs/input.txt",
        SandSlabs,
        "Combined (including parsing)"
    ),
    (
        day_023,
        "../day-023-a-long-walk/input.txt",
        ALongWalk,
        "Part 1",
        "Part 2"
    ),
    (
        day_024,
        "../day-024-never-tell-me-the-odds/input.txt",
        NeverTellMeTheOdds,
        "Part 1",
        "Part 2"
    ),
    (
        day_025,
        "../day-025-snowverload/input.txt",
        Snowverload,
        "Combined (including parsing)"
    ),
    // bench_marker
}
