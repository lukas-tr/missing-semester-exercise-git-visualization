use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::bindings::{get_repo_stats, GetRepoStatsArgs, show_error_message, GetRepoStatsResult};
use crate::contributions_by_year::ContributionsByYear;
use crate::repo_input::RepoInput;
use crate::stat_card::StatCard;
use crate::top_contributors::TopContributors;
use crate::word_table::WordTable;

#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::new());
    let result = use_state(|| GetRepoStatsResult {
        commits: 0,
        contributors: 0,
        contributions_by_year: vec![],
        top_contributors: vec![],
        top_words: vec![],
    });

    {
        let name = name.clone();
        let name2 = name.clone();
        let result = result.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let res = get_repo_stats(GetRepoStatsArgs { path: &*name }).await;

                    match res {
                        Ok(val) => result.set(val.to_owned()),
                        Err(e) => {show_error_message(e).await; return},
                    }
                });

                || {}
            },
            name2,
        );
    }

    let on_video_select = {
        let name = name.clone();
        Callback::from(move |dir| name.set(dir))
    };

    let result = (*result).clone();

    html! {
        <main class="container">
            <RepoInput on_choose_dir={on_video_select.clone()} />
            <div class="row mb-8">
                <StatCard title="Commits" class="mr-8">{result.commits}</StatCard>
                <StatCard title="Contributors">{result.contributors}</StatCard>
            </div>
            <div class="row mb-8">
                <TopContributors top_contributors={result.top_contributors} />
                <ContributionsByYear total={result.commits} contributions={result.contributions_by_year} />
            </div>
            <div class="row">
                <WordTable words={result.top_words} />
            </div>
        </main>
    }
}
