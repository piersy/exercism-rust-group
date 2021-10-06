pub mod graph {
    use std::collections::HashMap;
    pub trait Attributable<'a> {
        fn with_attrs(&mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            // self.attrs = HashMap::new();
            let m = self.get_attrs();
            for (k, v) in attrs {
                // self.attrs.insert(k, v);
                m.insert(k, v);
            }
            *self
        }

        fn get_attrs(&mut self) -> &mut HashMap<&'a str, &'a str>;
    }
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node<'a>>) -> Self {
            self.nodes = nodes.to_vec();
            // self.nodes = *nodes.clone();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge<'a>>) -> Self {
            self.edges = edges.to_vec();
            self
        }
    }

    impl<'a> Attributable<'a> for Graph<'a> {
        fn get_attrs(&mut self) -> &mut HashMap<&'a str, &'a str> {
            &mut self.attrs
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone)]
            pub struct Edge<'a> {
                start: &'a str,
                end: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(start: &'a str, end: &'a str) -> Self {
                    Edge {
                        start,
                        end,
                        attrs: HashMap::new(),
                    }
                }
            }

            impl<'a> super::super::Attributable<'a> for Edge<'a> {
                fn get_attrs(&mut self) -> &mut HashMap<&'a str, &'a str> {
                    &mut self.attrs
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone)]
            pub struct Node<'a> {
                pub v: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }
            impl<'a> Node<'a> {
                pub fn new(v: &'a str) -> Self {
                    Node {
                        v,
                        attrs: HashMap::new(),
                    }
                }
            }
            impl<'a> super::super::Attributable<'a> for Node<'a> {
                fn get_attrs(&mut self) -> &mut HashMap<&'a str, &'a str> {
                    &mut self.attrs
                }
            }
        }
    }
}
