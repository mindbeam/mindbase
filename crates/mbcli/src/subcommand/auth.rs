use structopt::StructOpt;

use dialoguer::{theme::ColorfulTheme, Input, Password};
use mindbase_core::MindBase;
use mindbase_crypto::{AgentKey, KeyManager, PassKey};

#[derive(StructOpt, Debug)]
pub enum Command {
    Show,
    Select {
        /// Identity or email
        search: Option<String>,
    },
    Create,
    Login,
    Logout,
    Reset,
}

pub(crate) fn run(mb: MindBase, keymanager: KeyManager, cmd: Command) -> Result<(), std::io::Error> {
    match cmd {
        Command::Create => create(mb, keymanager),
        Command::Login => login(mb, keymanager),
        Command::Logout => logout(mb, keymanager),
        Command::Show => show(mb, keymanager),
        Command::Reset => reset(mb, keymanager),
        Command::Select { search: ident } => select(mb, keymanager, ident),
    }
}

fn create(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    let email = get_email();

    println!("TODO 1: Update this to register with the server");

    let agentkey = AgentKey::create(Some(email));
    let id = agentkey.id();
    keymanager.put_agent_key(agentkey)?;
    keymanager.set_current_agent(id)?;

    Ok(())
}

fn login(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    let email = get_email();
    let password = get_password();

    unimplemented!("login (key recovery with the aid of a custodial server) is not yet implemented")
}

fn logout(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    unimplemented!()
}

fn show(_mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    let agents = keymanager.list_agents()?;

    let mut selected = 0usize;

    if let Some(current) = keymanager.current_agent_key()? {
        let pubkey = current.pubkey();
        if let Some((i, _)) = agents.iter().enumerate().find(|(i, a)| a.pubkey == pubkey) {
            selected = i;
        }
    }

    for (i, id) in agents.iter().enumerate() {
        if i == selected {
            println!("> {}", id);
        } else {
            println!("  {}", id);
        }
    }

    Ok(())
}
fn select(_mb: MindBase, keymanager: KeyManager, search: Option<String>) -> Result<(), std::io::Error> {
    let agents = keymanager.list_agents()?;

    if let Some(search) = search {
        if let Some((i, _)) = agents
            .iter()
            .enumerate()
            .find(|(_, a)| a.pubkey_base64().starts_with(&search) || a.email.as_ref() == Some(&search))
        {
            keymanager.set_current_agent(agents[i].clone())?;
            println!("set current agent to {}", agents[i]);
        } else {
            println!("Agent not found");
        }
    } else {
        // let items = vec!["Item 1", "item 2"];
        let mut selected = 0usize;

        if let Some(current) = keymanager.current_agent_key()? {
            let pubkey = current.pubkey();
            if let Some((i, _)) = agents.iter().enumerate().find(|(i, a)| a.pubkey == pubkey) {
                selected = i;
            }
        }

        let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .items(&agents)
            .default(selected)
            .interact_on_opt(&dialoguer::console::Term::stderr())?;

        if let Some(selection) = selection {
            keymanager.set_current_agent(agents[selection].clone())?;
        }
    }

    Ok(())
}

fn reset(_mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    keymanager.remove_all_agent_keys()?;
    println!("All agent keys removed from keymanager");
    Ok(())
}

fn get_email() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("This is not an email address")
            }
        })
        .interact_text()
        .unwrap()
}

fn get_password() -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        // .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap()
}
