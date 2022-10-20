use wasm_bindgen_futures::spawn_local;
use yew::function_component;
use yew::prelude::*;

use crate::bindings::show_directory_selector;

#[derive(Clone, Properties, PartialEq)]
pub struct RepoInputProps {
    pub on_choose_dir: Callback<String>,
}

#[function_component(RepoInput)]
pub fn repo_input(RepoInputProps { on_choose_dir }: &RepoInputProps) -> Html {
    let directory_input_ref = use_ref(|| NodeRef::default());

    let analyze = {
        let directory_input_ref = directory_input_ref.clone();
        let callback = on_choose_dir.clone();
        Callback::from(move |_| {
            let name = directory_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            callback.emit(name);
        })
    };

    let browse = {
        let directory_input_ref = directory_input_ref.clone();
        let callback = on_choose_dir.clone();
        Callback::from(move |_| {
            let directory_input_ref = directory_input_ref.clone();
            let callback = callback.clone();
            spawn_local(async move {
                if let Some(dir) = show_directory_selector().await {
                    directory_input_ref
                        .cast::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .set_value(&dir);
                    callback.emit(dir.to_owned());
                }
            });
        })
    };

    html! {
        <div class="row">
            <input id="directory-input" ref={&*directory_input_ref} placeholder="Enter a name..." />
            <button type="button" onclick={analyze}>{"Analyze"}</button>
            <button type="button" onclick={browse}>{"Browse"}</button>
        </div>
    }
}
