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

    /// Commands are structured as follows:
    /// 
    ///     command argument argument :final argument
    /// 
    /// The command and arguments are separated with a single space. If the final argument
    /// is prefixed with a colon (:), it may contain spaces. Normal arguments may not.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_space_pos = s.find(' ');    // marks the end of the command
        let last_argument_pos = s.find(" :"); // marks the start of the last argument

        let mid_args: Option<&str> = first_space_pos.filter(|_| last_argument_pos.is_none() ||
                                                                last_argument_pos > first_space_pos)
                                        .map(|p| &s[p+1 .. last_argument_pos.unwrap_or(s.len())]);
        let last_arg: Option<&str> = last_argument_pos.map(|p| &s[p+2 .. s.len()]);

        let command  = &s[0..first_space_pos.unwrap_or(s.len())];
        let mut args = mid_args.into_iter().flat_map(|a| a.split(' '))
                           .chain(last_arg.into_iter());

        match command {
            "create" => {
                let nodespec = args.next().ok_or("missing nodespec (1st argument)")?.parse()?;
                let name     = args.next().ok_or("missing name (2nd argument)")?.to_string();
                let valtype  = args.next().ok_or("missing valtype (3rd argument)")?.parse()?;
                if args.next().is_some() { return Err("too many arguments (expected 3)"); }
                Ok(Command::Create(nodespec, name, valtype))
            },
            "read" => {
                let nodespec = args.next().ok_or("missing nodespec (1st argument)")?.parse()?;
                if args.next().is_some() { return Err("too many arguments (expected 1)"); }
                Ok(Command::Read(nodespec))
            },
            "update" => {
                let nodespec = args.next().ok_or("missing nodespec (1st argument)")?.parse()?;
                let value    = args.next().ok_or("missing value (2nd argument)")?.to_string();
                if args.next().is_some() { return Err("too many arguments (expected 2)"); }
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
        assert_eq!("read :foo.bar".parse(), Ok(Command::Read("foo.bar".parse().unwrap())));
    }

    #[test]
    fn parse_update_command() {
        assert!("update".parse::<Command>().is_err());
        assert!("update foo".parse::<Command>().is_err());
        assert!("update foo hello world".parse::<Command>().is_err());

        assert_eq!("update foo bar".parse(),
            Ok(Command::Update("foo".parse().unwrap(), "bar".to_string())));

        assert_eq!("update foo :hello world".parse(),
            Ok(Command::Update("foo".parse().unwrap(), "hello world".to_string())));
    }
}
