pub mod graph {
    use std::collections::HashMap;
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone,Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Self {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(&mut self, attrs: &[(&str,&str)]) -> Self {
                    for (name, value) in attrs {
                        self.attrs.insert(String::from(*name), String::from(*value));
                    }
                    self.clone()
                }
                pub fn attr(&self, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        Some(value) => Some(value.as_str()),
                        None => None,
                    }
                }
            }
            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    let mut result = self.name == other.name && self.attrs.len() == other.attrs.len();
                    if result {
                        for (name,value) in self.attrs.iter() {
                            if other.attrs[name] != *value {
                                result = false;
                                break;
                            }
                        }
                    }
                    result
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone,Debug)]
            pub struct Edge {
                pub node1: String,
                pub node2: String,
                pub attrs: HashMap<String,String>,
            }
            impl Edge {
                pub fn new(node1: &'static str, node2: &'static str) -> Self {
                    Self {
                        node1: String::from(node1),
                        node2: String::from(node2),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(&mut self, attrs: &[(&str,&str)]) -> Self {
                    for (name, value) in attrs {
                        self.attrs.insert(String::from(*name), String::from(*value));
                    }
                    self.clone()
                }
                pub fn attr(&self, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        Some(value) => Some(value.as_str()),
                        None => None,
                    }
                }
            }
            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    let mut result = self.node1 == other.node1 && self.node2 == other.node2
                        && self.attrs.len() == other.attrs.len();
                    if result {
                        for (name,value) in self.attrs.iter() {
                            if other.attrs[name] != *value {
                                result = false;
                                break;
                            }
                        }
                    }
                    result
                }
            }
        }
    }
    #[derive(Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String,String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_edges(&mut self, edges: &[graph_items::edge::Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self.clone()
        }
        pub fn with_nodes(&mut self, nodes: &[graph_items::node::Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self.clone()
        }
        pub fn node(&self, name: &str) -> Option<&graph_items::node::Node> {
            for node in self.nodes.iter() {
                if node.name == name {
                    return Some(node);
                }
            }
            None
        }
        pub fn with_attrs(&mut self, attrs: &[(&str,&str)]) -> Self {
            for (name, value) in attrs {
                self.attrs.insert(String::from(*name), String::from(*value));
            }
            self.clone()
        }
        pub fn attr(&self, name: &str) -> Option<&str> {
            match self.attrs.get(name) {
                Some(value) => Some(value.as_str()),
                None => None,
            }
        }
    }
    impl Default for Graph {
        fn default() -> Self {
            Self::new()
        }
    }
}