use reqwest;
use clap::Parser;
use serde::{Deserialize, Serialize};
use csv::Writer;


#[derive(Parser)]
struct Cli {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct ConversationsListResponse {
    ok: bool,
    channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize)]
struct Channel {
    id: String,
    name: String,
    is_channel: bool,
    is_group: bool,
    is_im: bool,
    is_mpim: bool,
    is_private: bool,
    is_archived: bool,
}

const CONVERSATIONS_CSV_PATH: &str = ".bin/conversations.csv";

fn main() {
    let args = Cli::parse();

    set_up(args);
}

fn get_request_slack_api(method: &str, token: &str) -> reqwest::blocking::Response {
    let url = format!("https://slack.com/api/{}", method);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send();

    return resp.unwrap();
}

fn get_channels_from_slack(token: &str) -> Vec<Vec<String>> {
    let json_str = get_request_slack_api("conversations.list", token);
    let res: ConversationsListResponse = serde_json::from_str(&json_str.text().unwrap()).unwrap();
    let mut records: Vec<Vec<String>> = Vec::new();

    for channel in res.channels {
        let mut record: Vec<String> = Vec::new();
        record.push(channel.id);
        record.push(channel.name);
        if channel.is_private {
            record.push("private".to_string());
        } else {
            record.push("public".to_string());
        }

        records.push(record);
    }

    records
}

fn write_csv(path: &str, records: Vec<Vec<String>>) {
    let mut writer = Writer::from_path(path).unwrap();
    for record in records {
        writer.write_record(&record).unwrap();
    }
}

fn set_up(args: Cli) {
    let records = get_channels_from_slack(&args.token);

    write_csv(CONVERSATIONS_CSV_PATH, records)
}