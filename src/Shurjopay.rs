//!
//! A simple module to quickly integrate Shurjopay payment gateway
//! service into rust web program.
//! 
//! Features:
//! - Automatic handles html errors 
//! - Authenticates automatically during makePayments or verifyingPayments
//! 


/// Standard library to save `key` and `value` as Hashmap
extern crate std;
use std::collections::HashMap;

/// The `chrono` crate is included to calculate timeout using datetime 
extern crate chrono;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, FixedOffset, Duration, Timelike};
use chrono::format::{ParseError, format};

/// The `serde` crate is included to serialize structure to json and deserialize json to structure 
extern crate serde;
use serde::{Deserialize, Serialize};
use serde_json::Result;

/// The `log` crate is included to export log for debug purpose
extern crate log;
use log::{debug, error, info, warn};

/// The `reqwest` crate is included to make http request
extern crate reqwest;
use reqwest::blocking::Client;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};

/// This module handles http request verifications
pub mod ShurjopayClient;


/// Shurjopay token authorization data structure
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpAuthToken {
    pub token: String,
    pub store_id: i32,
    pub execute_url: String,
    pub token_type: String,
    pub sp_code: String,
    pub message: String,
    pub token_create_time: String,
    pub expires_in: i64,
}

/// Shurjopay checkout data structure
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
/// Each element of the structure must hold a value before checking out
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpCheckout {
    prefix:String,
    token:String,
    return_url:String,
    cancel_url:String,
    store_id:String,
    amount:String,
    order_id:String,
    currency:String,
    customer_name:String,
    customer_address:String,
    customer_phone:String,
    customer_city:String,
    customer_post_code:String,
    client_ip:String,
}


/// Shurjopay checkout response data structure
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
/// `customer_email` can hold `null` value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpCheckoutResponse {
   checkout_url: String,
   amount: String,
   currency: String,
   sp_order_id: String,
   customer_order_id: String,
   customer_name: String,
   customer_address: String,
   customer_city: String,
   customer_phone: String,
   customer_email: serde_json::value::Value,
   client_ip: String,
   intent: String,
   transactionStatus: String,
}

/// Shurjopay payment verifiacation data structure
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
/// `discsount_amount` , `card_holder_name`, `card_number, `email`, `transaction_status`, 
/// `method`, `value1`, `value2`, `value3`, `value4` can hold `null` value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpVerifyResponse {
    id:i64,
    order_id:String,
    currency:String,
    amount:f64,
    payable_amount:f64,
    discsount_amount:serde_json::value::Value,
    disc_percent:f64,
    received_amount:f64,
    usd_amt:f64,
    usd_rate:f64,
    card_holder_name:serde_json::value::Value,
    card_number:serde_json::value::Value,
    phone_no:String,
    bank_trx_id:String,
    invoice_no:String,
    bank_status:String,
    customer_order_id:String,
    sp_code:i64,
    sp_message:String,
    name:String,
    email:serde_json::value::Value,
    address:String,
    city:String,
    value1:serde_json::value::Value,
    value2:serde_json::value::Value,
    value3:serde_json::value::Value,
    value4:serde_json::value::Value,
    transaction_status:serde_json::value::Value,
    method:serde_json::value::Value,
    date_time:String,
}

/// Shurjopay configuration data structure
/// This structure should be declared as non mutable 
/// and ownership shouldn't be transferred to any other instance
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpConfig {
    _POST_DEFAULT_ADDRESS: &'static str,
    _TOKEN_END_POINT: &'static str,
    _SECURE_PAYMENT_END_POINT: &'static str,
    _VERIFICATION_END_POINT: &'static str,
    _PAYMENT_STATUS_END_POINT: &'static str,
    _SANDBOX_USERNAME: &'static str,
    _SANDBOX_PASSWORD: &'static str,
    _DEFAULT_RETURN_URL: String,
    _DEFAULT_CANCEL_URL: String,
    _DEFAULT_CLIENT_IP: String,
}

