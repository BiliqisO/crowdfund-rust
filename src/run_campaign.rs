
use crate::{campaign_struct::Campaign, create_campaign::create_campaigns, pay_campaign::pay_campaign, refund_contributors::refund_contributors, store_campaign_data::{load_campaigns, save_campaigns}};

use std::collections::HashMap;

use clap::{builder::Str, Parser, Subcommand};

pub fn run_campaign(username:&String) {
    let cli = Cli::parse();

    let mut campaigns: Vec<Campaign> =load_campaigns();
    let mut is_contributor:HashMap<String, bool> = HashMap::new();


    match &cli.command{
        Commands::Create { campaign_title, funding_goal, duration, campaign_owner }  =>{
        create_campaigns(& mut campaigns, campaign_title,funding_goal,duration, campaign_owner);
        save_campaigns(&campaigns).expect("Failed to save campaigns");
        println!("Campaign '{}' created successfully.", campaign_title);
        }
        Commands::Contribute { campaign_key, amount } => {
            if let   Some(  campaign) = campaigns.get(*campaign_key) {
                if campaign.is_active {
                    pay_campaign(*campaign_key,  *amount, &username,  &mut is_contributor);
                    println!(
                        "Contributor '{}' added {} to campaign '{}'. Total balance: {}",
                        &username, amount, campaign_key, campaign.campaign_balance
                    );
                } else {
                    println!("Campaign '{}' is closed and cannot accept contributions.", campaign.campaign_title);
                }
            } else {
                println!("Campaign '{}' does not exist.", campaign_key);
            }
        
        }
       Commands::Close { campaign_key, campaign_owner } => {
            if let Some(campaign) = campaigns.get(*campaign_key) {
                 if campaign.is_active {
                refund_contributors(*campaign_key,campaign_owner);
                println!("Campaign '{}' is now closed.", campaign_key);
            }else{
                  println!("Campaign '{}' is not active.", campaign_key);
            }
            } else {
                println!("Campaign '{}' does not exist.", campaign_key);
            }
        }
    }

}
#[derive(Parser)]
#[command(name = "campaign_cli")]
#[command(about = "Manage campaigns via the command line", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new campaign
    Create {
    #[arg(long, default_value_t = String::from("my campaign"), value_name = "campaign title")]
    campaign_title: String,

    /// The funding goal of the campaign
    #[arg(long)]
    funding_goal: u64,

    /// The duration of the campaign in seconds
    #[arg(long)]
    duration: u64,

    /// The owner of the campaign
    #[arg(long)]
    campaign_owner: String,
    },
    /// Add a contribution to the campaign
    Contribute {
        #[arg(long, value_name = "campaign-key")]
        campaign_key: usize,
        #[arg(long, value_name = "amount")]
        amount: u64,
    },
   /// Close the campaign
    Close {
         #[arg(long, value_name = "campiagn-key")]
        campaign_key: usize,
        #[arg(long, value_name = "campaign-owner")]
        campaign_owner: String,
    },
}