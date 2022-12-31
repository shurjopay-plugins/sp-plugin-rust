#[cfg(test)]
mod tests {

    use shurjopay_plugin::shurjopay::ShurjopayPlugin;
    use assert_str::assert_str_eq;
    use webbrowser;

    #[test]
    fn set_config_from_env_file_test() {

       // creating a new instance of Shurjopayplugin
        let mut sp_instance = ShurjopayPlugin::new();

        // setting configuration of Shurjopayplugin for sandbox
        sp_instance.set_config_from_env_file();
        let post_default_address = sp_instance.config.clone().unwrap().post_default_address;
        let sp_user = sp_instance.config.clone().unwrap().sp_user;
        let sp_pass = sp_instance.config.clone().unwrap().sp_pass;
        let default_return_url = sp_instance.config.clone().unwrap().default_return_url;
        let default_cancel_url = sp_instance.config.clone().unwrap().default_cancel_url;
        
        assert_str_eq!(post_default_address,"https://sandbox.shurjopayment.com".to_string());
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
        "https://sandbox.shurjopayment.com/response".to_string(),
        "https://sandbox.shurjopayment.com/response".to_string(),
        );


         let post_default_address = sp_instance.config.clone().unwrap().post_default_address;
         let sp_user = sp_instance.config.clone().unwrap().sp_user;
         let sp_pass = sp_instance.config.clone().unwrap().sp_pass;
         let default_return_url = sp_instance.config.clone().unwrap().default_return_url;
         let default_cancel_url = sp_instance.config.clone().unwrap().default_cancel_url;

         
         assert_str_eq!(post_default_address,"https://sandbox.shurjopayment.com".to_string());
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
        

        if let Some(checkout_url) = sp_instance.make_payment_no_auto_redirect(payment_req_obj) {
            
            if webbrowser::open(checkout_url.clone().as_str()).is_ok() {
                println!("Opened '{}' successfully.", checkout_url.clone())
            }
            else {
                println!("An error occurred when opening {}", checkout_url);
            }
        }
    }


    #[test]
    fn make_payment_auto_redirect_test()
     {
        let mut sp_instance = ShurjopayPlugin::new();
        sp_instance.set_config_from_env_file();

        let payment_req_obj = sp_instance.make_payment_request_object(
            "1000".to_string(),
            "unyhl123".to_string(),
            "BDT".to_string(),
            "Mahmudul Islam".to_string(),
            "Dhaka".to_string(),
            "01811177722".to_string(),
            "Dhaka".to_string(),
            "1203".to_string(),
            );
        

        sp_instance.make_payment(payment_req_obj);

    }

    #[test]
     fn verify_false_order_id_test()
     {
        let mut sp_instance = ShurjopayPlugin::new();
        sp_instance.set_default_config();

        let response = sp_instance.verify_payment(Some("random_oder_id_123".to_string()));
            // print!("verify Payment Response: ");
            // println!("{:?}",response);
        if response.is_some()
        {
            assert_str_eq!(response.clone().unwrap().sp_message.unwrap(),"Please check your order id".to_string());
            assert_eq!(response.clone().unwrap().sp_code.unwrap(),1011);
        }

    }
    
}
