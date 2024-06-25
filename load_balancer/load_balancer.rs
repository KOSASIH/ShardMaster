// load_balancer.rs
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::RwLock;

pub struct LoadBalancer {
    algorithm: String,
    nodes: Arc<Mutex<HashMap<String, Node>>>,
}

impl LoadBalancer {
    pub async fn new(config: HashMap<String, String>) -> Self {
        LoadBalancer {
            algorithm: config["load_balancing_algorithm"].clone(),
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&mut self, node: Node) {
        let mut nodes = self.nodes.lock().await;
        nodes.insert(node.id.clone(), node);
    }

    pub async fn remove_node(&mut self, node_id: String) {
        let mut nodes = self.nodes.lock().await;
        nodes.remove(&node_id);
    }

    pub async fn get_node(&self) -> Option<&Node> {
        let nodes = self.nodes.lock().await;
        nodes.iter().next()
    }
}

pub struct Node {
    id: String,
    address: String,
}
