#![allow(dead_code, unused_variables, non_snake_case, unused_imports, non_camel_case_types)]
use open;
use std::io;

mod Shurjopay;
use Shurjopay::ShurjopayPlugin;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // creating a new instance of Shurjopayplugin
    let mut sp_instance = ShurjopayPlugin::new();
    // setting configuration of Shurjopayplugin for sandbox
    sp_instance.set_config(());
    // getting authentication token from server
    let  sp_auth_token = sp_instance.get_auth_token();
    // checking out a dummy object
    if let Some(token)= sp_auth_token {
        if let Some(checkout_url) = sp_instance.secure_ckeckout(sp_instance.get_dummy_checkout_mgs()){
            // opeing the returned checkout url in the default browser 
            match open::that(checkout_url.clone()) {
                Ok(()) => {
                    println!("Opened '{}' successfully.", checkout_url);
                    println!("\nPress Enter to Verify Payment after completing your payment.");
                },
                Err(err) => eprintln!("An error occurred when opening '{}': {}", checkout_url, err),
            }
            // Waiting to press enter            
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            // Once enter is pressed the following code will be executed
            sp_instance.verifyPayment();
        }
    }
    Ok(())
}