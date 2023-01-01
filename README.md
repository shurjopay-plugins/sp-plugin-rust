# ![alt text](https://shurjopay.com.bd/dev/images/shurjoPay.png) Shurjopay Rust Crate (plugin)

[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)]()
[![Crate](https://img.shields.io/crates/v/rand.svg)](https://docs.rs/shurjopay-plugin)
[![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://github.com/shurjopay-plugins/shurjopay-plugin)
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://docs.rs/shurjopay-plugin)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/shurjopay-plugin)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51+-lightgray.svg)]()
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


Official shurjoPay Rust Crate (plugin) for merchants or service providers to connect with **shurjoPay Payment Gateway v2.1** developed and maintained by shurjoMukhi Limited.

It handles many of the things that most people need to do manually

- Handles http request and request errors
- JSON serialization and deserialization
- Authentication during checkout and verification of payments

> 📝 **NOTE** For shurjoPay live engine integration's all necessary credential will be given to merchant after subscription completed on shurjoPay gateway.


## Audience

This document is intended for the developers and technical personnel of merchants and service providers who want to integrate the shurjoPay online payment gateway using python.

# How to use this shurjoPay Plugin

#### To integrate the shurjoPay Payment Gateway in your Rust project do the following tasks sequentially.

### step:1  Cargo.toml file Configuration

```toml
[dependencies]
shurjopay-plugin = "0.1.0"
```

### step:2  Import Crate into your project
```
use shurjopay_plugin::Shurjopay::ShurjopayPlugin;
```
### step:3  creating a new instance of Shurjopayplugin
```
let mut sp_instance = ShurjopayPlugin::new();
```
## Configure Shurjopay  
### step:4  setting configuration of Shurjopayplugin

you can configure ShurjopayPlugin two ways

* option1: configure plugin using .env file
```
sp_instance.set_config_from_env_file();
```
in this way you need to configure .env file in this way.

*** keep the .toml and .env file in the same directiory *** 

```
# .env
SP_USERNAME="sp_sandbox"
SP_PASSWORD="pyyk97hu&6u6"
POST_DEFAULT_ADDRESS="https://sandbox.shurjopayment.com"
DEFAULT_RETURN_URL="https://sandbox.shurjopayment.com/response"
DEFAULT_CANCEL_URL="https://sandbox.shurjopayment.com/response"
```


* option2: Configure plugin using this function
```
sp_instance.set_all_config(
        "sp_sandbox".to_string(),                                   // SP_USERNAME
        "pyyk97hu&6u6".to_string(),                                 // SP_PASSWORD
        "https://sandbox.shurjopayment.com".to_string(),            // POST_DEFAULT_ADDRESS
        "https://sandbox.shurjopayment.com/response".to_string(),   // DEFAULT_RETURN_URL
        "https://sandbox.shurjopayment.com/response".to_string(),   // DEFAULT_CANCEL_URL
        );
```


## Make Payment
### step:5 To intiate make payment fisrt you need configure payment request object using the following function and pass the object into make_payment_no_auto_redirect() function
```
let payment_req_obj = sp_instance.make_payment_request_object(
    "786".to_string(),              // amount
    "abc123".to_string(),           // order_id
    "BDT".to_string(),              // currency
    "Mahmudul Islam".to_string(),   // customer_name
    "Dhaka".to_string(),            // customer_address
    "01800000000".to_string(),      // customer_phone
    "Dhaka".to_string(),            // customer_city
    "1203".to_string(),             // customer_post_code
    );
```

```
let checkout_url = sp_instance.make_payment(payment_req_obj); 
```


## Verify Payment
### step:6 to verify payment you need use this function
```
let order_id = Some("sp63935da67dfd3".to_string());
let response = sp_instance.verify_payment(order_id);
println!("verify Payment Response:  {:?}",response);
if response.is_some()
{
    println!("{:#?}", response.unwrap().clone().sp_message.unwrap());
}
```

## References
1. [shurjopay Plugins API documentation](https://docs.rs/sp-plugin-rust) plugin API documentation
2. [Rust example application](https://github.com/shurjopay-plugins/sp-plugin-usage-examples/tree/dev/rust-app-rust-plugin) showing usage of the Rust crate.
3. [Sample applications and projects](https://github.com/shurjopay-plugins/sp-plugin-usage-examples) in many different languages and frameworks showing shurjopay integration.
4. [shurjoPay Postman site](https://documenter.getpostman.com/view/6335853/U16dS8ig) illustrating the request and response flow using the sandbox system.
5. [shurjopay Plugins](https://github.com/shurjopay-plugins) home page on github


## License
This code is under the [MIT open source License](LICENSE).
#### Please [contact](https://shurjopay.com.bd/#contacts) with shurjoPay team for more detail.
### Copyright ©️2022 [ShurjoMukhi Limited](https://shurjopay.com.bd/)
