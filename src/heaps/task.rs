#[derive(Eq, Clone, Copy, Debug)]
pub struct Task<T>(i32, T);

impl<T> Task<T> {
    pub fn new(priority: i32, task: T) -> Self {
        Self(priority, task)
    }
}

impl<T> PartialOrd for Task<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Eq> Ord for Task<T> {
    fn cmp(&self, other: &Task<T>) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialEq for Task<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[test]
fn store_test() {
    let s = Task::new(3, "吃饭");
    let s2 = Task::new(4, "写代码");
    println!("{:?}", s < s2)
}
