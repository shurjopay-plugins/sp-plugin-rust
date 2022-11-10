#![allow(dead_code, unused_variables, non_snake_case, unused_imports, non_camel_case_types)]
use open;
use std::io;

mod Shurjopay;
use Shurjopay::{ShurjopayPlugin, SpCheckout};

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // creating a new instance of Shurjopayplugin
    let mut sp_instance = ShurjopayPlugin::new();
    // setting configuration of Shurjopayplugin for sandbox
    sp_instance.set_config(());
    // getting authentication token from server
    // checking out with a dummy checkout mgs
    if let Some(checkout_url) = sp_instance.MakePayment(get_dummy_checkout_mgs()) {
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
    // }
    Ok(())
}



pub fn get_dummy_checkout_mgs() -> Shurjopay::SpCheckout {
    Shurjopay::SpCheckout{
        prefix: "sp".to_string(),
        token: "".to_string(),
        return_url: "https://www.sandbox.shurjopayment.com/response".to_string(),
        cancel_url: "https://www.sandbox.shurjopayment.com/response".to_string(),
        store_id: "".to_string(),
        amount: "10".to_string(),
        order_id: "svd6asv1a".to_string(),
        currency: "BDT".to_string(),
        customer_name: "Shakil Anwar".to_string(),
        customer_address: "Dhaka".to_string(),
        customer_phone: "01521308009".to_string(),
        customer_city: "Dhaka".to_string(),
        customer_post_code: "1000".to_string(),
        client_ip: "192.168.0.99".to_string(),
    }
}