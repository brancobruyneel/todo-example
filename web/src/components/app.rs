use gloo::net::http::Request;
use yew::prelude::*;

use crate::components::task::{Task, TaskProps};
use crate::components::task_list::TaskList;


#[function_component]
pub fn App() -> Html {
    let tasks = use_state(std::vec::Vec::new);

    {
        let tasks = tasks.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_tasks: Vec<TaskProps> = Request::get("/api/task")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                tasks.set(fetched_tasks.iter().take(10).map(|props| {
                    html! { <Task id={props.id.clone()} title={props.title.clone()} completed={props.completed} /> }
                }).collect()
                );
            });
            || ()
        }, ());
    }

    html! {
        <main class="mx-auto w-96">
            <h1 class="font-bold text-4xl text-center my-8">{ "My todo list" }</h1>
            <TaskList>
                { (*tasks).clone() }
            </TaskList>
        </main>
    }
}
