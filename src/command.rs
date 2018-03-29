use nodespec::NodeSpec;
use std::str::FromStr;
use value::ValType;

#[derive(Debug, PartialEq)]
pub enum Command {
    Create(NodeSpec, String, ValType),
    Read(NodeSpec),
    Update(NodeSpec, String),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_pos = s.find(' ');
        let command = &s[0..space_pos.unwrap_or(s.len())];
        let mut args = match space_pos {
            Some(pos) => s[pos+1 .. s.len()].split(' ').collect::<Vec<_>>().into_iter(),
            None      => vec![].into_iter(),
        };

        match command {
            "create" => {
                let nodespec = args.next().ok_or("missing nodespec for 'create'")?.parse()?;
                let name     = args.next().ok_or("missing name for 'create'")?.to_string();
                let valtype  = args.next().ok_or("missing valtype for 'create'")?.parse()?;
                if args.next().is_some() { return Err("too many arguments"); }
                Ok(Command::Create(nodespec, name, valtype))
            },
            "read" => {
                let nodespec = args.next().ok_or("missing nodespec for 'read'")?.parse()?;
                if args.next().is_some() { return Err("too many arguments"); }
                Ok(Command::Read(nodespec))
            },
            "update" => {
                let nodespec = args.next().ok_or("missing nodespec for 'update'")?.parse()?;
                let value    = args.next().ok_or("missing value for 'update'")?.to_string();
                if args.next().is_some() { return Err("too many arguments"); }
                Ok(Command::Update(nodespec, value))
            },
            _ => Err("unknown command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_read_command() {
        assert!("read".parse::<Command>().is_err());

        assert_eq!("read foo.bar".parse(), Ok(Command::Read("foo.bar".parse().unwrap())));
    }

    #[test]
    fn parse_update_command() {
        assert!("update".parse::<Command>().is_err());
        assert!("update foo".parse::<Command>().is_err());

        assert_eq!("update foo bar".parse(), Ok(Command::Update("foo".parse().unwrap(), "bar".to_string())));
    }
}
