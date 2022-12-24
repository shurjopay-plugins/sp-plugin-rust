#[cfg(test)]
mod tests {

    use sp_plugin_rust_test::shurjopay::ShurjopayPlugin;
    use assert_str::assert_str_eq;
    

    #[test]
    fn set_config_from_env_file_test() {

       // creating a new instance of Shurjopayplugin
        let mut sp_instance = ShurjopayPlugin::new();

        // setting configuration of Shurjopayplugin for sandbox
        sp_instance.set_config_from_env_file();
        let post_default_address = sp_instance.config.clone().unwrap().post_default_address;
        let token_end_point = sp_instance.config.clone().unwrap().token_end_point;
        let secure_payment_end_point = sp_instance.config.clone().unwrap().secure_payment_end_point;
        let verification_end_point = sp_instance.config.clone().unwrap().verification_end_point;
        let payment_status_end_point = sp_instance.config.clone().unwrap().payment_status_end_point;
        let sp_user = sp_instance.config.clone().unwrap().sp_user;
        let sp_pass = sp_instance.config.clone().unwrap().sp_pass;
        let default_return_url = sp_instance.config.clone().unwrap().default_return_url;
        let default_cancel_url = sp_instance.config.clone().unwrap().default_cancel_url;
        
        assert_str_eq!(post_default_address,"https://sandbox.shurjopayment.com".to_string());
        assert_str_eq!(token_end_point,"/api/get_token".to_string());
        assert_str_eq!(secure_payment_end_point,"/api/secret-pay".to_string());
        assert_str_eq!(verification_end_point,"/api/verification".to_string());
        assert_str_eq!(payment_status_end_point,"/api/payment-status".to_string());
        assert_str_eq!(sp_user,"sp_sandbox".to_string());
        assert_str_eq!(sp_pass,"pyyk97hu&6u6".to_string());
        assert_str_eq!(default_return_url,"https://sandbox.shurjopayment.com/response".to_string());
        assert_str_eq!(default_cancel_url,"https://sandbox.shurjopayment.com/response".to_string());
       

    }


    #[test]
    fn set_all_config_test() {

        // creating a new instance of Shurjopayplugin
         let mut sp_instance = ShurjopayPlugin::new();
 
         // setting configuration of Shurjopayplugin for sandbox
         sp_instance.set_all_config(
        "sp_sandbox".to_string(),
        "pyyk97hu&6u6".to_string(),
        "https://sandbox.shurjopayment.com".to_string(),
        "/api/get_token".to_string(),
        "/api/secret-pay".to_string(),
        "/api/verification".to_string(),
        "/api/payment-status".to_string(),
        "https://sandbox.shurjopayment.com/response".to_string(),
        "https://sandbox.shurjopayment.com/response".to_string(),
        );


         let post_default_address = sp_instance.config.clone().unwrap().post_default_address;
         let token_end_point = sp_instance.config.clone().unwrap().token_end_point;
         let secure_payment_end_point = sp_instance.config.clone().unwrap().secure_payment_end_point;
         let verification_end_point = sp_instance.config.clone().unwrap().verification_end_point;
         let payment_status_end_point = sp_instance.config.clone().unwrap().payment_status_end_point;
         let sp_user = sp_instance.config.clone().unwrap().sp_user;
         let sp_pass = sp_instance.config.clone().unwrap().sp_pass;
         let default_return_url = sp_instance.config.clone().unwrap().default_return_url;
         let default_cancel_url = sp_instance.config.clone().unwrap().default_cancel_url;

         
         assert_str_eq!(post_default_address,"https://sandbox.shurjopayment.com".to_string());
         assert_str_eq!(token_end_point,"/api/get_token".to_string());
         assert_str_eq!(secure_payment_end_point,"/api/secret-pay".to_string());
         assert_str_eq!(verification_end_point,"/api/verification".to_string());
         assert_str_eq!(payment_status_end_point,"/api/payment-status".to_string());
         assert_str_eq!(sp_user,"sp_sandbox".to_string());
         assert_str_eq!(sp_pass,"pyyk97hu&6u6".to_string());
         assert_str_eq!(default_return_url,"https://sandbox.shurjopayment.com/response".to_string());
         assert_str_eq!(default_cancel_url,"https://sandbox.shurjopayment.com/response".to_string());
        
 
     }



     #[test]
     fn make_payment_test()
     {
        let mut sp_instance = ShurjopayPlugin::new();
        sp_instance.set_config_from_env_file();

        let payment_req_obj = sp_instance.make_payment_request_object(
            "786".to_string(),
            "abc123".to_string(),
            "BDT".to_string(),
            "Mahmudul Islam".to_string(),
            "Dhaka".to_string(),
            "01811177722".to_string(),
            "Dhaka".to_string(),
            "1203".to_string(),
            );
        

        if let Some(checkout_url) = sp_instance.MakePayment(payment_req_obj) {

            // opeing the returned checkout url in the default browser 
            match open::that(checkout_url.clone()) {
                Ok(()) => {
                    // println!("Opened '{}' successfully.", checkout_url);
                    // println!("\nPress Enter to Verify Payment after completing your payment.");
                },
                Err(err) => eprintln!("An error occurred when opening '{}': {}", checkout_url, err),
            }
        }

    }

    #[test]
     fn verify_false_order_id_test()
     {
        let mut sp_instance = ShurjopayPlugin::new();
        sp_instance.set_default_config();

        let response = sp_instance.verifyPayment(Some("random_oder_id_123".to_string()));
            // print!("verify Payment Response: ");
            // println!("{:?}",response);
        if response.is_some()
        {
            assert_str_eq!(response.clone().unwrap().sp_message.unwrap(),"Please check your order id".to_string());
            assert_eq!(response.clone().unwrap().sp_code.unwrap(),1011);

        }

     }




    
}
