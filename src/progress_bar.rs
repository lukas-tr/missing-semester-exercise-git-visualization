use yew::function_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ProgressBarProps {
    pub percentage: f64,
}

#[function_component(ProgressBar)]
pub fn repo_input(ProgressBarProps { percentage }: &ProgressBarProps) -> Html {

    html! {
        <div class="progress">
            <div class="bar" style={format!("width: {}%;", percentage * 100.0)}/>
        </div>
    }
}
