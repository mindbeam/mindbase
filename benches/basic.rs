use mindbase::{
    artifact::text,
    Analogy,
    Concept,
    Error,
    MindBase,
};

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

fn insert_test_dataset(mb: &MindBase) -> Result<(), Error> {
    for _i in 0..50 {
        let mut last_concept: Option<Concept> = None;
        // println!("Loop {}", _i);
        for letter in (b'A'..=b'Z').map(|v| String::from_utf8(vec![v]).unwrap()) {
            let concept = mb.alledge(text(&letter))?.subjective();

            if let Some(parent) = last_concept.take() {
                mb.alledge(Analogy::declarative(concept.clone(), parent))?;
            }

            last_concept = Some(concept);
        }
    }

    Ok(())
}

fn get_ground_concept(mb: &MindBase) -> Result<(), Error> {
    let _concept1: Concept = mb.get_ground_concept(vec!["A", "B", "C", "D"])?;
    let _concept2: Concept = mb.get_ground_concept(vec!["Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"])?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath).unwrap();

    insert_test_dataset(&mb).unwrap();

    // c.bench_function("insert_test_dataset", |b| b.iter(|| insert_test_dataset(&mb).unwrap()));
    c.bench_function("get_ground_concept", |b| b.iter(|| get_ground_concept(&mb).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
