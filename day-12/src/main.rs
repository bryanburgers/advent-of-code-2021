use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let graph = Graph::parse(input)?;
    let paths = graph.paths()?;
    let result = paths.count();
    println!("{}", result);

    let paths = graph.paths_part_b()?;
    let result = paths.count();
    println!("{}", result);
    Ok(())
}

pub struct Graph<'s> {
    vertices: Vec<&'s str>,
    edges: HashMap<VertexId, Vec<VertexId>>,
}

impl<'s> Graph<'s> {
    pub fn empty() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn parse(input: &'s str) -> Result<Self, &'static str> {
        let mut graph = Graph::empty();
        for line in input.trim().lines() {
            let (a, b) = line.trim().split_once("-").ok_or("No separator found")?;
            graph.add_edge(a, b);
        }
        Ok(graph)
    }

    fn assert_vertex<'g>(&'g mut self, name: &'s str) -> VertexId {
        if let Some(vertex) = self.vertex_by_name(name) {
            vertex.index
        } else {
            let vertex_id = { VertexId(self.vertices.len()) };
            self.vertices.push(name);
            self.edges.insert(vertex_id, Vec::new());
            vertex_id
        }
    }

    pub fn vertex_by_name<'g>(&'g self, name: &'s str) -> Option<Vertex<'g, 's>> {
        let idx = self
            .vertices
            .iter()
            .enumerate()
            .find(|(_idx, haystack)| **haystack == name)
            .map(|(idx, _)| idx)?;
        let vertex_id = VertexId(idx);
        let vertex = Vertex {
            index: vertex_id,
            graph: self,
        };
        Some(vertex)
    }

    fn vertex_by_id<'g>(&'g self, id: VertexId) -> Vertex<'g, 's> {
        Vertex {
            index: id,
            graph: self,
        }
    }

    pub fn add_edge<'g>(&'g mut self, a: &'s str, b: &'s str) {
        let vertex_a = self.assert_vertex(a);
        let vertex_b = self.assert_vertex(b);
        let vec_a = self.edges.entry(vertex_a).or_default();
        vec_a.push(vertex_b);
        let vec_b = self.edges.entry(vertex_b).or_default();
        vec_b.push(vertex_a);
    }

    pub fn paths<'g>(&'g self) -> Result<PathsIterator<'g, 's>, &'static str> {
        PathsIterator::new(self)
    }

    pub fn paths_part_b<'g>(&'g self) -> Result<PathsIteratorPartB<'g, 's>, &'static str> {
        PathsIteratorPartB::new(self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct VertexId(usize);

#[derive(Copy, Clone)]
pub struct Vertex<'g, 's> {
    index: VertexId,
    graph: &'g Graph<'s>,
}

impl<'g, 's> Vertex<'g, 's> {
    pub fn name(&self) -> &'s str {
        self.graph.vertices[self.index.0]
    }

    pub fn is_start(&self) -> bool {
        self.name() == "start"
    }

    pub fn is_end(&self) -> bool {
        self.name() == "end"
    }

    pub fn is_big(&self) -> bool {
        self.name()
            .chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
    }

    pub fn is_small(&self) -> bool {
        !self.is_big()
    }

    pub fn edges(&self) -> EdgesIterator<'g, 's> {
        EdgesIterator::new(self)
    }
}

impl<'g, 's> PartialEq for Vertex<'g, 's> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<'g, 's> Eq for Vertex<'g, 's> {}

impl<'g, 's> Debug for Vertex<'g, 's> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<'g, 's> Hash for Vertex<'g, 's> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

pub struct EdgesIterator<'g, 's> {
    iter: std::slice::Iter<'g, VertexId>,
    graph: &'g Graph<'s>,
}

impl<'g, 's> EdgesIterator<'g, 's> {
    fn new(vertex: &Vertex<'g, 's>) -> Self {
        let edges = vertex.graph.edges.get(&vertex.index).unwrap();
        let iter = edges.iter();
        Self {
            iter,
            graph: vertex.graph,
        }
    }
}

impl<'g, 's> Iterator for EdgesIterator<'g, 's> {
    type Item = Vertex<'g, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| self.graph.vertex_by_id(*id))
    }
}

