//!
//! A simple module to quickly integrate Shurjopay payment gateway
//! service into rust web program.
//! 
//! Features:
//! - Automatic handles html errors 
//! - Authenticates automatically during makePayments or verifyingPayments
//! 

#![allow(dead_code, unused_variables, non_snake_case, non_camel_case_types)]

/// Standard library to save `key` and `value` as Hashmap
extern crate std;
use std::collections::HashMap;

/// The `chrono` crate is included to calculate timeout using datetime 
extern crate chrono;
use chrono::{NaiveDateTime, Duration, Timelike, Utc};
// use chrono::format::{ParseError, format};

/// dotenv crate is used to fetch information from .env file
use dotenv::dotenv;

/// The `serde` crate is included to serialize structure to json and deserialize json to structure 
extern crate serde;
use serde::{Deserialize, Serialize};
// use serde_json::{Result, to_string};
use serde_json:: Result;

// The `log` crate is included to export log for debug purpose
// extern crate log;
// use log::{debug, error, info, warn};

/// The `reqwest` crate is included to make http request
extern crate reqwest;
// use reqwest::blocking::Client;
// use reqwest::Error;
// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use reqwest::header::CONTENT_TYPE;

/// This module handles http request verifications
use super::shurjopay_client;//::{HttpResponse,is_response_valid};


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
    pub prefix:String,
    pub token:String,
    pub return_url:String,
    pub cancel_url:String,
    pub store_id:String,
    pub amount:String,
    pub order_id:String,
    pub currency:String,
    pub customer_name:String,
    pub customer_address:String,
    pub customer_phone:String,
    pub customer_city:String,
    pub customer_post_code:String,
    pub client_ip:String,
}

impl Default for SpCheckout
{
    /// This function will set default value for SpConfig struct
    fn default() -> Self 
    {
        SpCheckout 
        { 
            prefix: "sp".to_string(),
            token: "".to_string(),
            return_url: "".to_string(),
            cancel_url: "".to_string(),
            store_id: "".to_string(),
            amount: "".to_string(),
            order_id: "".to_string(),
            currency: "".to_string(),
            customer_name: "".to_string(),
            customer_address: "".to_string(),
            customer_phone: "".to_string(),
            customer_city: "".to_string(),
            customer_post_code: "".to_string(),
            client_ip: "".to_string(),
        }
    }
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
    pub sp_code:Option<i64>,
    #[serde(default)]
    pub id:Option<i64>,
    pub order_id:Option<String>,
    pub currency:Option<String>,
    pub amount:Option<f64>,
    pub payable_amount:Option<f64>,
    pub discsount_amount:Option<f64>,
    pub disc_percent:Option<f64>,
    pub received_amount:Option<String>,
    pub usd_amt:Option<f64>,
    pub usd_rate:Option<f64>,
    pub card_holder_name:Option<String>,
    pub card_number:Option<String>,
    pub phone_no:Option<String>,
    pub bank_trx_id:Option<String>,
    pub invoice_no:Option<String>,
    pub bank_status:Option<String>,
    pub customer_order_id:Option<String>,
    pub sp_message:Option<String>,
    pub name:Option<String>,
    pub email:Option<String>,
    pub address:Option<String>,
    pub city:Option<String>,
    pub value1:Option<String>,
    pub value2:Option<String>,
    pub value3:Option<String>,
    pub value4:Option<String>,
    pub transaction_status:Option<String>,
    pub method:Option<String>,
    pub date_time:Option<String>,
    
}

/// Shurjopay payment another verifiacation data structure
/// This structure comes when payment is not successful
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpVerifyResponse2
{
    sp_code:Option<String>,
    message: Option<String>
}

/// Shurjopay configuration data structure
/// This structure should be declared as non mutable 
/// and ownership shouldn't be transferred to any other instance
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpConfig {
    pub post_default_address:  String,
    pub token_end_point:  String,
    pub secure_payment_end_point:  String,
    pub verification_end_point:  String,
    pub payment_status_end_point:  String,
    pub sp_user:  String,
    pub sp_pass:  String,
    pub default_return_url: String,
    pub default_cancel_url: String,
    pub default_client_ip: String,
}

