use pico_args::Arguments;
use serde::{Deserialize, Serialize};
use std::io::Write;
use xtask::glue::fs2;
use xtask::*;

/// A token is needed to run this script. To create a token, go to https://github.com/settings/tokens
/// and give it read access to the repository.
///
/// Only users that have read rights can run this script
fn main() -> Result<()> {
    let root = project_root().join("website/src/_includes");
    let mut args = Arguments::from_env();
    let token: String = args.value_from_str("--token").unwrap();
    let mut contributors = Vec::new();
    get_contributors(
        "https://api.github.com/repos/rome/tools/contributors",
        &token,
        &mut contributors,
    );

    let mut content = Vec::new();

    writeln!(content, "<!-- {} -->", PREAMBLE)?;
    writeln!(content)?;
    writeln!(content, "### Code contributors")?;
    writeln!(content)?;
    writeln!(content, "<ul class=\"team-list credits\">")?;
    for contributor in contributors {
        let escaped_login = html_escape::encode_text(&contributor.login);
        let escaped_avatar = html_escape::encode_text(&contributor.avatar_url);
        writeln!(
            content,
            "<li><a href=\"https://github.com/rome/tools/commits?author={}\">",
            contributor.login
        )?;
        writeln!(
            content,
            "<img src=\"{}\" alt=\"{}\" />",
            escaped_avatar, contributor.login
        )?;
        writeln!(content, "<span>{}</span>", escaped_login)?;
        writeln!(content, "</a></li>")?;
    }

    writeln!(content, "</ul>")?;
    fs2::write(root.join("contributors.md"), content)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Contributor {
    avatar_url: String,
    login: String,
}

fn get_contributors(url: &str, token: &str, contributors: &mut Vec<Contributor>) {
    let request = ureq::get(url)
        .set("User-Agent", "@rome")
        .set("Authorization", &format!("token {token}"));

    match request.call() {
        Ok(response) => {
            let next_url = if let Some(link) = response.header("link") {
                if link.contains("rel=\"next\"") {
                    let start_index = link
                        .find("rel=\"prev\", ")
                        .map(|index| index + "rel=\"prev\", ".len())
                        .unwrap_or(0);
                    // SAFETY: checked before
                    let end_index = link.find("; rel=\"next\"").unwrap();
                    let url = &link[start_index..end_index];
                    let url = url.replace('<', "").replace('>', "");
                    Some(url)
                } else {
                    None
                }
            } else {
                None
            };
            let result: Result<Vec<Contributor>, std::io::Error> = response.into_json();
            if let Ok(new_contributors) = result {
                contributors.extend(new_contributors);
            }

            if let Some(next_url) = next_url {
                get_contributors(&next_url, token, contributors);
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
