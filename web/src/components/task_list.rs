use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub children: Children,
}

#[function_component]
pub fn TaskList(TaskListProps { children }: &TaskListProps) -> Html {
    html! {
        <ul>
            { for children.iter() }
        </ul>
    }
}
