use std::{collections::{hash_map::Entry, HashMap}, time::SystemTime};

use crate::campaign_struct::Campaign;

 pub fn pay_campaign(campaign_key:usize, campaigns:   & mut Vec<Campaign>, donation_amount:u32, contributor_name: & String, is_contributor:&mut HashMap<String, bool>  ){
        let current_time = SystemTime::now();
        let mut campaign_struct = campaigns.get(campaign_key);
        match campaign_struct{
        Some(campaign) => {campaign_struct = Some(campaign)},
        None => panic!("Campaign does not exist"),
        };
        let mut  unwrapped_campaign_struct =   campaign_struct.unwrap().clone(); 
        assert!(current_time.duration_since((unwrapped_campaign_struct.start_time).unwrap()).unwrap() < unwrapped_campaign_struct.duration, "time passed" );

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
}