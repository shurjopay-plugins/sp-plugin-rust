![image](https://user-images.githubusercontent.com/57352037/170198396-932692aa-3354-4cf0-abc1-2b8ef43a6de3.png)
# ShurjoPay

[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)]()
[![Crate](https://img.shields.io/crates/v/rand.svg)]()
[![Book](https://img.shields.io/badge/book-master-yellow.svg)]()
[![API](https://img.shields.io/badge/api-master-yellow.svg)]()
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/rand)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51+-lightgray.svg)]()


# Shurjopaypayment gateway Rust Crate

It handles many of the things that most people need to do manually

- Handles http request and request errors
- JSON serialization and deserialization
- Authentication during checkout and verification of payments


### Shurjopay Rust integration steps

> 📝 **NOTE** For shurjoPay live engine integration's all necessary credential will be given to merchant after subscription completed on shurjoPay gateway.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sp-plugin-rust-test = "0.1.0"
```




# Shurjopay-Rust-Plugin
#### To integrate the shurjoPay Payment Gateway in your Rust project do the following tasks sequentially.

### step:1  Cargo.toml file Configuration

```toml
[dependencies]
sp-plugin-rust-test = "0.1.0"
open = "3.0.3"
```

### step:2  Import Crate into your project
```
use sp_plugin_rust_test::Shurjopay::ShurjopayPlugin;
use open;
use std::io;
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
// .env
POST_DEFAULT_ADDRESS="https://sandbox.shurjopayment.com"
TOKEN_END_POINT="/api/get_token"
SECURE_PAYMENT_END_POINT="/api/secret-pay"
VERIFICATION_END_POINT="/api/verification"
PAYMENT_STATUS_END_POINT="/api/payment-status"
SP_USER="sp_sandbox"
SP_PASS="pyyk97hu&6u6"
DEFAULT_RETURN_URL="https://sandbox.shurjopayment.com/response"
DEFAULT_CANCEL_URL="https://sandbox.shurjopayment.com/response"
DEFAULT_CLIENT_IP="192.168.0.99"
```


* option2: Configure plugin using this function
```
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
        "192.168.0.99".to_string(),);
```


## Make Payment
### step:5 To intiate make payment fisrt you need configure payment request object using the following function and pass the object into MakePayment() function
```
let payment_req_obj = sp_instance.make_payment_request_object(
    "786".to_string(),              // amount
    "abc123".to_string(),           // order_id
    "BDT".to_string(),              // currency
    "Mahmudul Islam".to_string(),   // customer_name
    "Dhaka".to_string(),            // customer_address
    "01811177722".to_string(),      // customer_phone
    "Dhaka".to_string(),            // customer_city
    "1203".to_string(),             // customer_post_code
    );
```


```
if let Some(checkout_url) = sp_instance.MakePayment(payment_req_obj) 
{
		// opeing the returned checkout url in the default browser 
		match open::that(checkout_url.clone()) {
				Ok(()) => {
				},
				Err(err) => eprintln!("An error occurred when opening '{}': {}", checkout_url, err),
		}
```


## Verify Payment
### step:6 to verify payment you need use this function
```
let response = sp_instance.verifyPayment(Some("sp63935da67dfd3".to_string()));
println!("verify Payment Response:  {:?}",response);
if response.is_some()
{
		println!("{:#?}", response.unwrap().clone().sp_message.unwrap());
}
```

## Sample Example
you can download a example from the below repository for better understanding

```
https://github.com/shurjopay-plugins/sp-plugin-usage-examples
```

## License

Licensed under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)

### Who do I talk to? ###
	For any technical assistance please contact to: https://shurjopay.com.bd/#contacts
