use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn message(message: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Contributor {
    pub email: String,
    pub commits: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub struct Year {
    pub year: i32,
    pub commits: i64,
}

#[derive(Serialize, Deserialize)]
pub struct OpenDirectoryArgs {
    pub directory: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetRepoStatsArgs<'a> {
    pub path: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct MessageArgs<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub title: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Word {
    pub word: String,
    pub occurences: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetRepoStatsResult {
    pub commits: i64,
    pub contributors: i64,
    pub top_contributors: Vec<Contributor>,
    pub contributions_by_year: Vec<Year>,
    pub top_words: Vec<Word>,
}

#[derive(Serialize, Deserialize)]
pub struct TauriError {
    pub error: String,
}

impl From<serde_wasm_bindgen::Error> for TauriError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        TauriError {
            error: err.to_string(),
        }
    }
}

pub async fn get_repo_stats<'a>(
    args: GetRepoStatsArgs<'a>,
) -> Result<GetRepoStatsResult, TauriError> {
    let new_msg = invoke("get_repo_stats", to_value(&args).unwrap()).await;
    let val = match new_msg {
        Ok(val) => val,
        Err(e) => {
            return match from_value::<TauriError>(e) {
                Ok(val) => Err(val),
                Err(e) => Err(e.into()),
            }
        }
    };
    match from_value::<GetRepoStatsResult>(val) {
        Ok(val) => Ok(val),
        Err(e) => Err(e.into()),
    }
}

pub async fn show_error_message(error: TauriError) {
    message(
        &error.error,
        to_value(&MessageArgs {
            kind: "error",
            title: "Error!",
        })
        .unwrap(),
    )
    .await;
}

pub async fn show_directory_selector() -> Option<String> {
    let new_msg = open(to_value(&OpenDirectoryArgs {directory: true}).unwrap()).await;
    new_msg.as_string()
}
