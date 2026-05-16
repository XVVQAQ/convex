pub struct Router<T: Copy + PartialEq> {
    history: Vec<T>,
}

impl<T: Copy + PartialEq + std::fmt::Debug> Router<T> {
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }

    pub fn current(&self) -> T {
        self.history.last().copied().unwrap()
    }

    pub fn depth(&self) -> usize {
        self.history.len()
    }

    pub fn navigate_to(&mut self, route: T) {
        if self.current() == route {
            return;
        }
        self.history.push(route);
    //    println!("🔀 Navigate → (depth: {})", self.depth());
    }

    pub fn navigate_to_root(&mut self, route: T) {
        self.history.clear();
        self.history.push(route);
    //    println!("🏠 Root → {:?} (depth: 1)", route);
    }

    pub fn go_back(&mut self) {
        if self.history.len() > 1 {
            self.history.pop();
    //    println!("🔙 Back (depth: {})", self.depth());
        }
    }
}
