
use AoC_11::part1;
use AoC_11::Args;

#[test]
fn p1() {
    let arg = Args{part : 1, input : "input/test-energy.txt".to_string()};
    assert_eq!(part1(arg), 1656);
}
