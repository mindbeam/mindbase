// #[cfg(test)]
mod test {
    use std::{
        collections::{btree_map::Entry, BTreeMap},
        sync::{Arc, Mutex},
    };

    use mindbase_artifact::{body::DataNode, Artifact, ArtifactId};
    use serde::Serialize;

    #[test]
    fn sprightly() {
        // The purpose of this test is to represent a simple poem
        // It may be rendered as a simple column of text, or it may be exploded and rearranged to show relationships

        //  [[The cat] rawred triumphently]
        //        |
        //   [As [he] did [zoomies]]
        //                      |
        // [Skittering across the [floor]]
        //              /- (of the)-/
        // [From [kitchen] to bedroom door]

        let mut graph = Graph::<Artifact<Kind, Instance>>::default();
        // TODO 1 - Left off here. How do we break these down? Should this be a kind of markup?
        let line1 = graph.put_artifact(text("The cat rawred triumphently"));
        // [The cat](c) [rawred triumphently]
        let line2 = graph.put_artifact(text("As he did zoomies"));
        // [As] [he](c) [did zoomies]
        let line3 = graph.put_artifact(text("Skittering across the floor"));
        // [Skittering across] [the floor](f)
        let line4 = graph.put_artifact(text("From kitchen to bedroom door"));
        // [From] [kitchen](f) [to bedroom door]
    }
    fn text(txt: &str) -> Artifact<Kind, Instance> {
        DataNode {
            data_type: Kind::Text,
            data: Some(txt.as_bytes().to_vec()),
        }
        .into()
    }

    #[derive(Clone, Serialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
    struct Instance {
        id: usize,
        artifact_id: ArtifactId,
    }
    #[derive(Clone, Serialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
    enum Kind {
        Text,
        NounEntity,
        RelNext,
        RelPrev,
    }

    impl NodeType for Kind {}
    impl NodeInstance for Instance {}

    lazy_static! {
        static ref INCREMENT: Arc<Mutex<usize>> = Arc::new(Mutex::new(0usize));
    }

    impl Instance {
        pub fn new(artifact_id: ArtifactId) -> Self {
            let mut inc = INCREMENT.lock().unwrap();
            let id = *inc;
            *inc += 1;

            Instance { id, artifact_id }
        }
    }
    impl std::fmt::Display for Instance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}~{}", self.id, self.artifact_id)
        }
    }
}
