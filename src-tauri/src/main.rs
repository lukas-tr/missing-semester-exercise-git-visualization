#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::prelude::DateTime;
use chrono::{Datelike, NaiveDateTime, Utc};
use git2::Repository;
use std::collections::HashMap;
use std::{env, str};

#[derive(serde::Serialize)]
struct TauriError {
    error: String,
}

impl From<git2::Error> for TauriError {
    fn from(item: git2::Error) -> Self {
        TauriError {
            error: item.to_string(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
struct Contributor {
    email: String,
    commits: i64,
}

#[derive(serde::Serialize, Debug, Clone)]
struct Year {
    year: i32,
    commits: i64,
}

#[derive(serde::Serialize)]
struct RepoStats {
    commits: i64,
    contributors: i64,
    top_contributors: Vec<Contributor>,
    contributions_by_year: Vec<Year>,
}

#[tauri::command]
fn get_repo_stats(path: &str) -> Result<RepoStats, TauriError> {
    let mut contributors: HashMap<String, Contributor> = HashMap::new();
    let mut commits_by_year: HashMap<i32, i64> = HashMap::new();
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    revwalk.push_head()?;
    let mut count = 0;
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;
        let seconds = commit.time().seconds();
        let naive = NaiveDateTime::from_timestamp(seconds, 0);
        let datetime = DateTime::<Utc>::from_utc(naive, Utc);

        commits_by_year
            .entry(datetime.year())
            .and_modify(|t| {
                *t = *t + 1;
            })
            .or_insert(1);
        count += 1;
        if let Some(author) = commit.author().email() {
            contributors
                .entry(author.to_owned())
                .and_modify(|t| {
                    t.commits += 1;
                })
                .or_insert(Contributor {
                    email: author.to_owned(),
                    commits: 1,
                });
        };
    }

    let mut contributions_by_year: Vec<Year> = commits_by_year
        .iter()
        .map(|a| Year {
            commits: a.1.to_owned(),
            year: a.0.to_owned(),
        })
        .collect();
    contributions_by_year.sort_by(|a, b| b.year.cmp(&a.year));

    let mut top_contributors: Vec<Contributor> = contributors.values().cloned().collect();
    top_contributors.sort_by(|a, b| b.commits.cmp(&a.commits));
    top_contributors.truncate(contributions_by_year.len());

    Ok(RepoStats {
        commits: count,
        contributors: contributors.keys().len() as i64,
        top_contributors,
        contributions_by_year,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_repo_stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
