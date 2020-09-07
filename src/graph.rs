use std::collections::HashMap;
use std::error::Error;
use std::{fs, io};

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use toml::Value;

pub struct To<T>(T, i64);

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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StringNode(String, Location);

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Location(i64, i64);

impl UndirectedGraphImpl<StringNode> {
    pub fn romania() -> Result<UndirectedGraphImpl<StringNode>, Box<dyn Error>> {
        return UndirectedGraphImpl::new(
            "data/romania_locations.toml",
            "data/romania_distances.toml",
        );
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
        return Ok(UndirectedGraphImpl {
            connections,
            nodes: locations
                .into_iter()
                .map(|(name, loc)| StringNode(name, loc))
                .collect(),
        });
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
