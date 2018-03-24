use std::iter::IntoIterator;
use std::str::FromStr;

type Iter<'a> = ::std::slice::Iter<'a, String>;

#[derive(Debug, PartialEq)]
pub struct NodeSpec {
    path: Vec<String>,
}

impl NodeSpec {
    pub fn iter(&self) -> Iter {
        self.path.iter()
    }
}

impl IntoIterator for NodeSpec {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl FromStr for NodeSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path: Vec<String> =
            if s == "." {
                vec![]
            } else {
                s.split('.')
                    .map(|s| s.to_string())
                    .collect()
            };

        Ok(NodeSpec {
            path,
            // shift_index: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_nodespec() {
        assert_eq!("foo".parse::<NodeSpec>().map(|n| n.path), Ok(vec![ "foo".into() ]));
        assert_eq!("bar".parse::<NodeSpec>().map(|n| n.path), Ok(vec![ "bar".into() ]));

        assert_eq!("foo.bar".parse::<NodeSpec>().map(|n| n.path),
            Ok(vec![ "foo".into(), "bar".into() ]));
    }

    // #[test]
    // fn peek_and_shift() {
    //     let mut ns: NodeSpec = "foo.bar.foobar".parse().unwrap();
    //     assert_eq!(ns.peek(),  Some(&"foo".to_string()));
    //     assert_eq!(ns.shift(), Some(&"foo".to_string()));
    //     assert_eq!(ns.shift(), Some(&"bar".to_string()));
    //     assert_eq!(ns.peek(),  Some(&"foobar".to_string()));
    //     assert_eq!(ns.shift(), Some(&"foobar".to_string()));
    //     assert_eq!(ns.peek(),  None);
    //     assert_eq!(ns.shift(), None);
    // }
}
