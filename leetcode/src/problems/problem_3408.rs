use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy)]
struct Task {
    task_id: i32,
    user_id: i32,
    priority: i32,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
            && self.user_id == other.user_id
            && self.priority == other.priority
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.priority == other.priority {
            other.task_id.partial_cmp(&self.task_id)
        } else {
            other.priority.partial_cmp(&self.priority)
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct TaskManager {
    id_map: HashMap<i32, Task>,
    tasks: BTreeSet<Task>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut id_map = HashMap::<i32, Task>::new();
        let mut tasks_tree = BTreeSet::<Task>::new();
        for task in tasks {
            let (user_id, task_id, priority) = (task[0], task[1], task[2]);
            let task = Task {
                user_id,
                task_id,
                priority,
            };
            id_map.insert(task_id, task.clone());
            tasks_tree.insert(task);
        }
        Self {
            id_map,
            tasks: tasks_tree,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        let task = Task {
            user_id,
            task_id,
            priority,
        };
        self.id_map.insert(task_id, task.clone());
        self.tasks.insert(task);
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(task) = self.id_map.get_mut(&task_id) {
            self.tasks.remove(task);
            task.priority = new_priority;
            self.tasks.insert(*task);
        }
    }

    fn rmv(&mut self, task_id: i32) {
        if let Some(task) = self.id_map.remove(&task_id) {
            self.tasks.remove(&task);
        }
    }

    fn exec_top(&mut self) -> i32 {
        if let Some(task) = self.tasks.pop_first() {
            self.id_map.remove(&task.task_id);
            task.user_id
        } else {
            -1
        }
    }
}

pub fn run() {
    let mut task_manager =
        TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
    task_manager.add(4, 104, 5);
    task_manager.edit(102, 8);
    tracing::info!("{}", task_manager.exec_top());
    task_manager.rmv(101);
    task_manager.add(5, 105, 15);
    tracing::info!("{}", task_manager.exec_top());
}

// [
//     [
//         [
//             [1,101,10],
//             [2,102,20],
//             [3,103,15]
//         ]
//     ],
//     [4,104,5],
//     [102,8],
//     [],
//     [101],
//     [5,105,15],
//     []
// ]
