use structopt::StructOpt;

use dialoguer::{theme::ColorfulTheme, Input, Password};
use mindbase_core::MindBase;
use mindbase_crypto::{AgentKey, KeyManager, PassKey};

#[derive(StructOpt, Debug)]
pub enum Command {
    Create,
    Login,
    Logout,
}

pub(crate) fn run(mb: MindBase, keymanager: KeyManager, cmd: Command) -> Result<(), std::io::Error> {
    match cmd {
        Command::Create => create(mb, keymanager),
        Command::Login => login(mb, keymanager),
        Command::Logout => logout(mb, keymanager),
    }
}

fn create(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    // let (email, password) = get_email_and_password();

    println!("TODO 1: Update this to register with the server");

    let agentkey = AgentKey::create();
    keymanager.put_agent_key(agentkey)?;

    Ok(())
}

fn login(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    let (email, password) = get_email_and_password();

    unimplemented!()
}

fn logout(mb: MindBase, keymanager: KeyManager) -> Result<(), std::io::Error> {
    let (email, password) = get_email_and_password();

    unimplemented!()
}

fn get_email_and_password() -> (String, String) {
    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("This is not an email address")
            }
        })
        .interact_text()
        .unwrap();
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        // .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    (email, password)
}
