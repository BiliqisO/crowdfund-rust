use std::time::SystemTime;

use crate::campaign_struct::Campaign;
pub fn refund_contributors(campaign_key:usize, campaigns:   & mut Vec<Campaign>, contributor_name: & String){
        let current_time = SystemTime::now();
        let mut campaign_struct: Option<&Campaign> = campaigns.get(campaign_key);
        match campaign_struct{
        Some(campaign) => {campaign_struct = Some(campaign)},
        None => panic!("Campaign does not exist"),
        };
    let mut  unwrapped_campaign_struct =   campaign_struct.unwrap().clone(); 
    assert!(unwrapped_campaign_struct.is_active, "campaign not active");
    assert!(current_time.duration_since((unwrapped_campaign_struct.start_time).unwrap()).unwrap() < unwrapped_campaign_struct.duration, "time not reached, you can't withdraw" );
    assert!(unwrapped_campaign_struct.campaign_balance > unwrapped_campaign_struct.funding_goal, "funding goal not reached");
    assert!(unwrapped_campaign_struct.contributors_balance.get(&contributor_name.to_owned()).is_some(), "you do not have money inn this campaignn");
  
    for  contributor in &unwrapped_campaign_struct.contributors {
        let contributor_balance = unwrapped_campaign_struct.contributors_balance[&contributor.clone()];      
        let entry = unwrapped_campaign_struct.contributors_balance.entry(contributor.to_string()).or_insert(0);
        println!("entrty {}", entry);
        *entry -= contributor_balance;
        // unwrapped_campaign_struct.contributors_balance.insert(contributor.to_string(), *entry);
        println!("entrty {}", entry); 
        
    };
    unwrapped_campaign_struct.campaign_balance  = 0;
    unwrapped_campaign_struct.is_active = false;
   campaigns[campaign_key] = unwrapped_campaign_struct; 

    
}