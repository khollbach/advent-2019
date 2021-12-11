mod intcode_computer;
mod solutions;

pub fn solve(day: u32) {
    assert!(1 <= day && day <= 25);

    use solutions::*;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        5 => day5::main(),
        9 => day9::main(),
        17 => day17::main(),
        19 => day19::main(),
        21 => day21::main(),
        22 => day22::main(),
        // 23 => day23::main(),
        24 => day24::main(),
        _ => panic!("Not yet implemented: Day {}", day),
    }
}
