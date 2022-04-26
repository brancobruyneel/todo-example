use yew::prelude::*;

use crate::components::task::Task;
use crate::components::task_list::TaskList;

#[function_component]
pub fn App() -> Html {
    let tasks = vec![
        html! { <Task id={1} title={"Take dog out for a walk"} completed={true} /> },
        html! { <Task id={2} title={"Feed the cats"} completed={false} /> },
        html! { <Task id={3} title={"Water the plants"} completed={false} /> },
    ];

    html! {
        <main class="mx-auto w-96">
            <h1 class="font-bold text-4xl text-center my-8">{ "My todo list" }</h1>
            <TaskList>
                {tasks}
            </TaskList>
        </main>
    }
}
