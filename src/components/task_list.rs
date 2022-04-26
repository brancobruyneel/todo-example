use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
}

#[function_component]
pub fn TaskList(TaskListProps { tasks }: &TaskListProps) -> Html {
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
