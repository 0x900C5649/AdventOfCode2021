
use AoC_11::part2;
use AoC_11::Args;

#[test]
fn p2() {
    let arg = Args{part : 2, input : "input/test-energy.txt".to_string()};
    assert_eq!(part2(arg), 195);
}
