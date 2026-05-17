use std::fmt::Debug;

pub struct Router<T: Copy + PartialEq> {
    history: Vec<T>,
}

impl<T: Copy + PartialEq + Debug> Router<T> {
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }

    pub fn current(&self) -> T {
        self.history.last().copied().unwrap()
    }

    /// 统一导航：is_root=true 时清空栈，false 时入栈
    pub fn navigate_to(&mut self, route: T, is_root: bool) {
        if !self.should_navigate(route) {
            return;
        }
        if is_root {
            self.history.clear();
        }
        self.history.push(route);
        log::info!("navigate (depth: {})", self.history.len());
    }

    pub fn go_back(&mut self) {
        if self.history.len() > 1 {
            self.history.pop();
            log::info!("go back (depth: {})", self.history.len());
        }
    }

    fn should_navigate(&self, route: T) -> bool {
        self.current() != route
    }
}