/// This the model user will create as a Shurjopay plugin instance
/// This structure should be declared as mutable
/// ShurjopayPlugin::set_config() must be called before making a payment request
/// and ownership shouldn't be transferred to any other instance
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
/// 
/// # Usage
/// 
/// ```
/// let mut sp_instance = ShurjopayPlugin::new();
/// sp_instance.set_config(("url", "username", "password", "return_url", "cancel_url"));
/// ```
/// 
#[derive(Debug, Clone)]
pub struct ShurjopayPlugin{
    client: Option<reqwest::blocking::Client>,
    config: Option<SpConfig>,
    auth_token: Option<SpAuthToken>,
    checkout_response: Option<SpCheckoutResponse>,
    verify_response: Option<SpVerifyResponse>,
    check_response: Option<SpVerifyResponse>,
    pub token_create_time: Option<NaiveDateTime>,
    pub token_expire_time: Option<NaiveDateTime>,
}

/// A trait to initialize 'Shurjopay Configuration' with function overloadding.
pub trait New<T> {
    fn new(arg: T) -> Self;
}

/// This traits implements initiation with sandbox configurations
impl New<()> for SpConfig {
    fn new(arg:()) -> Self {
        SpConfig::new(("sp_sandbox", "pyyk97hu&6u6"))
    }
}

/// This traits implements initiation with sandbox configurations with custom username and password
impl New<(&'static str, &'static str)> for SpConfig {
    fn new(arg: (&'static str, &'static str)) -> Self {
        SpConfig::new(("https://sandbox.shurjopayment.com", 
                            arg.0, 
                            arg.1,
                            "https://www.sandbox.shurjopayment.com/response", 
                            "https://www.sandbox.shurjopayment.com/response"))
    }
}

