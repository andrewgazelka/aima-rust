use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use serde::export::Formatter;
use serde::Deserialize;
use std::fmt;
use std::fmt::Debug;

pub struct To<T>(pub T, pub i64);

#[derive(Debug)]
struct Connection<T>(T, T, i64);

pub trait UndirectedGraph<T> {
    fn get_connections(&self, from: &T) -> Vec<To<&T>>;
    fn list_nodes(&self) -> &[T];
}

#[derive(Debug)]
pub struct UndirectedGraphImpl<T> {
    connections: Vec<Connection<T>>,
    nodes: Vec<T>,
}

#[derive(Clone, Eq)]
pub struct StringNode(String, Location);

impl Debug for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq for StringNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl StringNode {
    pub fn from(string: String) -> StringNode {
        StringNode(string, Location(0, 0))
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Location(i64, i64);

impl UndirectedGraphImpl<StringNode> {
    pub fn romania() -> Result<UndirectedGraphImpl<StringNode>, Box<dyn Error>> {
        UndirectedGraphImpl::new("data/romania_locations.toml", "data/romania_distances.toml")
    }

    pub fn new(
        locations_file: &str,
        distances_file: &str,
    ) -> Result<UndirectedGraphImpl<StringNode>, Box<dyn Error>> {
        let locations_str = read_to_string(locations_file)?;
        let locations: HashMap<String, Location> = toml::from_str(&locations_str)?;

        let distances_str = read_to_string(distances_file)?;
        let connections_data: HashMap<String, HashMap<String, i64>> =
            toml::from_str(&distances_str)?;
        let mut connections = vec![];
        for (parent_name, child_table) in connections_data.iter() {
            for (child_name, distance) in child_table.iter() {
                let parent_location = locations.get(parent_name).ok_or("parent location DNE")?;
                let child_location = locations.get(child_name).ok_or("child location DNE")?;
                let connection = Connection(
                    StringNode(parent_name.clone(), parent_location.clone()),
                    StringNode(child_name.clone(), child_location.clone()),
                    *distance,
                );
                connections.push(connection);
            }
        }
        Ok(UndirectedGraphImpl {
            connections,
            nodes: locations
                .into_iter()
                .map(|(name, loc)| StringNode(name, loc))
                .collect(),
        })
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
        vec
    }

    fn list_nodes(&self) -> &[T] {
        self.nodes.as_slice()
    }
}
