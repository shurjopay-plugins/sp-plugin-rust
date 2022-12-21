//!
//! This module is designed to do http communication in the integration of
//! Shurjopay payment gateway service.
//! 
//! This module:
//! - Automatic handles html errors
//! - Authenticates automatically during makePayments or verifyingPayments
//! 

/// The `log` crate is included to export log for debug purpose
extern crate log;
// use log::{debug, error, info, warn};
use log::info;

/// The `reqwest` crate is included to make http request
extern crate reqwest;
// use reqwest::blocking::Client;
// use reqwest::{Error, Response};
// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};

#[derive(Debug)]
pub struct HttpResponse{
    pub http_code: u16,
    pub http_body: String,
}



pub fn is_response_valid(res: Result<reqwest::blocking::Response, reqwest::Error>) -> Option<HttpResponse>
{

    match res {
        Ok(_) => {
            info!("{:?}", "URL is valid");
            let status = res.unwrap();
            let status_code = status.status();
            // println!("{:?}", status_code);
            let body = status.text();
            // println!("{:?}", body);

            match body {
                Ok(mgs_body) => {
                    // mgs_body.what_is_it();
                    let resbody = HttpResponse{
                        http_code: status_code.as_u16(),
                        http_body: mgs_body,
                    };
                    // print!("resbody_string: ");
                    // println!("{:?}",resbody);
                    return Some(resbody);
                },
                Err(mgs_body2) => {
                    let resbody = HttpResponse{
                        http_code: status_code.as_u16(),
                        http_body: mgs_body2.to_string(),
                    };
                    return Some(resbody);
                },
            }
        },
        Err(_error) =>   {
            return None;
        },
    };

}