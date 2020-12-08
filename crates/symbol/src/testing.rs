use crate::traits::Entity;

// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"

/// Simple Entity which can be used for unit tests
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SimpleEntity {
    pub id: &'static str,
    pub text: &'static str,
}

impl Entity for SimpleEntity {}

impl From<&'static str> for SimpleEntity {
    fn from(id: &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleEntity { id, text }
    }
}
impl From<&'static &'static str> for SimpleEntity {
    fn from(id: &'static &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleEntity { id, text }
    }
}

impl std::fmt::Display for SimpleEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
