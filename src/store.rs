use command::Command;
use node::Node;
use nodespec::NodeSpec;
use response::Response;
use value::{Map, ValType, Value};

pub struct Store {
    root: Node,
}

impl Store {
    pub fn new() -> Self {
        let root = Node::with_type(&ValType::Map);

        Store {
            root,
        }
    }

    pub fn execute(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::Create(nodespec, name, valtype) => {
                let parent = self.get_node(nodespec)?;
                match parent.value_mut() {
                    Value::Map(m) => {
                        if m.contains_key(&name) {
                            return Response::Error("node already exists");
                        }
                        m.insert(name, Node::with_type(&valtype));
                    },
                    _             => return Response::Error("parents exist but is not a map"),
                }
                Response::Success
            },
            Command::Read(nodespec) => {
                Response::Value(self.get_node(nodespec)?.read_value())
            },
            Command::Update(nodespec, value) => {
                self.get_node(nodespec)?.update_value(&value)?;
                Response::Success
            }
        }
    }

    pub fn get_node(&mut self, nodespec: NodeSpec) -> Result<&mut Node, &'static str> {
        let mut iter = &mut self.root;
        for childname in nodespec.iter() {
            if let &mut Value::Map(ref mut m) = iter.value_mut() {
                iter = m.get_mut(childname).ok_or("node does not exist")?;
            } else {
                return Err("node does not exist (some parent node does but is not a map)");
            };
        }
        Ok(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut store = Store::new();

        // Create a map `foo`
        let res = store.execute("create . foo map".parse().unwrap());
        if let Response::Success = res {} else {
            panic!("create foo failed: {:?}", res);
        }

        // Create a string `foo.bar`
        let res = store.execute("create foo bar string".parse().unwrap());
        if let Response::Success = res {} else {
            panic!("create foo.bar failed: {:?}", res);
        }

        // Try to create `foo.bar` again (should fail)
        let res = store.execute("create foo bar string".parse().unwrap());
        if !res.is_err() {
            panic!("duplicate create foo.bar didn't fail: {:?}", res);
        }

        // Update `foo.bar` with 'hello_world'
        let res = store.execute("update foo.bar hello_world".parse().unwrap());
        if let Response::Success = res {} else {
            panic!("update foo.bar failed: {:?}", res);
        }

        // Read 'hello world' from `foo.bar`
        let res = store.execute("read foo.bar".parse().unwrap());
        match res {
            Response::Value(&Value::String(ref s)) => assert_eq!(s, "hello_world"),
            _                                      => panic!("expected a string value but got {:?}", res),
        };
    }
}
