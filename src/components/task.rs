use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Deserialize)]
pub struct TaskProps {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[function_component]
pub fn Task(props: &TaskProps) -> Html {
    let completed = use_state(|| props.completed);

    let task_id = format!("task-{}", props.id);

    let onclick = {
        let completed = completed.clone();

        Callback::from(move |_| {
            if *completed {
                completed.set(false);
            } else {
                completed.set(true);
            }
        })
    };

    let completed_class = {
        if *completed {
            "line-through"
        } else {
            ""
        }
    };

    html! {
        <div class="flex gap-2 justify-left items-center">
            <input
                {onclick}
                type="checkbox"
                id={task_id.clone()}
                checked={*completed}
            />
            <label
                class={classes!(completed_class)}
                for={task_id.clone()}>{props.title.clone()}
            </label>
        </div>
    }
}
