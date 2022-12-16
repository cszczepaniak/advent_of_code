use advent::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! benchmark_day {
    ($c:expr, day $day:literal, $f1:expr, $f2:expr) => {
        let mut group = $c.benchmark_group(format!("day{}", $day));

        let input = common::network::get_input(2022, $day).unwrap();
        group.bench_function("part1", |b| b.iter(|| $f1(black_box(&input))));
        group.bench_function("part2", |b| b.iter(|| $f2(black_box(&input))));
        group.finish();
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    benchmark_day!(c, day 1, day1::part_one, day1::part_two);
    benchmark_day!(c, day 2, day2::part_one, day2::part_two);
    benchmark_day!(c, day 3, day3::part_one, day3::part_two);
    benchmark_day!(c, day 4, day4::part_one, day4::part_two);
    benchmark_day!(c, day 5, day5::part_one, day5::part_two);
    benchmark_day!(c, day 6, day6::part_one, day6::part_two);
    benchmark_day!(c, day 7, day7::part_one, day7::part_two);
    benchmark_day!(c, day 8, day8::part_one, day8::part_two);
    benchmark_day!(c, day 9, day9::part_one, day9::part_two);
    benchmark_day!(c, day 10, day10::part_one, day10::part_two);
    benchmark_day!(c, day 11, day11::part_one, day11::part_two);
    benchmark_day!(c, day 12, day12::part_one, day12::part_two);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
