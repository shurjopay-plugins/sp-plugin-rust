#![allow(dead_code, unused_variables, non_snake_case, unused_imports, non_camel_case_types)]
use open;
use std::io;

mod Shurjopay;
use Shurjopay::ShurjopayPlugin;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut sp_instance = ShurjopayPlugin::new();
    sp_instance.set_config(());
    let  sp_auth_token = sp_instance.get_auth_token();
    if let Some(token)= sp_auth_token {
        if let Some(checkout_url) = sp_instance.secure_ckeckout(sp_instance.get_dummy_checkout_mgs()){
            match open::that(checkout_url.clone()) {
                Ok(()) => {
                    println!("Opened '{}' successfully.", checkout_url);
                    println!("\nPress Enter to Verify Payment after completing your payment.");
                },
                Err(err) => eprintln!("An error occurred when opening '{}': {}", checkout_url, err),
            }
                        
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            sp_instance.verifyPayment();
        }
    }

    Ok(())
}
