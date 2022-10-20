use yew::function_component;
use yew::prelude::*;

use crate::bindings::Year;

#[derive(Clone, Properties, PartialEq)]
pub struct ContributionsByYearProps {
    pub contributions: Vec<Year>,
}

#[function_component(ContributionsByYear)]
pub fn repo_input(ContributionsByYearProps { contributions }: &ContributionsByYearProps) -> Html {
    let result = contributions
        .iter()
        .map(|contributor| {
            html! {
                <p>{format!("{}: {}", contributor.year, contributor.commits)}</p>
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
