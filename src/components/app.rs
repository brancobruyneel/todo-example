use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Properties, PartialEq)]
struct TaskListProps {
    tasks: Vec<Task>,
}

#[function_component]
fn TaskList(TaskListProps { tasks }: &TaskListProps) -> Html {
    tasks
        .iter()
        .map(|task| {
            let task_id = format!("task-{}", task.id);
            html! {
                <div class="flex gap-2 justify-center items-center">
                    <input type="checkbox" id={task_id.clone()} checked={task.completed} />
                    <label for={task_id.clone()}>{task.title.clone()}</label>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn App() -> Html {
    let tasks = vec![
        Task {
            id: 1,
            title: "Take dog out for a walk".to_string(),
            completed: true,
        },
        Task {
            id: 2,
            title: "Feed the cats".to_string(),
            completed: false,
        },
        Task {
            id: 3,
            title: "Water plants".to_string(),
            completed: false,
        },
    ];

    html! {
        <div class="container-sm mx-auto text-center">
            <h1 class="font-bold text-4xl my-4">{ "My todo list" }</h1>
            <div class="flex flex-col">
                <TaskList tasks={tasks}  />
            </div>
        </div>
    }
}
