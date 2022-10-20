use yew::function_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct StatCardProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or("".to_string())]
    pub class: String,
}

#[function_component(StatCard)]
pub fn repo_input(
    StatCardProps {
        title,
        children,
        class,
    }: &StatCardProps,
) -> Html {
    html! {
        <div class={format!("card w-100 {}", class)}>
            <h2>{title}</h2>
             { for children.iter() }
        </div>
    }
}
