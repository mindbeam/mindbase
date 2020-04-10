use mindbase::*;

#[test]
fn apple() -> Result<(), MBError> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    let malus_domestica1 = mb.get_ground_concept(vec!["Biological Taxonomy",
                                                      "Domain: Eukarya",
                                                      "Kingdom: Plantae",
                                                      "Phylum: Magnoliophyta",
                                                      "Class: Magnoliopsida",
                                                      "Order: Rosales",
                                                      "Family: Rosaceae",
                                                      "Genus: Malus",
                                                      "Species: Malus domestica",])?;

    let malus_domestica2 = mb.get_ground_concept(vec!["Biological Taxonomy",
                                                      "Domain: Eukarya",
                                                      "Kingdom: Plantae",
                                                      "Phylum: Magnoliophyta",
                                                      "Class: Magnoliopsida",
                                                      "Order: Rosales",
                                                      "Family: Rosaceae",
                                                      "Genus: Malus",
                                                      "Species: Malus domestica",])?;

    assert_eq!(malus_domestica1, malus_domestica2);

    // let tree = mb.get_ground_concept(vec!["Plant", "Tree"])?;
    // let fruit = mb.get_ground_concept(vec!["Fruit"])?;

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
