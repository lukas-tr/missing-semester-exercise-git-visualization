mod app;
mod repo_input;
mod top_contributors;
mod contributions_by_year;
mod bindings;
mod progress_bar;
mod stat_card;
mod word_table;

use app::App;

fn main() {
    yew::start_app::<App>();
}
