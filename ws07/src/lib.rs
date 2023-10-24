#[macro_export]
macro_rules! add_task {
    ($scheduler:ident, $prerequisites:expr, $task:block) => {
        $scheduler.add_task(Task {
            prerequisites: $prerequisites,
            task: Box::new(|| $task),
        });
    };
}

// #[macro_export]
// macro_rules! add_task {
//     ($scheduler:ident, [$($prerequisite:ident),*], $task:block) => {
//         {
//             let prerequisites = HashSet::from([$(Prerequisites::$prerequisite),*]);
//             let task = Box::new(|| $task);
//             $scheduler.add_task(Task { prerequisites, task });
//         }
//     };
// }
