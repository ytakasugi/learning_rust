use std::collections::{VecDeque, HashSet};
use std::borrow::Borrow;
use std::hash::Hash;

pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

pub struct Crawler<'a, G: AdjacentNodes> {
    graph: &'a G,
    visit: VecDeque<<G as AdjacentNodes>::Node>,
    visited: HashSet<<G as AdjacentNodes>::Node>,
}

impl<'a, G> Crawler<'a, G>
where
    G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    pub fn new(graph: &'a G, start: <G as AdjacentNodes>::Node) -> Self {
        let mut visit = VecDeque::new();
        let visited = HashSet::new();

        visit.push_back(start);

        Self {
            graph: graph,
            visit: visit,
            visited: visited,
        }
    }
}

impl<'a, G> Iterator for Crawler<'a, G>
where
    G: AdjacentNodes,
    // Clone:`visit`から取り出した要素を`visited`に挿入してイテレータから返すため
    // `HaseSet`は保持するデータを所有する必要があり、戻り値も所有権が呼び出し側に移るために、取り出した要素のコピーが必要になる
    // `Hash`,`Eq`,`Borrow`:`HashSet`のメソッドを呼び出すため
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    type Item = <G as AdjacentNodes>::Node;

    fn next(&mut self) -> Option<Self::Item> {
        // `visit`から取り出した要素が`visited`に含まれるか調べる
        while let Some(v) = self.visit.pop_front() {
            // 含まれていた場合、`continue`で処理をスキップする
            if self.visited.contains(&v) {
                continue;
            }

            let adj = self.graph.adjacent_nodes(&v);

            for u in adj.into_iter() {
                // `visited`に取り出した要素が含まれていなかった場合、`visit`の後ろに要素を追加する
                if !self.visited.contains(&u) {
                    self.visit.push_back(u);
                }
            }
            self.visited.insert(v.clone());
            return Some(v);
        }
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    struct AdjVec(Vec<Vec<usize>>);

    impl AdjacentNodes for AdjVec {
        type Node = usize;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            self.0.get(*v)
                .cloned()
                .unwrap_or(Vec::new())
        }
    }

    #[test]
    fn bfs() {
        let graph = AdjVec(vec![
            vec![1, 2],
            vec![0, 3],
            vec![3],
            vec![2, 0]
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();
        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }
}