impl Default for SpConfig
{
    /// This function will set default value for SpConfig struct
    fn default() -> Self 
    {
        SpConfig 
        { 
            post_default_address: "https://sandbox.shurjopayment.com".to_string(), 
            token_end_point: "/api/get_token".to_string(), 
            secure_payment_end_point: "/api/secret-pay".to_string(), 
            verification_end_point: "/api/verification".to_string(), 
            payment_status_end_point: "/api/payment-status".to_string(), 
            sp_user: "sp_sandbox".to_string(), 
            sp_pass: "pyyk97hu&6u6".to_string(), 
            default_return_url: "https://www.sandbox.shurjopayment.com/response".to_string(), 
            default_cancel_url: "https://www.sandbox.shurjopayment.com/response".to_string(), 
            default_client_ip: "192.168.0.99".to_string() ,
        }    
    }
}


/// This the model user will create as a Shurjopay plugin instance
/// This structure should be declared as mutable
/// ShurjopayPlugin::set_config() must be called before making a payment request
/// and ownership shouldn't be transferred to any other instance
/// This structure implements `Serialize`, `Deserialize`, `Debug` and `Clone` functions
/// 
/// 
#[derive(Debug, Clone)]
pub struct ShurjopayPlugin{
    client: Option<reqwest::blocking::Client>,
    pub config: Option<SpConfig>,
    pub auth_token: Option<SpAuthToken>,
    pub checkout_response: Option<SpCheckoutResponse>,
    pub verify_response: Option<SpVerifyResponse>,
    pub check_response: Option<SpVerifyResponse>,
    pub token_create_time: Option<NaiveDateTime>,
    pub token_expire_time: Option<NaiveDateTime>,
}

/// A trait to initialize 'Shurjopay Configuration' with function overloadding.
pub trait New<T> {
    fn new(arg: T) -> Self;
}

/// implementation for ShurjopayPlugin
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



    /// This function will set default value for `ShurjopayPlugin`'s Config
    pub fn set_default_config(&mut self)
    {
        let sp_config = SpConfig
        {
            ..Default::default()
        };

        self.config  = Some(sp_config);
    } 
    
    /// This function will set username and password for `ShurjopayPlugin`'s Config 
    /// and keep other Config's value default
    pub fn set_config_username_password(&mut self, sp_user:String, sp_pass: String )
    {
        let sp_config = SpConfig
        {
            sp_user,
            sp_pass,
            ..Default::default()
        };
        self.config  = Some(sp_config);
    }
    

    /// Using this function all the member of ShurjopayPluging Config can be set
    pub fn set_all_config(&mut self, 
        sp_user:String, 
        sp_pass: String, 
        post_default_address:String,
        token_end_point:String,
        secure_payment_end_point:String,
        verification_end_point:String,
        payment_status_end_point:String,
        default_return_url:String,
        default_cancel_url:String,
        default_client_ip:String,
    )
    {
        let sp_config = SpConfig
        {
            sp_user,
            sp_pass,
            post_default_address,
            token_end_point,
            secure_payment_end_point,
            verification_end_point,
            payment_status_end_point,
            default_return_url,
            default_cancel_url,
            default_client_ip,
        };
        self.config  = Some(sp_config);
    }



    /// Using this ShurjopayPlugin config can be set from .env file
    pub fn set_config_from_env_file(&mut self)
    {
        if check_env_file_availble()
        {
            let sp_config = SpConfig
            {
                post_default_address: std::env::var("POST_DEFAULT_ADDRESS").unwrap(),
                token_end_point: std::env::var("TOKEN_END_POINT").unwrap(),
                secure_payment_end_point: std::env::var("SECURE_PAYMENT_END_POINT").unwrap(),
                verification_end_point: std::env::var("VERIFICATION_END_POINT").unwrap(),
                payment_status_end_point: std::env::var("PAYMENT_STATUS_END_POINT").unwrap(),
                sp_user: std::env::var("SP_USER").unwrap(),
                sp_pass: std::env::var("SP_PASS").unwrap(),
                default_return_url: std::env::var("DEFAULT_RETURN_URL").unwrap(),
                default_cancel_url: std::env::var("DEFAULT_CANCEL_URL").unwrap(),
                default_client_ip: std::env::var("DEFAULT_CLIENT_IP").unwrap(),
            };
    
            self.config  = Some(sp_config);
        }
        else 
        {
            println!(".env file not available & config is not set");
            self.config = None;
        }
        
    }


    
    /// this function helps to generate make request object
    /// argument : amount, order_id, currency, customer_name, customer_address, 
    /// customer_phone, customer_phone, customer_city, customer_post_code
    pub fn make_payment_request_object(&mut self,
        amount:String,
        order_id:String,
        currency:String,
        customer_name:String,
        customer_address:String,
        customer_phone:String,
        customer_city:String,
        customer_post_code:String,
    ) -> SpCheckout
    {
        let sp_checkout =SpCheckout
        {
          prefix: "sp".to_string(),
          token: "".to_string(),//self.auth_token.clone().unwrap().token,
          return_url: self.config.clone().unwrap().default_return_url,
          cancel_url:self.config.clone().unwrap().default_cancel_url,
          store_id:"".to_string(),//self.auth_token.clone().unwrap().token,
          amount,
          order_id,
          currency,
          customer_name,
          customer_address,
          customer_phone,
          customer_city,
          customer_post_code,
          client_ip:self.config.clone().unwrap().default_client_ip,
        };
        return sp_checkout;
    }

   
}


