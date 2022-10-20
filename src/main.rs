mod app;
mod repo_input;
mod top_contributors;
mod contributions_by_year;
mod bindings;

use app::App;

fn main() {
    yew::start_app::<App>();
}
