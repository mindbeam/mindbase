use mindbase::*;
use rustyline::{
    error::ReadlineError,
    Editor,
};

fn main() -> Result<(), std::io::Error> {
    let dir = std::env::current_dir().unwrap();
    println!("Loading database in {}", dir.as_path().display());
    let mb = MindBase::open(&dir.as_path()).unwrap();

    let agent = mb.default_agent().unwrap();
    println!("Using Agent {}", agent);

    // TODO 1 - Look this artifact up based on my agent ID
    let isaid = mb.make_artifact(ArtifactKind::FlatText(FlatText { text: "Things that I said".to_string(), }))
                  .unwrap();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mindbasecli_history").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let statement = mb.make_artifact(ArtifactKind::FlatText(FlatText { text: line.clone() }))
                                  .unwrap();

                let allegation = mb.alledge(&agent, Analogy::declare(statement.narrow_concept(), isaid.narrow_concept()))
                                   .unwrap();

                rl.add_history_entry(line.as_str());
                println!("{}", allegation.to_string(),);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".mindbasecli_history").unwrap();

    Ok(())
}
