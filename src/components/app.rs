use yew::prelude::*;

use crate::components::task_list::Task;
use crate::components::task_list::TaskList;

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
