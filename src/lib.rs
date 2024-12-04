pub mod day1 {
    mod part1;
    pub use part1::solution as part1;

    mod part2;
    pub use part2::solution as part2;
}

pub mod day2 {
    mod part1;
    pub use part1::solution as part1;

    mod part2;
    pub use part2::solution as part2;
}

pub mod day3 {
    mod part1;
    pub use part1::solution as part1;

    mod part2;
    pub use part2::solution as part2;
}

pub mod day4 {
    pub static INPUT: &str = include_str!("./day4/input");
    pub static EXAMPLE: &str = include_str!("./day4/example");

    mod part1;
    pub use part1::solution as part1;

    mod part2;
    pub use part2::solution as part2;
}
