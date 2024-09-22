pub mod campaign_struct;
pub mod create_campaign;
pub mod pay_campaign;
pub mod refund_contributors;
pub mod store_campaign_data;

mod register;
pub mod run_campaign;

use register::read_username_from_file;
use run_campaign::run_campaign;

fn main() {
    if let Ok(Some(username)) = read_username_from_file(){
     run_campaign(&username);
    }else{
      register::register().unwrap();  
    }
}