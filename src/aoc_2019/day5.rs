use crate::aoc_2019::intcode_cpu::IntCPU;

const INPUT: &str = include_str!("./inputs/day5.txt");

#[test]
fn solutions() {
    let program = IntCPU::parse_program(INPUT);

    let mut cpu = IntCPU::new(&program);
    cpu.push_input(1);
    cpu.exec();
    assert_eq!(Some(13285749), cpu.last_output());

    let mut cpu = IntCPU::new(&program);
    cpu.push_input(5);
    cpu.exec();
    assert_eq!(Some(5000972), cpu.last_output());
}
