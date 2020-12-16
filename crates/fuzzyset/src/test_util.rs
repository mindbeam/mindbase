use crate::{polar::PolarFuzzySet, traits::Member};

/// Simple Entity which can be used for unit tests
#[derive(Debug, Clone)]
pub struct SimpleMember {
    pub id: &'static str,
    pub text: &'static str,
    pub set: Option<PolarFuzzySet<Self>>,
}
impl Ord for SimpleMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(other.id)
    }
}
impl PartialOrd for SimpleMember {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(other.id)
    }
}
impl Eq for SimpleMember {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialEq for SimpleMember {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(other.id)
    }
}

impl Member for SimpleMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(other.id)
    }

    fn display_fmt(&self, item: &crate::fuzzyset::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{:0.1}", item.member, item.degree)
    }
}

impl std::fmt::Display for SimpleMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<(&'static str, PolarFuzzySet<Self>)> for SimpleMember {
    fn from(tup: (&'static str, PolarFuzzySet<Self>)) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&tup.0).unwrap().get(1).unwrap().as_str();

        SimpleMember {
            id: tup.0,
            text,
            set: Some(tup.1),
        }
    }
}

impl From<&'static str> for SimpleMember {
    fn from(id: &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleMember { id, text, set: None }
    }
}
impl From<&'static &'static str> for SimpleMember {
    fn from(id: &'static &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleMember { id, text, set: None }
    }
}

impl<T> From<&(T, f32)> for crate::fuzzyset::Item<SimpleMember>
where
    T: Into<SimpleMember> + Clone,
{
    fn from(tuple: &(T, f32)) -> Self {
        crate::fuzzyset::Item {
            member: tuple.0.clone().into(),
            degree: tuple.1,
        }
    }
}

impl<T> From<&T> for crate::fuzzyset::Item<SimpleMember>
where
    T: Into<SimpleMember> + Clone,
{
    fn from(member: &T) -> Self {
        crate::fuzzyset::Item {
            member: member.clone().into(),
            degree: 1.0,
        }
    }
}
