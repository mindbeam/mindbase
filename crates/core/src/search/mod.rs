mod context;
mod node;

pub use context::SearchContext;
pub use node::SearchNode;

#[cfg(test)]
mod test {
    use crate::{
        mbql::{
            error::{
                MBQLError,
                MBQLErrorKind,
            },
            Query,
        },
        MindBase,
    };
    use std::io::Cursor;

    #[test]
    fn ground1() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let query = mb.query_str(r#"Ground!(("Smile" : "Mouth") : ("Wink" : "Eye"))"#)?;
        match query.apply() {
            Err(MBQLError { kind: MBQLErrorKind::GSymNotFound,
                            .. }) => {
                // This should fail, because we're disallowing vivification
            },
            r @ _ => panic!("Ground symbol vivification is disallowed {:?}", r),
        }

        let query = mb.query_str(
                                 r#"
        $foo = Allege(("Smile" : "Mouth") : ("Wink":"Eye"))
        $bar = Ground!(("Smile" : "Mouth") : ("Wink" : "Eye"))
        Diag($foo, $bar)
        "#,
        )?;

        // This time it should work, because we are alleging it above in what happens to be exactly the right way to be matched by
        // the ground symbol search
        query.apply()?;

        let bogus = query.get_symbol_for_var("bogus")?;
        assert_eq!(bogus, None);

        let foo = query.get_symbol_for_var("foo")?.expect("foo");
        let bar = query.get_symbol_for_var("bar")?.expect("bar");

        assert_eq!(foo, bar);
        assert!(foo.intersects(&bar));

        Ok(())
    }

    #[test]
    fn ground2() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let mbql = Cursor::new(r#"$gs = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))"#);

        let query = Query::new(&mb, mbql)?;
        query.apply()?;

        let _gs = query.get_symbol_for_var("gs")?.expect("gs");

        Ok(())
    }

    #[test]
    fn ground3() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let mbql = Cursor::new(
                               r#"
            $foo = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
            $bar = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
            Diag($foo, $bar)
        "#,
        );

        let query = Query::new(&mb, mbql)?;
        query.apply()?;

        let foo = query.get_symbol_for_var("foo")?.expect("foo");
        let bar = query.get_symbol_for_var("bar")?.expect("bar");

        assert_eq!(foo, bar);
        assert!(foo.intersects(&bar));

        Ok(())
    }

    #[test]
    fn ground4() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        // $a = Allege("Raggedy Ann": "Ragdoll")
        // $b = Allege("Raggedy Andy": "Ragdoll")

        // WIP: How to validate that we're properly re-symbolizing?
        let query = mb.query_str(
                                 r#"
            $a = Allege("Ragdoll" : "Leopard")
            $b = Allege("Shepherd" : "Wolf")
            $c = Allege($a : $b)
            $x = Ground(("Ragdoll" : "Leopard") : ("Shepherd" : "Wolf"))
            Diag($a, $b, $x)
        "#,
        )?;

        query.apply()?;

        let a = query.get_symbol_for_var("a")?.expect("a");
        let b = query.get_symbol_for_var("b")?.expect("b");
        let x = query.get_symbol_for_var("x")?.expect("x");

        let lr = x.left_right(&mb)?.expect("left/right referents");

        assert_eq!(a, lr.0);
        assert_eq!(b, lr.1);

        // let stdout = std::io::stdout();
        // let handle = stdout.lock();
        // crate::xport::dump_json(&mb, handle).unwrap();

        // let foo = query.get_symbol_var("foo")?.expect("foo");
        // let bar = query.get_symbol_var("bar")?.expect("bar");

        // assert_eq!(foo, bar);
        // assert!(foo.intersects(&bar));

        Ok(())
    }
}
