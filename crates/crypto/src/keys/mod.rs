pub(crate) mod custodian;
pub(crate) mod private;
pub(crate) mod public;

pub use custodian::*;
pub use private::*;
pub use public::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keys_basic() {
        let agentkey = AgentKey::create();

        let passkey = PassKey::new("I like turtles");
        let custkey = agentkey.custodial_key(passkey);
        let passkey2 = PassKey::new("I like turtles");
        let agentkey2 = AgentKey::from_custodial_key(custkey, passkey2).unwrap();

        assert!(agentkey == agentkey2);
    }

    #[test]
    fn wrong_passphrase() {
        let agentkey = AgentKey::create();

        let passkey = PassKey::new("I like turtles");
        let custkey = agentkey.custodial_key(passkey);
        let passkey2 = PassKey::new("I like turtleb");

        let should_be_error = AgentKey::from_custodial_key(custkey, passkey2);
        match should_be_error {
            Ok(_) => panic!("This is not supposed to succeed"),
            Err(crate::Error::Mac(_)) => {
                // All good
            }
            _ => panic!("Not supposed to get here"),
        }
    }
}
