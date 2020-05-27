// IMPORTANT NOTE: In this experiment, we are using string in lieu of unique identifier.
// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SimpleId {
    pub id:   &'static str,
    pub text: &'static str,
}

impl From<&'static str> for SimpleId {
    fn from(id: &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleId { id, text }
    }
}
