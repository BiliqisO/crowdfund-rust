mod campaign_struct;
mod create_campaign;
mod pay_campaign;
mod refund_contributors;



use std::collections::HashMap;
// use std::hash::{DefaultHasher, Hash, Hasher};

use create_campaign::create_campaigns;
use pay_campaign::pay_campaign;
use refund_contributors::refund_contributors;
use campaign_struct::Campaign;
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


