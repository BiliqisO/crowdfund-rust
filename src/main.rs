use std::collections::hash_map::Entry;
use std::time::{SystemTime, Duration};
use std::{collections::HashMap, io};
// use std::hash::{DefaultHasher, Hash, Hasher};

// // use std::{collections::HashMap, io};
// use chrono::{prelude::*, TimeDelta};

#[derive(Default, Debug, Clone)]

struct Campaign {
    campaign_id: u8,
    campaign_title: String,
    funding_goal: u32,
    duration:  Duration,
    start_time:Option<SystemTime>,
    campaign_owner: String,
    campaign_balance:u32,
    is_active:bool,
    contributors: Vec<String>,
    contributors_balance: HashMap<String, u32>
}
fn main() {
    let mut campaigns: Vec<Campaign> =Vec::new();
    //this one is just for me to be able to track  the storage of contributors
    let mut is_contributor:HashMap<String, bool> = HashMap::new();

    let  mut campaign: &mut Vec<Campaign> = create_campaigns(& mut campaigns);
    println!("  campaign key: {:?}",  (&campaign));

    pay_campaign(0, &mut campaign,30 ,&String::from("Alake"), &mut is_contributor);
    pay_campaign(0, &mut campaign,30 ,&String::from("Ashraf"), &mut is_contributor);
    pay_campaign(0, &mut campaign,80 ,&String::from("Alake"), &mut is_contributor);
    println!("  campaign key111: {:?}", (&campaign));
    refund_contributors(0, &mut campaign ,&String::from("Alake"));
    println!("  campaign key14441: {:?}",  (&campaign));
    // pay_campaign(1, &mut campaign,80 ,&String::from("Alake"), &mut is_contributor);

}


fn create_campaigns( campaigns: &mut  Vec<Campaign>) ->  &mut Vec<Campaign>{
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

    fn pay_campaign(campaign_key:usize, campaigns:   & mut Vec<Campaign>, donation_amount:u32, contributor_name: & String, is_contributor:&mut HashMap<String, bool>  ){
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
fn refund_contributors(campaign_key:usize, campaigns:   & mut Vec<Campaign>, contributor_name: & String){
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