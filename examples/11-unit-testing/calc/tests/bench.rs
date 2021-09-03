#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_add(b: &mut Bencher) {
    b.iter(|| calc::calc::add(1000, 11121124))
}
