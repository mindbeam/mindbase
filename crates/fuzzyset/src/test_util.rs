use crate::traits::Member;

/// Simple Entity which can be used for unit tests
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SimpleMember {
    pub id: &'static str,
    pub text: &'static str,
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

impl From<&'static str> for SimpleMember {
    fn from(id: &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleMember { id, text }
    }
}
impl From<&'static &'static str> for SimpleMember {
    fn from(id: &'static &'static str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"([^\d]+)\d*").unwrap();
        let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

        SimpleMember { id, text }
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
