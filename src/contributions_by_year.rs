use yew::function_component;
use yew::prelude::*;

use crate::bindings::Year;
use crate::progress_bar::ProgressBar;
use crate::stat_card::StatCard;

#[derive(Clone, Properties, PartialEq)]
pub struct ContributionsByYearProps {
    pub contributions: Vec<Year>,
    pub total: i64,
}

#[function_component(ContributionsByYear)]
pub fn repo_input(ContributionsByYearProps { contributions, total }: &ContributionsByYearProps) -> Html {
    let max = contributions.iter().map(|c| c.commits).max().unwrap_or_default();
    let result = contributions
        .iter()
        .map(|contributor| {
            html! {
                <>
                    <p>{format!("{}: {} ({}% of all contributions)", contributor.year, contributor.commits, ((contributor.commits as f64 / *total as f64) * 100.0)as i64)}</p>
                    <ProgressBar percentage={contributor.commits as f64 / max as f64} />
                </>
            }
        })
        .collect::<Html>();

    html! {
        <StatCard title="Contributions per Year" class="w-50">
            {result}
        </StatCard>
    }
}
