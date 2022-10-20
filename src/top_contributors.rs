use yew::function_component;
use yew::prelude::*;

use crate::bindings::Contributor;

#[derive(Clone, Properties, PartialEq)]
pub struct TopContributorsProps {
    pub top_contributors: Vec<Contributor>,
}

#[function_component(TopContributors)]
pub fn repo_input(TopContributorsProps { top_contributors }: &TopContributorsProps) -> Html {
    let result = top_contributors
        .iter()
        .map(|contributor| {
            html! {
                <p>{format!("{}: {}", contributor.email, contributor.commits)}</p>
            }
        })
        .collect::<Html>();

    html! {
        <div class="row">
            <div>
                {result}
            </div>
        </div>
    }
}
