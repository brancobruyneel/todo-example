use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq, Deserialize)]
pub struct TaskProps {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[function_component]
pub fn Task(props: &TaskProps) -> Html {
    let completed = use_state(|| props.completed);

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
        <li class="flex gap-2 justify-left items-center">
            <input
                {onclick}
                type="checkbox"
                id={props.id.clone()}
                checked={*completed}
            />
            <label
                class={classes!(completed_class)}
                for={props.id.clone()}>{props.title.clone()}
            </label>
        </li>
    }
}
