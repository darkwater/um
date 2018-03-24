use node::Node;
use std::collections::HashMap;
use std::str::FromStr;

pub type Map = HashMap<String, Node>;

#[derive(Debug)]
pub enum Value {
    Empty,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Map(Map),
}

#[derive(Debug, PartialEq)]
pub enum ValType {
    Empty,
    Boolean,
    Integer,
    Float,
    String,
    Map,
}

impl Value {
    pub fn valtype(&self) -> ValType {
        match *self {
            Value::Empty      => ValType::Empty,
            Value::Boolean(_) => ValType::Boolean,
            Value::Integer(_) => ValType::Integer,
            Value::Float(_)   => ValType::Float,
            Value::String(_)  => ValType::String,
            Value::Map(_)     => ValType::Map,
        }
    }

    pub fn from_str(s: &str, valtype: &ValType) -> Result<Value, &'static str> {
        Ok(match *valtype {
            ValType::Empty   => Value::Empty,
            ValType::Boolean => Value::Boolean(s.parse().map_err(|_| "invalid boolean")?),
            ValType::Integer => Value::Integer(s.parse().map_err(|_| "invalid integer")?),
            ValType::Float   => Value::Float(s.parse().map_err(|_| "invalid float")?),
            ValType::String  => Value::String(s.to_string()),
            ValType::Map     => return Err("can't update a map node"),
        })
    }
}

impl FromStr for ValType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "empty"   => Ok(ValType::Empty),
            "boolean" => Ok(ValType::Boolean),
            "integer" => Ok(ValType::Integer),
            "float"   => Ok(ValType::Float),
            "string"  => Ok(ValType::String),
            "map"     => Ok(ValType::Map),
            _         => Err("invalid type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_nodespec() {
    }
}
