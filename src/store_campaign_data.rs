use std::fs;
use std::time::{Duration, UNIX_EPOCH};
use serde_json::json;

use crate::campaign_struct::Campaign;
// const CAMPAIGNS_FILE: &str = "campaigns.json";

pub fn save_campaigns(campaigns: &Vec<Campaign>) -> Result<(), serde_json::Error> {
    let json = campaigns.iter().map(|campaign| {
        json!({
            "campaign_id": campaign.campaign_id,
            "campaign_title": campaign.campaign_title,
            "funding_goal": campaign.funding_goal,
            "duration": campaign.duration.as_secs(),
            "start_time": campaign.start_time.map(|st| st.duration_since(UNIX_EPOCH).unwrap().as_secs()),
            "campaign_owner": campaign.campaign_owner,
            "campaign_balance": campaign.campaign_balance,
            "is_active": campaign.is_active,
            "contributors": campaign.contributors,
            "contributors_balance": campaign.contributors_balance,
        })
    }).collect::<Vec<_>>();

    let json_string = serde_json::to_string_pretty(&json)?;
    fs::write("campaigns.json", json_string).expect("Unable to write file");
    Ok(())
}

pub fn load_campaigns() -> Vec<Campaign> {
    let data = fs::read_to_string("campaigns.json").unwrap_or_else(|_| "[]".to_string());
    let json: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);

    json.into_iter().map(|item| {
        Campaign {
            campaign_id: item["campaign_id"].as_u64().unwrap_or(0) as u8,
            campaign_title: item["campaign_title"].as_str().unwrap_or("").to_string(),
            funding_goal: item["funding_goal"].as_u64().unwrap_or(0),
            duration: Duration::from_secs(item["duration"].as_u64().unwrap_or(0)),
            start_time: item["start_time"].as_u64().map(|secs| UNIX_EPOCH + Duration::from_secs(secs)),
            campaign_owner: item["campaign_owner"].as_str().unwrap_or("").to_string(),
            campaign_balance: item["campaign_balance"].as_u64().unwrap_or(0),
            is_active: item["is_active"].as_bool().unwrap_or(false),
            contributors: item["contributors"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap_or("").to_string()).collect(),
            contributors_balance: item["contributors_balance"].as_object().unwrap_or(&serde_json::Map::new()).iter().filter_map(|(k, v)| v.as_u64().map(|u| (k.clone(), u))).collect(),
        }
    }).collect()
}
pub fn update_campaign(campaign_id: u8, updated_data: serde_json::Value) -> Result<(), String> {
    let mut campaigns = load_campaigns();
    if let Some(index) = campaigns.iter().position(|c| c.campaign_id == campaign_id) {
        let campaign = &mut campaigns[index];
    
        if let Some(title) = updated_data["campaign_title"].as_str() {
            campaign.campaign_title = title.to_string();
        }
        if let Some(goal) = updated_data["funding_goal"].as_u64() {
            campaign.funding_goal = goal;
        }
        if let Some(duration) = updated_data["duration"].as_u64() {
            campaign.duration = Duration::from_secs(duration);
        }
        if let Some(start_time) = updated_data["start_time"].as_u64() {
            campaign.start_time = Some(UNIX_EPOCH + Duration::from_secs(start_time));
        }
        if let Some(owner) = updated_data["campaign_owner"].as_str() {
            campaign.campaign_owner = owner.to_string();
        }
        if let Some(balance) = updated_data["campaign_balance"].as_u64() {
            campaign.campaign_balance = balance;
        }
        if let Some(is_active) = updated_data["is_active"].as_bool() {
            campaign.is_active = is_active;
           
        }
        if let Some(contributors) = updated_data["contributors"].as_array() {
            campaign.contributors = contributors.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect();
        }
        if let Some(contributors_balance) = updated_data["contributors_balance"].as_object() {
            campaign.contributors_balance = contributors_balance
        .iter()
        .filter_map(|(k, v)| v.as_u64().map(|u| (k.clone(), u)))  // Keep the value as u64
        .collect();
        }
       fs::write("campaigns.json", serde_json::to_string(&updated_data).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Campaign with ID {} not found", campaign_id))
    }
}