pub struct PathsIterator<'g, 's> {
    stack_vertices: Vec<Vertex<'g, 's>>,
    stack_iterators: VecDeque<EdgesIterator<'g, 's>>,
}

impl<'g, 's> PathsIterator<'g, 's> {
    pub fn new(graph: &'g Graph<'s>) -> Result<Self, &'static str> {
        let start = graph
            .vertex_by_name("start")
            .ok_or("start vertex not found")?;
        let edges = start.edges();
        let stack_vertices = vec![start];
        let mut stack_iterators = VecDeque::new();
        stack_iterators.push_front(edges);
        Ok(Self {
            stack_iterators,
            stack_vertices,
        })
    }

    pub fn next_path(&mut self) -> Option<Vec<Vertex<'g, 's>>> {
        loop {
            let iterator = self.stack_iterators.back_mut()?;
            if let Some(next_vertex) = iterator.next() {
                if next_vertex.is_end() {
                    // Found an end. Return the entire path!
                    let mut ret = self.stack_vertices.clone();
                    ret.push(next_vertex);
                    return Some(ret);
                } else if next_vertex.is_small() && self.stack_vertices.contains(&next_vertex) {
                    // Small vertex already seen. Ignore.
                } else {
                    // Found the next cave. Add it to the stack and "recurse"
                    let edges = next_vertex.edges();
                    self.stack_vertices.push(next_vertex);
                    self.stack_iterators.push_back(edges);
                }
            } else {
                // We've looked at all of the current cave's edges. Pop it off and continue with the
                // previous cave.
                self.stack_vertices.pop();
                self.stack_iterators.pop_back();
            }
        }
    }
}

impl<'g, 's> Iterator for PathsIterator<'g, 's> {
    type Item = Vec<Vertex<'g, 's>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_path()
    }
}

pub struct PathsIteratorPartB<'g, 's> {
    stack_vertices: Vec<Vertex<'g, 's>>,
    stack_iterators: VecDeque<EdgesIterator<'g, 's>>,
}

impl<'g, 's> PathsIteratorPartB<'g, 's> {
    pub fn new(graph: &'g Graph<'s>) -> Result<Self, &'static str> {
        let start = graph
            .vertex_by_name("start")
            .ok_or("start vertex not found")?;
        let edges = start.edges();
        let stack_vertices = vec![start];
        let mut stack_iterators = VecDeque::new();
        stack_iterators.push_front(edges);
        Ok(Self {
            stack_iterators,
            stack_vertices,
        })
    }

    fn would_be_valid_path(&self, vertex: &Vertex<'_, '_>) -> bool {
        if vertex.is_start() {
            // Can't revisit start twice.
            return false;
        }
        if vertex.is_big() {
            // We can always go to a big cave.
            return true;
        }

        let mut seen_twice = false;
        let mut seen_vertexes = HashSet::new();

        for vertex in self.stack_vertices.iter().chain(std::iter::once(vertex)) {
            if vertex.is_big() {
                continue;
            }
            let already_seen = !seen_vertexes.insert(vertex);
            if already_seen {
                if seen_twice {
                    // Another one has already been seen twice!
                    return false;
                } else {
                    seen_twice = true;
                }
            }
        }

        true
    }

    pub fn next_path(&mut self) -> Option<Vec<Vertex<'g, 's>>> {
        loop {
            let iterator = self.stack_iterators.back_mut()?;
            if let Some(next_vertex) = iterator.next() {
                if next_vertex.is_end() {
                    // Found an end. Return the entire path!
                    let mut ret = self.stack_vertices.clone();
                    ret.push(next_vertex);
                    return Some(ret);
                } else if !self.would_be_valid_path(&next_vertex) {
                    // Would not be a valid path. Ignore
                } else {
                    // Found the next cave. Add it to the stack and "recurse"
                    let edges = next_vertex.edges();
                    self.stack_vertices.push(next_vertex);
                    self.stack_iterators.push_back(edges);
                }
            } else {
                // We've looked at all of the current cave's edges. Pop it off and continue with the
                // previous cave.
                self.stack_vertices.pop();
                self.stack_iterators.pop_back();
            }
        }
    }
}

