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

    let isaid = mb.alledge_artifact(&agent, FlatText::new("Things that I said".to_string()))?;

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mindbasecli_history").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let statement = mb.alledge_artifact(&agent, FlatText::new(line.clone()))?;

                let allegation = Allegation::new(&agent, Analogy::declare(statement.narrow(), isaid.narrow()))?;
                mb.put_allegation(&allegation)?;

                // TODO 3 - create a linkage between this allegation and the previous one:
                // * [A1] Screw you
                // * [A2 ]...and the horse you rode in on (in the category of [things that follow A1])
                rl.add_history_entry(line.as_str());
                println!("{}", allegation);
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
