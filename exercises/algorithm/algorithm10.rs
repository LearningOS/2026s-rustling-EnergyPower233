/*
    graph
    This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 把传入的元组拆开，拿出 节点1、节点2 和 权重(距离)
        let (node1, node2, weight) = edge;

        // 1. 先确保这两个人都在我们的通讯录里。
        // 调用我们刚刚写的 add_node，如果他们不在，会自动给他们建好空白页
        self.add_node(node1);
        self.add_node(node2);

        // 2. 把 node2 添加到 node1 的好友列表里
        self.adjacency_table_mutable()
            .get_mut(node1) // 找到 node1 的那一页
            .unwrap() // 确认找到了（因为我们上面刚 add_node 保证了它一定在）
            .push((node2.to_string(), weight)); // 把 node2 记进去

        // 3. 因为是“无向图”，所以必须双向奔赴！把 node1 也添加到 node2 的列表里
        self.adjacency_table_mutable()
            .get_mut(node2)
            .unwrap()
            .push((node1.to_string(), weight));
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        // 如果通讯录里已经有这个人了，直接返回 false，什么也不做
        if self.contains(node) {
            return false;
        }
        // 如果没有这个人，就给他新建一页，并配上一个空的“好友列表”
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
