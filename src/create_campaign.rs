use std::time::{Duration, SystemTime};
use std::{collections::HashMap, io};
use crate::campaign_struct::Campaign;
pub fn create_campaigns<'a>( campaigns: &'a mut  Vec<Campaign>, campaign_title: &'a String,
    funding_goal: &'a u64,
    duration_secs: &'a u64,
    campaign_owner: &'a String,) ->  &'a mut Vec<Campaign>{
    let start_time = SystemTime::now();
    let duration = Duration::from_secs(*duration_secs);
    let campaign_id: u8 = campaigns.len() as u8 + 1;
        let campaign = Campaign{
            campaign_id,
            campaign_title: campaign_title.to_string(),
            funding_goal: *funding_goal,
            duration,
            start_time:Some(start_time),
            campaign_owner: campaign_owner.to_string(),
            campaign_balance:0,
            is_active:true,
            contributors:  vec![],
            contributors_balance: HashMap::new(),
          
        };
        campaigns.push(campaign);
        campaigns
    }
