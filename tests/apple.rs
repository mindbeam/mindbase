use mindbase::*;

#[test]
fn apple() -> Result<(), MBError> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    let query = mb.query_str(
                             r#"
        $t = Ground("Type" : "Token")
        $h = Ground("Holonym" : "Hyponym")

        $d = Ground($t : ("Domain"  : "Eukarya"))
        $k = Ground($t : ("Kingdom" : "Plantae"))
        $p = Ground($t : ("Phylum"  : "Magnoliophyta"))
        $c = Ground($t : ("Class"   : "Magnoliopsida"))
        $o = Ground($t : ("Order"   : "Rosales"))
        $f = Ground($t : ("Family"  : "Rosaceae"))
        $g = Ground($t : ("Genus"   : "Malus"))
        $s = Ground($t : ("Species" : "Malus domestica"))

        $1 = Ground( $h : ( $d : $k ) )
        $2 = Ground( $h : ( $1 : $p ) )
        $3 = Ground( $h : ( $2 : $c ) )
        $4 = Ground( $h : ( $3 : $o ) )
        $5 = Ground( $h : ( $4 : $f ) )
        $6 = Ground( $h : ( $5 : $g ) )
        $7 = Ground( $h : ( $5 : $s ) )

        let $apple = Ground(("English Word" : "Apple") : $7 )

        # Not yet sure how to deal with these
        # $x = Ground($h : ("Ontological System" : "Biological Taxonomy"))
        # $e = Ground($x : ("System" : "Element") )
        # Ground($x : $d) # This seems screwy
    "#,
    )?;
    query.apply()?;

    let apple = query.get_symbol_var("apple")?.unwrap();

    // TODO 1 - fix this

    // let malus_domestica1 = mb.get_ground_symbol(vec![,])?;

    // let malus_domestica2 = mb.get_ground_symbol(vec!["Biological Taxonomy",
    //                                                  "Domain: Eukarya",
    //                                                  "Kingdom: Plantae",
    //                                                  "Phylum: Magnoliophyta",
    //                                                  "Class: Magnoliopsida",
    //                                                  "Order: Rosales",
    //                                                  "Family: Rosaceae",
    //                                                  "Genus: Malus",
    //                                                  "Species: Malus domestica",])?;

    // assert_eq!(malus_domestica1, malus_domestica2);

    // let tree = mb.get_ground_symbol(vec!["Plant", "Tree"])?;
    // let fruit = mb.get_ground_symbol(vec!["Fruit"])?;

    // A Tree is a
    //   "Type of Plant"
    //   With "an elongated stem or trunk",
    //   And "has branches and leaves",
    //
    // mb.alledge(Analogy::declarative(malus_domestica1.clone(), tree.clone()))?;
    //
    // A fruit is a
    //   "seed-bearing structure",
    //   "of a flowering plant",
    //   "formed from the ovary after flowering"

    Ok(())
}
