use mindbase_core::{
    analogy::Analogy,
    artifact::{text, Text},
    *,
};

use keyplace::KeyManager;
use rustyline::{error::ReadlineError, Editor};

pub fn run(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    // TODO 1 - LEFT OFF HERE - Update core to use crypto
    let agent_key = match keymanager.current_agent_key()? {
        Some(agent_key) => agent_key,
        None => {
            println!("No default agent found. use `mbcli auth login` or `mbcli auth create`");
            return Ok(());
        }
    };

    // What situations might have precipitated that would lead me to conjuring a non-narrow symbol?

    // A: Hey, do you want to take a [trip1] with me? -- Narrow symbol conjured from new allegation I just made
    // B: Sure, I'll take a [trip2,trip1] with you    -- Create a new allegation to represent interpreted meaning (same artifact)
    // A: What's a good day for our [trip1,trip2]?    -- "trip" Symbol broadens to 2 allegations
    // B: How about tuesday? A: Great, I'll get the psilocybin
    // B: Whoa, I thought you meant a [trip2] not a [trip1] -- {negative analogy between [trip1] and [trip2]}

    // Things that I said
    // Where "I" is my agent ( Agent is an allegation too? )
    // (bit of a bootstrapping dilemma here)

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mbcli_history").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                // let statement = mb.alledge(Text::new(&line))?;
                // let analogy = mb.alledge(Analogy::declarative(statement.subjective(), isaid.clone()))?;

                // TODO 3 - create a linkage between this allegation and the previous one:
                // * [A1] Screw you
                // * [A2 ]...and the horse you rode in on (in the category of [things that follow A1])
                rl.add_history_entry(line.as_str());
                // println!("{}", analogy);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".mindbasecli_history").unwrap();

    Ok(())
}
