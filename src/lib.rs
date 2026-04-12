#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum EdgeKind { Trust, Communication, Cooperation, Observation }
pub struct Agent { id: u32, trust: f64, influence: f64, group: u8 }
pub struct Edge { from: u32, to: u32, weight: f64, kind: EdgeKind }
pub struct SocialGraph { agents: Vec<Agent>, edges: Vec<Edge> }
impl SocialGraph {
    pub fn new() -> Self { Self { agents: Vec::new(), edges: Vec::new() } }
    pub fn add_agent(&mut self, id: u32, group: u8) -> bool {
        if self.agents.iter().any(|a| a.id == id) { return false; }
        self.agents.push(Agent { id, trust: 0.5, influence: 0.0, group }); true
    }
    pub fn remove_agent(&mut self, id: u32) { self.agents.retain(|a| a.id != id); self.edges.retain(|e| e.from != id && e.to != id); }
    pub fn connect(&mut self, from: u32, to: u32, kind: EdgeKind, weight: f64) -> bool {
        if !self.agents.iter().any(|a| a.id == from) || !self.agents.iter().any(|a| a.id == to) { return false; }
        self.edges.retain(|e| !(e.from == from && e.to == to));
        self.edges.push(Edge { from, to, weight: weight.clamp(0.0, 1.0), kind }); true
    }
    pub fn disconnect(&mut self, from: u32, to: u32) { self.edges.retain(|e| !(e.from == from && e.to == to)); }
    pub fn trust(&self, from: u32, to: u32) -> f64 { self.edges.iter().filter(|e| e.from == from && e.to == to && e.kind == EdgeKind::Trust).map(|e| e.weight).sum() }
    pub fn set_trust(&mut self, from: u32, to: u32, t: f64) {
        if let Some(e) = self.edges.iter_mut().find(|e| e.from == from && e.to == to && e.kind == EdgeKind::Trust) { e.weight = t.clamp(0.0, 1.0); }
        else { self.connect(from, to, EdgeKind::Trust, t); }
    }
    pub fn neighbors(&self, id: u32) -> Vec<&Agent> {
        let nids: Vec<u32> = self.edges.iter().filter(|e| e.from == id).map(|e| e.to).collect();
        nids.iter().filter_map(|&nid| self.agents.iter().find(|a| a.id == nid)).collect()
    }
    pub fn influence(&self, id: u32) -> f64 { self.edges.iter().filter(|e| e.to == id).map(|e| e.weight).sum() }
    pub fn group_of(&self, id: u32) -> Option<u8> { self.agents.iter().find(|a| a.id == id).map(|a| a.group) }
    pub fn group_members(&self, group: u8) -> Vec<&Agent> { self.agents.iter().filter(|a| a.group == group).collect() }
    pub fn path(&self, from: u32, to: u32) -> Vec<u32> {
        let mut visited = std::collections::HashSet::new(); let mut queue = std::collections::VecDeque::new();
        queue.push_back((from, vec![from])); visited.insert(from);
        while let Some((cur, path)) = queue.pop_front() { for e in &self.edges { if e.from == cur && !visited.contains(&e.to) {
            let mut np = path.clone(); np.push(e.to);
            if e.to == to { return np; } visited.insert(e.to); queue.push_back((e.to, np));
        }}} vec![]
    }
    pub fn degree(&self, id: u32) -> usize { self.edges.iter().filter(|e| e.from == id || e.to == id).count() }
    pub fn decay(&mut self, rate: f64) { for e in &mut self.edges { e.weight = (e.weight * (1.0 - rate)).max(0.0); } }
    pub fn agent_count(&self) -> usize { self.agents.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let g = SocialGraph::new(); assert_eq!(g.agent_count(), 0); }
    #[test] fn test_add_agent() { let mut g = SocialGraph::new(); assert!(g.add_agent(1, 0)); assert!(!g.add_agent(1, 0)); }
    #[test] fn test_connect() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); assert!(g.connect(1, 2, EdgeKind::Trust, 0.8)); }
    #[test] fn test_trust() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.8); assert!((g.trust(1, 2) - 0.8).abs() < 1e-6); }
    #[test] fn test_set_trust() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.set_trust(1, 2, 0.9); assert!((g.trust(1, 2) - 0.9).abs() < 1e-6); }
    #[test] fn test_neighbors() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.add_agent(3, 0); g.connect(1, 2, EdgeKind::Trust, 0.5); g.connect(1, 3, EdgeKind::Trust, 0.5); assert_eq!(g.neighbors(1).len(), 2); }
    #[test] fn test_influence() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.8); assert!((g.influence(2) - 0.8).abs() < 1e-6); }
    #[test] fn test_path() { let mut g = SocialGraph::new(); for i in 1..=3 { g.add_agent(i, 0); } g.connect(1, 2, EdgeKind::Communication, 0.5); g.connect(2, 3, EdgeKind::Communication, 0.5); assert_eq!(g.path(1, 3), vec![1, 2, 3]); }
    #[test] fn test_disconnect() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.5); g.disconnect(1, 2); assert!((g.trust(1, 2)).abs() < 1e-6); }
    #[test] fn test_group() { let mut g = SocialGraph::new(); g.add_agent(1, 1); g.add_agent(2, 2); assert_eq!(g.group_members(1).len(), 1); }
    #[test] fn test_degree() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.5); assert_eq!(g.degree(1), 1); }
    #[test] fn test_decay() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.8); g.decay(0.5); assert!(g.trust(1, 2) < 0.8); }
    #[test] fn test_remove_agent() { let mut g = SocialGraph::new(); g.add_agent(1, 0); g.add_agent(2, 0); g.connect(1, 2, EdgeKind::Trust, 0.5); g.remove_agent(1); assert_eq!(g.agent_count(), 1); assert_eq!(g.edges.len(), 0); }
}