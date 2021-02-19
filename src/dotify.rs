use std::{collections::HashMap, io::Write};

type Table = usize;
type Reference<'a> = &'a (usize, usize);
struct Graph<'a> {
    nodes: Vec<&'a String>,
    edges: Vec<(usize, usize)>,
}

pub fn render_to<W: Write>(fks: &HashMap<String, Vec<String>>, output: &mut W) {
    let nodes = fks.keys().collect::<Vec<&String>>();
    let mut edges = vec![];
    for (index, &node) in nodes.iter().enumerate() {
        let xrefs = fks.get(node).unwrap();
        for xref in xrefs {
            if let Some(xref_index) = nodes.iter().position(|&x| x == xref) {
                edges.push((index, xref_index));
            }
        }
    }
    let graph = Graph { nodes, edges };
    dot::render(&graph, output).unwrap()
}

impl<'a> dot::Labeller<'a, Table, Reference<'a>> for Graph<'a> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("sqlsense").unwrap()
    }

    fn node_id(&'a self, n: &Table) -> dot::Id<'a> {
        dot::Id::new(format!("table{}", n)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Table) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].into())
    }

    fn edge_label<'b>(&'b self, _: &Reference) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(" references ".into())
    }
}

impl<'a> dot::GraphWalk<'a, Table, Reference<'a>> for Graph<'a> {
    fn nodes(&self) -> dot::Nodes<'a, Table> {
        (0..self.nodes.len()).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Reference<'a>> {
        self.edges.iter().collect()
    }

    fn source(&self, e: &Reference) -> Table {
        e.0
    }

    fn target(&self, e: &Reference) -> Table {
        e.1
    }
}
