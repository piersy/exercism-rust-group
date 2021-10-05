fn pair_to_string(x: &(&str, &str)) -> (String, String) {
    (x.0.to_string(), x.1.to_string())
}

pub mod graph {

    use crate::pair_to_string;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use crate::pair_to_string;
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Node {
                        id: id.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.into_iter().map(pair_to_string).collect();
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }

        pub mod edge {
            use crate::pair_to_string;
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub u: String,
                pub v: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(u: &str, v: &str) -> Self {
                    Edge {
                        u: u.to_string(),
                        v: v.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.into_iter().map(pair_to_string).collect();
                    self
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            };
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.into_iter().map(pair_to_string).collect();
            self
        }

        pub fn get_node(self, node_id: &str) -> Option<graph_items::node::Node> {
            self.nodes.into_iter().find(|x| x.id == node_id)
        }
    }
}