impl ShurjopayPlugin{

    /// This function can only be called once
    /// This function automatically authenticates if requires
    pub fn verifyPayment(&mut self, order_id: Option<String>)-> Option<SpVerifyResponse> {
        if let Some(_) = self.verify_auth_token()
        {
            if order_id.is_some()
            {
                return self.verify_payment_id(order_id.unwrap());
            }
            else 
            {
                println!("oder id not found");
                return None;                
            }
            
        }
        return None;
    }

    /// This function automatically authenticates and commits secure checkout
    /// It takes `SpCheckout` Struct as input
    pub fn MakePayment(&mut self, checkout_item: SpCheckout)->Option<String> {
        if let Some(_) = self.verify_auth_token()
        {
            let auth_token_val = self.auth_token.clone().unwrap();
            let checkout_mgs = SpCheckout{
                token: auth_token_val.token,
                store_id: auth_token_val.store_id.to_string(),
                ..checkout_item
            };
            return self.secure_ckeckout(checkout_mgs);
        }
        return None;
    }

    pub fn get_order_id(&self) -> Option<String>
    {
        if self.checkout_response.clone().is_some()
        {
            let order_id_test = self.checkout_response.clone().unwrap().sp_order_id;
            // println!("order id : {:#?}", order_id_test);
            return Some(order_id_test);
        }
        else 
        {
            println!("SP order ID not found");
            return None;
        }
    }

    
    /// This function is called to verify payments only once
    /// Further verification can be done by `checkPayments` function
    pub fn verify_payment_id(&mut self,order_id: String)-> Option<SpVerifyResponse> {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // Checking if client is valid or not
            if let Some(client) = sp_ins.client{
                // Constructing url, header and body
                let url = format!("{}{}/",spay.post_default_address, spay.verification_end_point);
                // let body = format!("{{\"order_id\": \"{}\"}}", self.checkout_response.clone().unwrap().sp_order_id);
                let body = format!("{{\"order_id\": \"{}\"}}",order_id);
                // let body = "{\"order_id\": \"sp636384e391650\"}";
                let header =format!{"{} {}", self.auth_token.clone().unwrap().token_type, self.auth_token.clone().unwrap().token };
                
                // Making HTTP request
                let response = client.post(url.as_str())
                                .header(CONTENT_TYPE, "application/json")
                                .header("Authorization", header)
                                .body(body)
                                .send();

                // Checking if respons is valid or not
                if let Some(responseData) = shurjopay_client::is_response_valid(response) {
  
                    // Mapping JSON string to structure

                    let modified_http_body =remove_first_and_last_ch(responseData.http_body.as_str());
                    // println!("modified http body: {:?}", modified_http_body);

                    let mut verify_json_option = SpVerifyResponse::new();
                    verify_json_option.string_to_json(modified_http_body);


            
                    if verify_json_option.string_to_json(modified_http_body) == true
                    {
                        // println!("verify json : {:#?}", verify_json_option);
                        return Some(verify_json_option);
                    }
                    else 
                    {
                        let mut verify_json_option2 = SpVerifyResponse2::new();
                        verify_json_option2.string_to_json(modified_http_body);

                        verify_json_option.convert_sp_response2_to_sp_respose(verify_json_option2);
                        
                        return Some(verify_json_option);

                    }
                
                }
                else {
                    println!("response is not valid");
                }
            }
        }
        return None;
    }

    
    /// This function sends a checkout structure to the Shurjopay server
    /// It returns `Option<checkout_url>` for the frontend
    pub fn secure_ckeckout(&mut self, checkout_item: SpCheckout)->Option<String> {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // println!("{:?}", spay);
            if let Some(client) = sp_ins.client{
                let url = format!("{}{}/",spay.post_default_address, spay.secure_payment_end_point);
                let body_json = serde_json::to_string(&checkout_item);
                let header =format!{"{} {}", self.auth_token.clone().unwrap().token_type, self.auth_token.clone().unwrap().token };
                
                // Making HTTP request
                let response = client.post(url.as_str())
                                .header(CONTENT_TYPE, "application/json")
                                .header("Authorization", header)
                                .body(body_json.unwrap())
                                .send();

                // Checking if respons is valid or not
                if let Some(responseData) = shurjopay_client::is_response_valid(response) {
                    // println!("Checkout Response: {:?}", responseData);
                    // Mapping JSON string to structure
                    let checkout_json_option: Option<SpCheckoutResponse> = unwrap_json(&responseData);
                    
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  checkout_json_option {
                        self.checkout_response = Some(valid_json_data.clone());
                        // println!("Checkout Response: {:?}", valid_json_data);
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


    /// This function gets auth token if no token is available
    /// or the existing token is expired
    /// This function return Option<auth_token_as_string>, if it successfully retrives a auth token
    pub fn verify_auth_token(&mut self) -> Option<String> 
    {
        // Check if the any previous auth token exist or not
        let token_struct =  self.auth_token.clone();
        // let token = 
        match token_struct {
            Some(auth_token) => {
                // Cheking token expiration validity
                if self.is_token_valid() {
                    return Some(auth_token.token);
                }else {
                    // If token not valid
                    self.auth_token = None;
                    return self.verify_auth_token();
                }
            }
            None => {
                let token_value = self.get_auth_token();
                return token_value
            },
        };
        // return None;
    }


    /// This function compares if the last received token is expires or not
    pub fn is_token_valid(&mut self) -> bool {
        // get local_time in unix timestamp
        let current_unix_time = Utc::now().timestamp()+21600;
        // println!("Current unix time: {:?}", current_unix_time);

        // Cenverting Datetime to unix timestamp
        let token_expires_at = self.token_expire_time.clone().unwrap().timestamp();
        // println!("Token Expire Time: {:?}", token_expires_at);

        // Coparing token expiration time with current time setting
        if current_unix_time <= token_expires_at  {
            return true;
        }
        return false;
    }

    /// This function gets auth token before initiating communication with `Shurjopay server`
    /// It returns `Option<auth_token>`
    pub fn get_auth_token(&mut self) -> Option<String> 
    {
        let sp_ins = self.clone();
        if let Some(spay) = sp_ins.config {
            // println!("{:?}", spay);
            if let Some(client) = sp_ins.client{
                let url = format!("{}{}/",spay.post_default_address, spay.token_end_point);

                let mut body = HashMap::new();
                body.insert("username", spay.sp_user);
                body.insert("password", spay.sp_pass);

                // Making HTTP request
                let response = client.post(url.as_str())
                                .header(CONTENT_TYPE, "application/json")
                                .json(&body)
                                .send();
                // Checking if respons is valid or not
                if let Some(responseData) = shurjopay_client::is_response_valid(response) 
                {
                    let auth_json_option: Option<SpAuthToken> = unwrap_json(&responseData);
                    // Checking JSON structure is matched or not
                    if let Some(valid_json_data) =  auth_json_option 
                    {
                        self.auth_token = Some(valid_json_data.clone());
                        self.set_expire_time();
                        return Some(valid_json_data.token);
                    } 
                    else 
                    {
                        // println!("{:?}", responseData);
                        let mut verify_json_option2 = SpVerifyResponse2::new();
                        if verify_json_option2.string_to_json(responseData.http_body.as_str()) == true
                        {
                            // println!("{:#?}", verify_json_option2.message.unwrap());
                            println!("Unauthorized Access: Check your username and password");
                        }
                        
                        return None;
                    }
                }
            } 
            else 
            {
                println!("Shurjopay http client is not set yet!");
            }            
        } 
        else 
        {
            println!("Shurjopay Configuration is not set yet!");
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
            } 
            else 
            {
                self.token_create_time = None;
            }
        }
    }  
}


/// implementation for SpAuthToken
impl SpAuthToken {
    /// Takes `SpAuthToken` structure and and converts `String` timestamp stored in `SpAuthToken.token_create_time` 
    /// return time in `NativeDateTime`
    pub fn get_time(&self) -> Option<NaiveDateTime> {
        // let custom = DateTime::parse_from_str("2022-11-02 05:26:19pm", "%d-%m-%Y %H:%M:%S%P")?;
        let time_t = self.token_create_time.clone();
        let mut time_offset:i64 = 0;
        let mut time_val: String = "".to_string();

        // Checking the time is in AM or PM
        // println!(" {:?}", time_t.find("pm"));
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

/// implementation for `SpVerifyRespose`
impl SpVerifyResponse 
{
    /// This is a constructor to initiate `null` instance of `SpVerifyResponse`
    /// returns `SpVerifyResponse`
    pub fn new() -> Self
    {
        SpVerifyResponse 
        { 
            id:  None, 
            order_id:  None, 
            currency:  None, 
            amount:  None, 
            payable_amount:  None, 
            discsount_amount:  None, 
            disc_percent:  None, 
            received_amount:  None, 
            usd_amt:  None, 
            usd_rate:  None, 
            card_holder_name:  None, 
            card_number:  None, 
            phone_no:  None, 
            bank_trx_id:  None, 
            invoice_no:  None, 
            bank_status:  None, 
            customer_order_id:  None, 
            sp_code:  None, 
            sp_message:  None, 
            name:  None, 
            email:  None, 
            address:  None, 
            city:  None, 
            value1:  None, 
            value2:  None, 
            value3:  None, 
            value4:  None, 
            transaction_status:  None, 
            method:  None, 
            date_time:  None,
        }
    }

    /// This function unwraps `JSON` `String` into `SpVerifyResponse` data structure'
    /// if conversion is possible return true or return false
    pub fn string_to_json(&mut self, msg: &str) ->bool
    { 
        // println!("msg = {}", msg); 
        let result:Result<SpVerifyResponse> = serde_json::from_str(msg);        
        match result {
            Ok(data) => 
            {
                *self = data.clone();
                // println!("sp_response: {:#?}", self);              
                return true;
            },
            Err(_) => 
            {
                // println!("error created");
                return false;
            },
        }
    }


    /// This function will convert `SpVerifyResponse2` data structure to `SoVerifyResponse`
    pub fn convert_sp_response2_to_sp_respose(&mut self, sp_response2: SpVerifyResponse2)
    {
        self.sp_code = Some(sp_response2.sp_code.unwrap().parse::<i64>().unwrap());
        self.sp_message = Some(sp_response2.message.unwrap().clone());
    }
}

/// implementation for SpVerifyResponse2
impl SpVerifyResponse2
{
    /// This is a constructor to initiate `null` instance of `SpVerifyResponse2`
    /// returns `SpVerifyResponse2`
    pub fn new() ->Self
    {
        SpVerifyResponse2 { sp_code: None, message: None }

    }

    /// This function unwraps `JSON` `String` into `SpVerifyResponse2` data structure'
    /// if conversion is possible return true
    pub fn string_to_json(&mut self, msg: &str) ->bool
    { 
    
        let result:Result<SpVerifyResponse2> = serde_json::from_str(msg);        
        // println!("\n\n\n result{:#?}", result);

        match result {
            Ok(data) => {
                *self = data.clone();
                // println!("sp_response2: {:#?}", self);
            return true;
        },
            Err(_) => 
            {
                println!("failed to convert spVerify Response2");
                return false;
            },
        }
    
        
    }
}



/// This function unwraps `JSON` `String` into specified `<T>` data structure
pub fn unwrap_json<'a, T>(response_data: &'a shurjopay_client::HttpResponse) -> Option<T> 
where T: Deserialize<'a>+ Clone+ std::fmt::Debug {
    if response_data.http_code == 200
    {
        let modified_http_body =remove_first_and_last_ch(response_data.http_body.as_str());
        // println!("modified http body: {:?}", modified_http_body);

        
        let json_data : Result<T> = serde_json::from_str(modified_http_body);
        match json_data {
            Ok(data) => {
                let data_clone = data.clone();
                // println!(" data: {:?}",data_clone);

            return Some(data)
        },
            Err(_) => 
            {
                // println!("error created");
                return None;
            },
        }
    } 
    else 
    {
        println!("response_data: {:?}", response_data);
    }
    return None
}




/// this function remove first and last char of string if first char is '[' and last char is ']'
/// Return modified string
pub fn remove_first_and_last_ch(input: &str) -> &str {

    let firstch = input.chars().nth(0).unwrap();
    let lastch = input.chars().last().unwrap();
    if firstch == '[' && lastch == ']'
    {
        let first_last_off: &str = &input[1..input.len() - 1];
        // println!("{}", first_last_off);
        return first_last_off;
    
    };
    
    return input;
}

/// This function will check if .env file available or not
/// return if available return true or return false
pub fn check_env_file_availble() -> bool
    {
        let check = dotenv().ok(); // This line loads the environment variables from the ".env" file.
        if check.is_none()
        {
            return false;
        }
        else
        {
            return true;
        }
    }
