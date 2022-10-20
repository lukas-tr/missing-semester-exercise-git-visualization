use yew::function_component;
use yew::prelude::*;

use crate::bindings::Contributor;
use crate::progress_bar::ProgressBar;
use crate::stat_card::StatCard;

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
                <>
                    <p>{format!("{}: {}", contributor.email, contributor.commits)}</p>
                    <ProgressBar percentage={contributor.commits as f64 / top_contributors.first().and_then(|f| Some(f.commits as f64)).or(Some(1.0)).unwrap()} />
                </>
            }
        })
        .collect::<Html>();

    html! {
        <StatCard title="Top Contributors" class="w-50 mr-8">
            {result}
        </StatCard>
    }
}
