
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
#[derive(Default, Debug, Clone)]


pub struct Campaign {
   pub campaign_id: u8,
   pub campaign_title: String,
   pub funding_goal: u32,
   pub(crate) duration:  Duration,
   pub start_time:Option<SystemTime>,
   pub campaign_owner: String,
   pub campaign_balance:u32,
   pub is_active:bool,
   pub contributors: Vec<String>,
   pub contributors_balance: HashMap<String, u32>
}