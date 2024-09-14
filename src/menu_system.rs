#[derive(Debug, Clone)]
pub struct MenuNode {
    pub name: String,
    pub children: Option<Box<[MenuNode]>>,
}

impl MenuNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: None,
        }
    }

    pub fn add_children(&mut self, children: Box<[MenuNode]>) {
        self.children = Some(children);
    }
}

#[derive(Debug)]
pub struct MenuTree {
    pub path: Vec<MenuNode>,
}

impl MenuTree {
    pub fn new(root: MenuNode) -> Self {
        let mut new_path: Vec<MenuNode> = Vec::new();
        new_path.push(root);
        Self { path: new_path }
    }
    pub fn current(&self) -> MenuNode {
        self.path.last().unwrap().clone()
    }
    pub fn next(&mut self, target: usize) {
        self.path
            .push(self.current().children.unwrap()[target].clone())
    }
    pub fn back(&mut self) -> MenuNode {
        self.path.pop().unwrap()
    }
}
