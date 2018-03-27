#[derive(Debug)]
struct Node {
    id: u32,
    children: Vec<Node>
}

impl Node {
    fn new(id: u32, children: Vec<Node>) -> Node {
        assert_ne!(id, 0);
        Node { id, children }
    }

    fn get_children_node_by_id(&self, id: u32) -> Option<&Node> {
        for child in self.children.iter() {
            if child.id == id {
                return Some(child);
            }
            if self.children.len() > 0 {
                match child.get_children_node_by_id(id) {
                    Some(node) => return Some(node),
                    None => continue
                };
            }
        }
        None
    }

    fn get_path(&self, id: u32, path: &mut Vec<u32>) -> Option<&Node> {
        for child in self.children.iter() {
            if child.id == id {
                path.push(child.id);
                return Some(child);
            }
            if self.children.len() > 0 {
                match child.get_path(id, path){
                    Some(node) => {
                        path.push(child.id);
                        return Some(node);
                    },
                    None => continue
                };
            }
        }
        None
    }
}

fn main() {
    //      4    7
    //   2
    // 1    5
    //
    //   3
    //      6

    let graph = Node::new(
        1,
        vec![
            Node::new(
                2,
            vec![
                Node::new(
                    4,
                    vec![
                        Node::new(7, vec![])
                    ]
                ),
                Node::new(
                    5,
                    vec![]
                )
            ]),
            Node::new(
                3,
                vec![
                    Node::new(
                        6,
                        vec![]
                    ),
                ])
        ]);

    match graph.get_children_node_by_id(6) {
        Some(node) => {
            println!("Found a node: {:?}!", node);
        },
        None => println!("Couldn't find a node!")
    }

    let mut path: Vec<u32> = vec![];
    match graph.get_path(6, &mut path) {
        Some(node) => {
            path.reverse();
            println!("Found a path to node with id {}, its path is {:?}", node.id, path);
        },
        None => println!("There is no path!")
    };
}
