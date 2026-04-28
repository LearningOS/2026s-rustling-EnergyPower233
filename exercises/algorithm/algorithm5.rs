/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }
    //洗了喵 不会图喵 不会广搜喵 不会vector喵
    //失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了失败了
    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 1. 准备一个数组，用来记录节点是否已经被访问过，防止死循环
        // 初始状态下，所有节点都没有被访问过 (false)
        let mut has_been_visited = vec![false; self.adj.len()];

        // 2. 准备一个队列，用来安排节点被处理的顺序
        let mut queue = VecDeque::new();

        // 3. 准备一个动态数组，用来保存最终的访问顺序
        let mut visit_order = vec![];

        // --- 开始 BFS 初始化 ---

        // 标记起点为“已访问”，并把它加入队列排队
        has_been_visited[start] = true;
        queue.push_back(start);

        // --- 开始 BFS 循环 ---

        // 只要队列里还有节点在排队，我们就一直处理
        while let Some(current_node) = queue.pop_front() {
            // 从队首请出一个节点，把它加入到我们的最终结果列表中
            visit_order.push(current_node);

            // 看看这个节点都有哪些直接相邻的邻居
            for &neighbor in &self.adj[current_node] {
                // 如果这个邻居之前还没被访问过
                if !has_been_visited[neighbor] {
                    // 赶紧给它打上“已访问”的标记
                    has_been_visited[neighbor] = true;
                    // 然后让它去队列尾部排队，等着之后处理它的邻居
                    queue.push_back(neighbor);
                }
            }
        }

        // 循环结束，所有能到达的节点都已经处理完了！
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
