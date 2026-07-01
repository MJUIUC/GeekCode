use std::collections::HashMap;
use std::collections::VecDeque;

struct FleetTaskScheduler;

impl FleetTaskScheduler {
    fn schedule_tasks(num_tasks: usize, dependencies: Vec<(usize, usize)>) -> Option<Vec<usize>> {
        // tasks are from 0 to num_tasks. represent each as a node
        let nodes = Vec::from_iter(0..num_tasks);

        // (node, n_degree_count)
        // let mut dep_map: HashMap<usize, usize> =
        let mut n_degree_map: HashMap<usize, usize> =
            HashMap::from_iter(nodes.iter().map(|&e| (e, 0 as usize)));

        // (node, children[])
        let mut dep_map: HashMap<usize, Vec<usize>> =
            HashMap::from_iter(nodes.iter().map(|&e| (e, vec![])));

        for dep in &dependencies {
            // count degrees
            n_degree_map
                .entry(dep.0)
                .and_modify(|e| *e += 1)
                .or_default();
            // map children
            dep_map
                .entry(dep.1)
                .and_modify(|e| e.push(dep.0))
                .or_default();
        }

        let mut task_queue: VecDeque<usize> = VecDeque::new();

        for node_n_degree_pair in n_degree_map.iter().filter(|p| *p.1 == 0) {
            task_queue.push_back(*node_n_degree_pair.0);
        }

        // println!("{:?}", task_queue);
        let mut execution_ordering = vec![];
        // level order traverse the graph, only enquing deps with 0 degree
        while !task_queue.is_empty() {
            if let Some(cur_task) = task_queue.pop_front() {
                execution_ordering.push(cur_task);

                if let Some(dependants) = dep_map.get(&cur_task) {
                    for &dep in dependants {
                        let mut n_degrees = n_degree_map[&dep];
                        if n_degrees > 0 {
                            n_degrees -= 1;
                        }

                        if n_degrees == 0 {
                            task_queue.push_back(dep);
                        }

                        // update n_degree for the dependant node
                        n_degree_map.insert(dep, n_degrees);
                    }
                }
            }
        }

        if execution_ordering.len() < num_tasks {
            return None;
        }

        Some(execution_ordering)
    }

    fn schedule_tasks_attempt1(
        num_tasks: usize,
        dependencies: Vec<(usize, usize)>,
    ) -> Option<Vec<usize>> {
        // - tasks are nodes, dependencies are edges
        // - map creates task to dependent child array graph representation
        // - we can level order traverse through the graph in order of dependency
        // - we can mark visited tasks in order to detect cycles since each task should only be executed once
        let mut dep_map = HashMap::<usize, Vec<usize>>::new();
        for dep in dependencies {
            dep_map
                .entry(dep.1)
                .and_modify(|e| e.push(dep.0))
                .or_insert(vec![dep.0]);
        }

        println!("{:?}", dep_map);

        let mut visited: HashMap<usize, usize> = HashMap::new(); // (node, level)
        let mut task_queue: VecDeque<usize> = VecDeque::new();
        let mut execution_order: Vec<usize> = vec![];

        if let Some(&first_dep) = dep_map.keys().min() {
            // println!("{:?}", first_dep);
            // push the first "task" in the the queue
            task_queue.push_back(first_dep);
        } else if num_tasks > 0 {
            // no dependencies if dep_map empty, so return a linear ordering
            for i in 0..num_tasks {
                execution_order.push(i);
                return Some(execution_order);
            }
        }

        let mut current_level = 0;
        // begin level order traversal of dep_map
        while !task_queue.is_empty() {
            for _ in 0..task_queue.len() {
                if let Some(cur_task) = task_queue.pop_front() {
                    // check if cycle, i.e. when a node is executed again at a different level of the tree.

                    if let Some(&visited_level) = visited.get(&cur_task) {
                        if current_level != visited_level {
                            return None;
                        }
                    } else {
                        visited.insert(cur_task, current_level);

                        // add the current task to the execution ordering
                        execution_order.push(cur_task);
                    }

                    if let Some(dependants_arr) = dep_map.get(&cur_task) {
                        for i in 0..dependants_arr.len() {
                            task_queue.push_back(dependants_arr[i]);
                        }
                    }
                }
            }
            current_level += 1;
        }

        Some(execution_order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let result =
            FleetTaskScheduler::schedule_tasks(4 as usize, vec![(1, 0), (2, 0), (3, 1), (3, 2)]);

        println!("{:?}", result);

        assert_eq!(result, Some(vec![0, 1, 2, 3]))
    }
}
