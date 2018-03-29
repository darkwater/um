use value::{Map, ValType, Value};

#[derive(Debug)]
pub struct Node {
    value: Value,
}

impl Node {
    pub fn with_value(value: Value) -> Node {
        Node {
            value,
        }
    }

    pub fn with_type(valtype: &ValType) -> Node {
        Node::with_value(match *valtype {
            ValType::Empty   => Value::Empty,
            ValType::Boolean => Value::Boolean(false),
            ValType::Integer => Value::Integer(0),
            ValType::Float   => Value::Float(0.0),
            ValType::String  => Value::String(String::new()),
            ValType::Map     => Value::Map(Map::new()),
        })
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn read_value(&self) -> &Value {
        self.value()
    }

    pub fn update_value(&mut self, s: &str) -> Result<(), &'static str> {
        self.value = Value::from_str(s, &self.value.valtype())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_nodespec() {
    }
}
