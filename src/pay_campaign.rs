use std::{collections::{hash_map::Entry, HashMap}, time::SystemTime};
use crate::store_campaign_data::{load_campaigns, update_campaign};

pub fn pay_campaign(campaign_key:usize,  donation_amount:u64, contributor_name: & String, is_contributor:&mut HashMap<String, bool>  ){
        let current_time = SystemTime::now();
        let mut campaigns = load_campaigns();
        let mut campaign_struct = campaigns.get(campaign_key);
        match campaign_struct{
        Some(campaign) => {campaign_struct = Some(campaign)},
        None => panic!("Campaign does not exist"),
        };
        let mut  unwrapped_campaign_struct =   campaign_struct.unwrap().clone(); 
        if current_time.duration_since((unwrapped_campaign_struct.start_time).unwrap()).unwrap()  < unwrapped_campaign_struct.duration{
            println!("Time passed");
            unwrapped_campaign_struct.is_active = false;

        }; 
        match is_contributor.get(contributor_name) {
            Some(&true) => {println!("name already in struct")},
            Some(&false) =>{},
            None => {unwrapped_campaign_struct.contributors.push(contributor_name.to_owned());
            is_contributor.insert(contributor_name.to_owned(), true);}, 
        };
        let entry = unwrapped_campaign_struct.contributors_balance.entry(contributor_name.to_owned());
        match entry {
        Entry::Occupied(mut o) => {
            *o.get_mut() += donation_amount;
        }
        Entry::Vacant(v) => {
            v.insert(donation_amount);  
        }
    }
    unwrapped_campaign_struct.campaign_balance +=donation_amount;
    campaigns[campaign_key] = unwrapped_campaign_struct; 
    let _ = update_campaign(campaign_key as u8, serde_json::to_value(&campaigns).expect("failed to convert"));


}