use std::time::{Duration, SystemTime};
use std::{collections::HashMap, io};

use crate::campaign_struct::Campaign;
pub fn create_campaigns( campaigns: &mut  Vec<Campaign>) ->  &mut Vec<Campaign>{
    let mut campaign_title = String::new();
    let mut funding_goal = String::new();
    let mut duration = String::new(); 
    let mut campaign_owner = String::new(); 
    println!(" kindly pass in your name::");
    io::stdin()
            .read_line(&mut campaign_owner)
            .expect("Failed to read line");
    println!(" kindly pass in campaign title::");
    io::stdin()
            .read_line(&mut campaign_title)
            .expect("Failed to read line");
        println!(" kindly pass in funding goal::");
    io::stdin()
            .read_line(&mut funding_goal)
            .expect("Failed to read line");
            println!(" kindly pass in duration::");
    io::stdin()
            .read_line(&mut duration)
            .expect("Failed to read line");

        let campaign_owner = campaign_owner.trim().parse().expect("Please type in campaign title!");
        let campaign_title = campaign_title.trim().parse().expect("Please type in campaign title!");
        let funding_goal = funding_goal.trim().parse().expect("Please type a number!");
        let duration: u64=  duration.trim().parse().expect("Please type a number!");
        let start_time = SystemTime::now();
        let duration=  Duration::from_secs(duration);
        let mut campaign_id:u8 = 0;
      
        campaign_id +=1;
        let campaign = Campaign{
            campaign_id,
            campaign_title,
            funding_goal,
            duration,
            start_time:Some(start_time),
            campaign_owner,
            campaign_balance:0,
            is_active:true,
            contributors:  vec![],
            contributors_balance: HashMap::new(),
          
        };
        campaigns.push(campaign.clone());
        campaigns
    }
