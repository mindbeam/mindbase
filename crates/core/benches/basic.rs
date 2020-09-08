use criterion::{criterion_group, criterion_main, Criterion};
use mindbase_core::{artifact::text, prelude::*};

fn insert_test_dataset(mb: &MindBase) -> Result<(), MBError> {
    for _i in 0..50 {
        let mut last_symbol: Option<Symbol> = None;
        // println!("Loop {}", _i);
        for letter in (b'A'..=b'Z').map(|v| String::from_utf8(vec![v]).unwrap()) {
            let symbol = mb.alledge(text(&letter))?.subjective();

            if let Some(parent) = last_symbol.take() {
                mb.alledge(Analogy::declarative(symbol.clone(), parent))?;
            }

            last_symbol = Some(symbol);
        }
    }

    Ok(())
}

fn get_ground_symbol(mb: &MindBase) -> Result<(), MBError> {
    unimplemented!()
    // let _symbol1: Symbol = mb.get_ground_symbol(vec!["A", "B", "C", "D"])?;
    // let _symbol2: Symbol = mb.get_ground_symbol(vec!["Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"])?;
    // Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath).unwrap();

    insert_test_dataset(&mb).unwrap();

    // c.bench_function("insert_test_dataset", |b| b.iter(|| insert_test_dataset(&mb).unwrap()));
    c.bench_function("get_ground_symbol", |b| {
        b.iter(|| get_ground_symbol(&mb).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
