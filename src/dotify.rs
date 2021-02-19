use std::{collections::HashMap, io::Write};

type Nd = usize;
type Ed<'a> = &'a (usize, usize);
struct Graph<'a> {
    nodes: Vec<&'a String>,
    edges: Vec<(usize, usize)>,
}

pub fn render_to<'a, W: Write>(fks: &'a HashMap<String, Vec<&'a str>>, output: &mut W) {
    let nodes = fks.keys().collect::<Vec<&String>>();
    let mut edges = vec![];
    for (index, &node) in nodes.iter().enumerate() {
        let xrefs = fks.get(node).unwrap();
        for &xref in xrefs {
            if let Some(xref_index) = nodes.iter().position(|&x| x == xref) {
                edges.push((index, xref_index));
            }
        }
    }
    let graph = Graph { nodes, edges };
    dot::render(&graph, output).unwrap()
}

impl<'a> dot::Labeller<'a, Nd, Ed<'a>> for Graph<'a> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example2").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].into())
    }
    fn edge_label<'b>(&'b self, _: &Ed) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(" references ".into())
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph<'a> {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.nodes.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges.iter().collect()
    }
    fn source(&self, e: &Ed) -> Nd {
        e.0
    }
    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}
