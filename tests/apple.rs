use mindbase::*;

#[test]
fn apple() -> Result<(), MBError> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    // Questions / notes:
    // - Where precisely is the taxonomy?
    // – How do we navigate this?
    // - What *exactly* do $d, $k, $p etc represent? -- How do we bind to the desired subsymbol in a grounding? (No really: ?)
    // - Something need to make the Token "weightier" when it comes time to render this, but why? --  Adding emphasis or schematic
    //   annotation to this effect is clearly wrong, tantamount to putting one's thumb on the scale
    // - How do we do N dimensional analogical grounding rather than 1 dimensional?
    // - It seems we need some way to break associativity, and refer to the symbol itself -- .left creates a new composite symbol
    //   of left-handed branch sub-symbols (abbreviated as <) -- .right creates a new composite symbol left-handed branch
    //   sub-symbols (abbreviated as >) -- Thus $7.right.right (abbreviated as $7>>) should yield the unique symbol which is
    //   tagged with "Malus Domestica" (and which may be queried by Ground("Token":"Type") to yeild the unique symbol tagged with
    //   "Species" )

    // TODO 1 - determine if subsymbol binding is compatible with resymbolizion in ground.rs - It should be
    // TODO 1 - determine how we might be able to ground in multiple dimensions. Maybe Symbolvars need to perform lazy
    // symbolization after all? Perhaps lazy symbolvar plus AND? Eg: Ground("Bar" : $serves_booze & !$restaurant)
    // TODO 1 - what types of analogies do we want to represent?
    //     catagorical ~
    //     associative :
    //     what others?
    //     Eg: $pets = Ground("Dog" ~ "Cat") # Find a preexisting symbol which correlates Dog and Cat OR create one
    //     Eg: $petz = Ground(("Dog"? : "Doggy") : ("Cat"? : "Kitty" )) # Extract "Dog" and "Cat" into a single symbol
    //              QUESTION: How does $pets compare to $petz? Should they be identical? or different?
    //     Eg: $shmoopy_pets = Ground(("Cat" : "Kitty"?) : ("Dog" : "Doggy"?))
    //     Eg: ("Cat" "Kitty") : ("Synonyms")
    //     Eg: ("The tree fruit relationship as a single entity" ~ ("Tree" : "Fruit"))

    //    ("Smirk") ~ ("Smile"? : "Mouth")
    //    ("Smirk") :

    let query = mb.query_str(
                             r#"

        # Alice has an apple

        $tt = Ground("Type" : "Token")? # Trailing ? is the default if omitted
        $hh = Ground("Holonym" : "Hyponym")?
        $euk = Lazy("Eukarya")
        $d = Ground( $tt : ("Domain"  : $euk))
        $k = Ground( $tt : ("Kingdom" : "Plantae"?))
        $p = Ground( $tt : ("Phylum"  : "Magnoliophyta"?))
        $c = Ground( $tt : ("Class"   : "Magnoliopsida"?))
        $o = Ground( $tt : ("Order"   : "Rosales"?))
        $f = Ground( $tt : ("Family"  : "Rosaceae"?))
        $g = Ground( $tt : ("Genus"   : "Malus"?))
        $s = Ground( $tt : ("Species" : "Malus domestica"?))
        $1 = Ground( $hh : ( $d : $k? ) )
        $2 = Ground( $hh : ( $c : $p? ) )
        $3 = Ground( $hh : ( $o : $c? ) )
        $4 = Ground( $hh : ( $f : $o? ) )
        $5 = Ground( $hh : ( $g : $f? ) )
        $6 = Ground( $hh : ( $5 : $g? ) )
        $apple = Ground( $hh : ( $6 : $s? ) )

        # Fruit is hyponymous of Tree _only_ when it's still attached
        $tfhh_f = Ground(("Tree" : "Fruit"?) : $hh)
        $tfpp_f = Ground(("Tree" : "Fruit"?) : ("Progenitor" : "Progeny"))

        # Now we have a term that refers to fruit which may be ON or OF the tree
        $fruit = $tfhh_f | $tfpp_f

        # I think we need to have a syntax to navigate subsymbols even though ? binding is cleaner
        $mdappletreefruit = Ground(($7>> : "Apple") : $tf)  # >> is abbreviated form of .right.right
        $apple = $mdsappletreefruit<>                       # <> is abbreviated form of of .left.right

        # Identical to the above
        $apple2 = Ground(($7>> : "Apple"?) : $treefruit)
        Assert($apple = $apple2)

        $pos = Ground("Posessor":"Possession")
        
        # How do we do binding of ALL of $apple to $t<, and return a new symbol for $t>? 
        $aa = Allege( "Alice" : ($t > *$apple) : $pos )

        $personsaid = Ground("")
        Allege($apple : $s>>)


        ("English Word" : "Apple") : ("Species": "Malus Domestica" )

        # Not yet sure how to deal with these
        # $x = Ground($h : ("Ontological System" : "Biological Taxonomy"))
        # $e = Ground($x : ("System" : "Element") )
        # Ground($x : $d) # This seems screwy
    "#,
    )?;
    query.apply()?;

    let apple = query.get_symbol_var("apple")?.unwrap();

    // I have three apples
    //

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