/// This traits called to initiate SpConfig with proper initialization
impl New<(&'static str, &'static str, &'static str, &'static str, &'static str)> for SpConfig {
    fn new(arg: (&'static str, &'static str, &'static str, &'static str, &'static str)) -> Self {
        SpConfig {
            _POST_DEFAULT_ADDRESS: arg.0,
            _TOKEN_END_POINT: "/api/get_token",
            _SECURE_PAYMENT_END_POINT: "/api/secret-pay",
            _VERIFICATION_END_POINT: "/api/verification",
            _PAYMENT_STATUS_END_POINT: "/api/payment-status",
            _SANDBOX_USERNAME: arg.1,
            _SANDBOX_PASSWORD: arg.2,
            _DEFAULT_RETURN_URL: String::from(arg.3),
            _DEFAULT_CANCEL_URL: String::from(arg.4),
            _DEFAULT_CLIENT_IP: String::from("192.168.0.99"),
        }
    }
}


impl SpAuthToken {
    /// Takes `SpAuthToken` structure and and converts `String` timestamp stored in `SpAuthToken.token_create_time` 
    /// return time in `NativeDateTime`
    pub fn get_time(&self) -> Option<NaiveDateTime> {
        // let custom = DateTime::parse_from_str("2022-11-02 05:26:19pm", "%d-%m-%Y %H:%M:%S%P")?;
        let time_t = self.token_create_time.clone();
        let mut time_offset:i64 = 0;
        let mut time_val: String = "".to_string();

        // Checking the time is in AM or PM
        println!(" {:?}", time_t.find("pm"));
        if let Some(val) = time_t.find("pm") {
            time_offset = 12;
            let (first, last) = time_t.split_at(val);
            time_val = first.to_string();
        } else if let Some(val) = time_t.find("am") {
            time_offset = 0;
            let (first, last) = time_t.split_at(val);
            time_val = first.to_string();
        }
        // println!("time offset: {} Time: {:?}", time_offset, time_val);
        let time_T = NaiveDateTime::parse_from_str(time_val.as_str(), "%Y-%m-%d %H:%M:%S");
        // println!("Token create time: {:?}", time_T);
        // Correcting time based on AM or PM
        match time_T {
            Ok(time) => {
                if time.hour() == 12 && time_offset == 12 {
                    return Some(time);
                } else {
                    let time_24 = time+ Duration::hours(time_offset);
                    // println!("Token create time: {}", time_24);
                    return Some(time_24);
                }
            },
            Err(_)=> return None, 
        }
    }
}


impl ShurjopayPlugin {
    /// This is a constructor to initiate `null` instance of `ShurjopayPlugin`
    /// returns `ShurjopayPlugin`
    pub fn new() -> Self {
        let http_client = reqwest::blocking::Client::new();
        ShurjopayPlugin{            
            client : Some(http_client),
            config : None,
            auth_token: None,
            checkout_response: None,
            verify_response: None,
            check_response: None,
            token_create_time : None,
            token_expire_time : None,
        }
    }

    /// This function takes arguments to implement `New` trait for `SpConfig`
    pub fn set_config<T>(&mut self, arg:T) where SpConfig: New<T> {
        let sp_config = SpConfig::new(arg);
        self.config  = Some(sp_config);
    }
}


impl ShurjopayPlugin{

    /// This function is called to check any old payment status
    pub fn checkPayment(&mut self)-> Option<SpVerifyResponse> {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // println!("{:?}", spay);
            // Checking if client is valid or not
            if let Some(client) = sp_ins.client{
                // Constructing url, header and body
                let url = format!("{}{}/",spay._POST_DEFAULT_ADDRESS, spay._VERIFICATION_END_POINT);
                let body = format!("{{\"order_id\": \"{}\"}}", self.checkout_response.clone().unwrap().sp_order_id);
                let header =format!{"{} {}", self.auth_token.clone().unwrap().token_type, self.auth_token.clone().unwrap().token };
                
                // Making HTTP request
                let response = client.post(url.as_str())
                                .header("Content-Type", "application/json")
                                .header("Authorization", header)
                                .body(body)
                                .send();

                // Checking if respons is valid or not
                if let Some(responseData) = ShurjopayClient::is_response_valid(response) {
                    // println!("Checkout Response: {:?}", responseData);

                    // Mapping JSON string to structure
                    let verify_json_option: Option<SpVerifyResponse> = unwrap_json(&responseData);
                    
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  verify_json_option {
                        self.verify_response = Some(valid_json_data.clone());
                        println!("Checkout Response: {:?}", valid_json_data);
                        return Some(valid_json_data);
                    } else {
                        self.verify_response = None;
                        println!("{:?}", responseData);                        
                    }                    
                }
            }
        }
        return None;
    }

    /// This function is called to verify payments only once
    /// Further verification can be done by `checkPayments` function
    pub fn verifyPayment(&mut self)-> Option<SpVerifyResponse> {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // Checking if client is valid or not
            if let Some(client) = sp_ins.client{
                // Constructing url, header and body
                let url = format!("{}{}/",spay._POST_DEFAULT_ADDRESS, spay._VERIFICATION_END_POINT);
                let body = format!("{{\"order_id\": \"{}\"}}", self.checkout_response.clone().unwrap().sp_order_id);
                // let body = "{\"order_id\": \"sp636384e391650\"}";
                let header =format!{"{} {}", self.auth_token.clone().unwrap().token_type, self.auth_token.clone().unwrap().token };
                
                // Making HTTP request
                let response = client.post(url.as_str())
                                .header("Content-Type", "application/json")
                                .header("Authorization", header)
                                .body(body)
                                .send();

                // Checking if respons is valid or not
                if let Some(responseData) = ShurjopayClient::is_response_valid(response) {
                    // println!("Checkout Response: {:?}", responseData);
                    // Mapping JSON string to structure
                    let verify_json_option: Option<SpVerifyResponse> = unwrap_json(&responseData);
                    
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  verify_json_option {
                        self.verify_response = Some(valid_json_data.clone());
                        println!("Checkout Response: {:?}", valid_json_data);
                        return Some(valid_json_data);
                    } else {
                        self.verify_response = None;
                        println!("{:?}", responseData);                        
                    }                    
                }
            }
        }
        return None;
    }

    // pub fn MakePayment(&mut self, checkout_item: SpCheckout)->Option<String> {

    // }
    
    /// This function sends a checkout structure to the Shurjopay server
    /// It returns `Option<checkout_url>` for the frontend
    pub fn secure_ckeckout(&mut self, checkout_item: SpCheckout)->Option<String> {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // println!("{:?}", spay);
            if let Some(client) = sp_ins.client{
                let url = format!("{}{}/",spay._POST_DEFAULT_ADDRESS, spay._SECURE_PAYMENT_END_POINT);
                let body_json = serde_json::to_string(&checkout_item);

                // Making HTTP request
                let response = client.post(url.as_str())
                                .header("Content-Type", "application/json")
                                .body(body_json.unwrap())
                                .send();

                // Checking if respons is valid or not
                if let Some(responseData) = ShurjopayClient::is_response_valid(response) {
                    // println!("Checkout Response: {:?}", responseData);
                    // Mapping JSON string to structure
                    let checkout_json_option: Option<SpCheckoutResponse> = unwrap_json(&responseData);
                    
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  checkout_json_option {
                        self.checkout_response = Some(valid_json_data.clone());
                        println!("Checkout Response: {:?}", valid_json_data);
                        return Some(valid_json_data.checkout_url);
                    } else {
                        self.checkout_response = None;
                        println!("{:?}", responseData);                        
                    }                    
                }
            }
        }
        return None;
    }


    /// This function gets auth token before initiating communication with `Shurjopay server`
    /// It returns `Option<auth_token>`
    pub fn get_auth_token(&mut self) -> Option<String> 
    {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // println!("{:?}", spay);
            if let Some(client) = sp_ins.client{
                let url = format!("{}{}/",spay._POST_DEFAULT_ADDRESS, spay._TOKEN_END_POINT);

                let mut body = HashMap::new();
                body.insert("username", spay._SANDBOX_USERNAME);
                body.insert("password", spay._SANDBOX_PASSWORD);

                // Making HTTP request
                let response = client.post(url.as_str())
                                .header("Content-Type", "application/json")
                                .json(&body)
                                .send();
                // Checking if respons is valid or not
                if let Some(responseData) = ShurjopayClient::is_response_valid(response) {
                    let auth_json_option: Option<SpAuthToken> = unwrap_json(&responseData);
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  auth_json_option {
                        self.auth_token = Some(valid_json_data.clone());
                        self.set_expire_time();
                        return Some(valid_json_data.token);
                    } else {
                        println!("{:?}", responseData);
                    }
                }
            } else {
                println!("Shurjopay http client is not set yet!");
            }            
        } else {
            println!("Shurjopay Configuration is not set yet!");
        }
        return None;
    }


    pub fn get_http_response(&mut self, url: String, header: String, body: String) -> Option<ShurjopayClient::HttpResponse> {
        if let Some(client) = self.client.clone() {
            let response = client.post(url.as_str())
                                .header("Content-Type", "application/json")
                                .json(&body)
                                .send();

            let res =  ShurjopayClient::is_response_valid(response);
        }
        return None;
    } 

    /// This function extracts expiration time of authenticaton token
    fn set_expire_time(&mut self) {
        let json_data_option = self.auth_token.clone();
        if let Some(json_data) = json_data_option {
            self.token_create_time = json_data.get_time();
            if let Some(token_ctime) = self.token_create_time.clone() {
                if json_data.expires_in != 0 {
                    self.token_expire_time = Some(token_ctime+Duration::seconds(json_data.expires_in));
                } else {
                    self.token_expire_time = None;
                }
            } else {
                self.token_create_time = None;
            }
        }
    }

    pub fn get_dummy_checkout_mgs(&self) -> SpCheckout {
        let sp_config = self.config.clone().unwrap();
        let sp_auth_token = self.auth_token.clone().unwrap();

        SpCheckout{
            prefix: "sp".to_string(),
            token: self.auth_token.clone().unwrap().token,
            return_url: sp_config._DEFAULT_RETURN_URL,
            cancel_url: sp_config._DEFAULT_CANCEL_URL,
            store_id: sp_auth_token.store_id.to_string(),
            amount:"10".to_string(),
            order_id: "svd6asv1a".to_string(),
            currency: "BDT".to_string(),
            customer_name: "Shakil Anwar".to_string(),
            customer_address: "Dhaka".to_string(),
            customer_phone: "01521308009".to_string(),
            customer_city: "Dhaka".to_string(),
            customer_post_code:"1000".to_string(),
            client_ip: sp_config._DEFAULT_CLIENT_IP,
        }

    }
}


/// This function unwraps `JSON` `String` into specified `<T>` data structure
pub fn unwrap_json<'a, T>(response_data: &'a ShurjopayClient::HttpResponse) -> Option<T> 
where T: Deserialize<'a> {
    if response_data.http_code == 200
    {
        let json_data : Result<T> = serde_json::from_str(response_data.http_body.as_str());
        match json_data {
            Ok(data) => return Some(data),
            Err(_) => return None,
        }
    } else {
        println!("{:?}", response_data);
    }
    return None
}

