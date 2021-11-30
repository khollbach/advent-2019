mod solutions;

pub fn solve(day: u32) {
    assert!(1 <= day && day <= 25);

    use solutions::*;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        _ => panic!("Not yet implemented: Day {}", day),
    }
}
