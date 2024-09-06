use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::collections::HashMap;


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Campaign {

   pub campaign_id: u8,
   pub campaign_title: String,
   pub campaign_owner: String,
   pub campaign_balance:u64,
   pub funding_goal: u64,
   pub duration:  Duration,
   pub start_time:Option<SystemTime>,
   pub is_active:bool,
   pub contributors: Vec<String>,
   pub contributors_balance: HashMap<String, u64>
}
