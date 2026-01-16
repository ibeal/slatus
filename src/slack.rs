use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const PROFILE_SET_URL: &str = "https://slack.com/api/users.profile.set";
const PROFILE_GET_URL: &str = "https://slack.com/api/users.profile.get";

#[derive(Serialize)]
struct ProfileUpdate {
    profile: ProfileStatus,
}

#[derive(Serialize)]
struct ProfileStatus {
    status_text: String,
    status_emoji: String,
    status_expiration: u64,
}

#[derive(Deserialize)]
struct SlackResponse {
    ok: bool,
    error: Option<String>,
    profile: Option<ProfileData>,
}

#[derive(Deserialize)]
struct ProfileData {
    status_text: Option<String>,
    status_emoji: Option<String>,
}

pub fn set_status(token: &str, text: &str, emoji: &str, expiration: u64) -> Result<()> {
    let payload = ProfileUpdate {
        profile: ProfileStatus {
            status_text: text.to_string(),
            status_emoji: emoji.to_string(),
            status_expiration: expiration,
        },
    };

    let response = minreq::post(PROFILE_SET_URL)
        .with_header("Authorization", format!("Bearer {}", token))
        .with_json(&payload)?
        .send()
        .context("Failed to connect to Slack API")?;

    let slack_response: SlackResponse = response
        .json()
        .context("Failed to parse Slack response")?;

    if !slack_response.ok {
        let error = slack_response.error.unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!("Slack API error: {}", error);
    }

    Ok(())
}

pub fn get_status(token: &str) -> Result<(String, String)> {
    let response = minreq::get(PROFILE_GET_URL)
        .with_header("Authorization", format!("Bearer {}", token))
        .send()
        .context("Failed to connect to Slack API")?;

    let slack_response: SlackResponse = response
        .json()
        .context("Failed to parse Slack response")?;

    if !slack_response.ok {
        let error = slack_response.error.unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!("Slack API error: {}", error);
    }

    let profile = slack_response.profile.context("No profile in response")?;

    Ok((
        profile.status_text.unwrap_or_default(),
        profile.status_emoji.unwrap_or_default(),
    ))
}
