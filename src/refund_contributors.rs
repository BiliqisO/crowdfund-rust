use crate::{campaign_struct::Campaign, store_campaign_data::{load_campaigns, update_campaign}};
pub fn refund_contributors(campaign_key:usize, campaign_owner: & String){

        let mut campaigns = load_campaigns();
        let mut campaign_struct: Option<&Campaign> = campaigns.get(campaign_key);
        match campaign_struct{
        Some(campaign) => {campaign_struct = Some(campaign)},
        None => panic!("Campaign does not exist"),
        };
    let mut  unwrapped_campaign_struct =   campaign_struct.unwrap().clone(); 
    if &unwrapped_campaign_struct.campaign_owner != campaign_owner {
        println!("you are not authorised to perform this action")
    }
    if !unwrapped_campaign_struct.is_active {
        println!( "campaign not active");
    }
    if &unwrapped_campaign_struct.campaign_balance > &unwrapped_campaign_struct.funding_goal{
    unwrapped_campaign_struct.contributors_balance.insert(campaign_owner.to_string(), unwrapped_campaign_struct.campaign_balance);
    println!("funding goal reached, funds paid to campaign owner")
    };
    if unwrapped_campaign_struct.campaign_balance < unwrapped_campaign_struct.funding_goal{
        //possibly if there is a 3rd party wallet or I decide to create wallet for the users that signs
        //up on the platform  the user funds should be directed to the wallets or their bank acct
    println!("funding goal not reached, contributors funds refunded");
    };
    //if a wallet is implemented, this action should go into the "goal reached" conditional
    for  contributor in &unwrapped_campaign_struct.contributors {
        let contributor_balance = unwrapped_campaign_struct.contributors_balance[&contributor.clone()];      
        let entry = unwrapped_campaign_struct.contributors_balance.entry(contributor.to_string()).or_insert(0);
        println!("entrty {}", entry);
        *entry -= contributor_balance;
    };
    unwrapped_campaign_struct.campaign_balance  = 0;
    unwrapped_campaign_struct.is_active = false;
    campaigns[campaign_key] = unwrapped_campaign_struct; 
    
    let _ = update_campaign(campaign_key as u8, serde_json::to_value(&campaigns).expect("failed to convert"));

    
}