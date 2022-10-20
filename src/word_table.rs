use yew::function_component;
use yew::prelude::*;

use crate::bindings::Word;
use crate::stat_card::StatCard;

#[derive(Clone, Properties, PartialEq)]
pub struct WordTableProps {
    pub words: Vec<Word>,
}

#[function_component(WordTable)]
pub fn repo_input(WordTableProps { words }: &WordTableProps) -> Html {
    let result = words
        .iter()
        .map(|contributor| {
            html! {
                <>
                    <div class="inline-block w-25">{format!("{} ({})", contributor.word, contributor.occurences)}</div>
                </>
            }
        })
        .collect::<Html>();

    html! {
        <StatCard title="Top Words">
        {result}
        </StatCard>
    }
}