impl<'g, 's> Iterator for PathsIteratorPartB<'g, 's> {
    type Item = Vec<Vertex<'g, 's>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graph() {
        let mut graph = Graph::empty();
        graph.add_edge("start", "A");
        graph.add_edge("start", "b");
        graph.add_edge("A", "c");
        graph.add_edge("A", "b");
        graph.add_edge("b", "d");
        graph.add_edge("A", "end");
        graph.add_edge("b", "end");

        let start = graph
            .vertex_by_name("start")
            .expect("vertex should be findable");
        assert_eq!(start.name(), "start");
        assert!(start.is_start());
        assert!(start.is_small());
        assert!(!start.is_big());

        let a = graph
            .vertex_by_name("A")
            .expect("vertex should be findable");
        assert_eq!(a.name(), "A");
        assert!(!a.is_small());
        assert!(a.is_big());

        let b = graph
            .vertex_by_name("b")
            .expect("vertex should be findable");
        assert_eq!(b.name(), "b");
        assert!(b.is_small());
        assert!(!b.is_big());

        let end = graph
            .vertex_by_name("end")
            .expect("vertex should be findable");
        assert_eq!(end.name(), "end");
        assert!(end.is_end());
        assert!(end.is_small());
        assert!(!end.is_big());
    }

    #[test]
    fn test_parse() {
        let graph = Graph::parse(
            r#"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            "#,
        )
        .expect("Expected valid parse");

        let start = graph
            .vertex_by_name("start")
            .expect("vertex should be findable");
        assert_eq!(start.name(), "start");
        assert!(start.is_start());
        assert!(start.is_small());
        assert!(!start.is_big());

        let a = graph
            .vertex_by_name("A")
            .expect("vertex should be findable");
        assert_eq!(a.name(), "A");
        assert!(!a.is_small());
        assert!(a.is_big());

        let b = graph
            .vertex_by_name("b")
            .expect("vertex should be findable");
        assert_eq!(b.name(), "b");
        assert!(b.is_small());
        assert!(!b.is_big());

        let end = graph
            .vertex_by_name("end")
            .expect("vertex should be findable");
        assert_eq!(end.name(), "end");
        assert!(end.is_end());
        assert!(end.is_small());
        assert!(!end.is_big());

        let start_edges = start.edges().collect::<Vec<_>>();
        assert_eq!(start_edges.len(), 2);
        assert!(start_edges
            .iter()
            .any(|other_vertex| other_vertex.name() == "A"));
        assert!(start_edges
            .iter()
            .any(|other_vertex| other_vertex.name() == "b"));
    }

    #[test]
    fn test_paths_on_example_1() {
        let graph = Graph::parse(
            r#"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths().expect("Expected paths");
        let count = paths.take(100 /* to catch infinite recursion */).count();
        assert_eq!(count, 10);
    }

    #[test]
    fn test_paths_on_example_2() {
        let graph = Graph::parse(
            r#"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths().expect("Expected paths");
        let count = paths.take(100 /* to catch infinite recursion */).count();
        assert_eq!(count, 19);
    }

    #[test]
    fn test_paths_on_example_3() {
        let graph = Graph::parse(
            r#"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths().expect("Expected paths");
        let count = paths.take(1000 /* to catch infinite recursion */).count();
        assert_eq!(count, 226);
    }

    #[test]
    fn test_paths_b_on_example_1() {
        let graph = Graph::parse(
            r#"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths_part_b().expect("Expected paths");
        let count = paths.take(100 /* to catch infinite recursion */).count();
        assert_eq!(count, 36);
    }

    #[test]
    fn test_paths_b_on_example_2() {
        let graph = Graph::parse(
            r#"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths_part_b().expect("Expected paths");
        let count = paths.take(1000 /* to catch infinite recursion */).count();
        assert_eq!(count, 103);
    }

    #[test]
    fn test_paths_b_on_example_3() {
        let graph = Graph::parse(
            r#"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
            "#,
        )
        .expect("Expected valid parse");

        let paths = graph.paths_part_b().expect("Expected paths");
        let count = paths.take(10000 /* to catch infinite recursion */).count();
        assert_eq!(count, 3509);
    }
}
