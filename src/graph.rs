use std::collections::HashMap;
use std::fs;

use toml::Value;

struct To<T>(T, i64);

#[derive(Debug)]
struct Connection<T>(T, T, i64);

trait UndirectedGraph<T> {
    fn get_connections(&self, from: &T) -> Vec<To<&T>>;
    fn list_nodes(&self) -> &[T];
}

#[derive(Debug)]
pub struct UndirectedGraphImpl<T> {
    connections: Vec<Connection<T>>,
    nodes: Vec<T>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StringNode(String, i64, i64);

struct Location(i64, i64);

impl UndirectedGraphImpl<StringNode> {
    pub fn new(locations_file: &str, distances_file: &str) -> UndirectedGraphImpl<StringNode> {
        let mut locations = HashMap::new();
        {
            let contents: Value = fs::read_to_string(locations_file).unwrap().parse().unwrap();
            let main_table = contents.as_table().unwrap();
            for (key, value) in main_table {
                let arr = value.as_array().unwrap();
                let first = arr.get(0).unwrap().as_integer().unwrap();
                let second = arr.get(1).unwrap().as_integer().unwrap();
                let node = StringNode(key.clone(), first, second);
                locations.insert(key.clone(), node);
            }
        }
        let mut connections = vec![];
        {
            let contents: Value = fs::read_to_string(distances_file).unwrap().parse().unwrap();
            let main_table = contents.as_table().unwrap();
            for (parent_name, children) in main_table {
                let children_table = children.as_table().unwrap();
                let children_keys = children_table.keys();
                for child_name in children_keys {
                    let parent_node = locations.get(parent_name);
                    let child_node = locations.get(child_name);
                    let value = children_table.get(child_name).unwrap().as_integer().unwrap();
                    let con = Connection(parent_node.unwrap().clone(), child_node.unwrap().clone(), value);
                    connections.push(con);
                }
            }
        }
        return UndirectedGraphImpl {
            connections,
            nodes: locations.into_iter().map(|x| x.1).collect()
        }
    }
}

impl<T: PartialEq> UndirectedGraph<T> for UndirectedGraphImpl<T> {
    fn get_connections(&self, from: &T) -> Vec<To<&T>> {
        let mut vec = vec![];
        for Connection(a, b, dist) in self.connections.iter() {
            if a == from {
                vec.push(To(b, *dist));
            } else if b == from {
                vec.push(To(a, *dist));
            }
        }
        return vec;
    }

    fn list_nodes(&self) -> &[T] {
        return self.nodes.as_slice();
    }
}
