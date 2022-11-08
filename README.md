# Rust Plugin for Shurjopay
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/reqwest.svg)](./LICENSE-APACHE)
This repository contains rust plugin for Shurjopay

The `Shurjopay` crate provides a convenient way to integrate Shurjopay payment gateway.

It handles many of the things that most people need to do manually
- Handles http request and request errors
- JSON serialization and deserialization
- Authentication during checkout and verification of payments

This library uses following crates which are included in `Cargo.toml` file

```toml
[dependencies]
chrono = "0.4"
reqwest = { version = "0.11", features = ["json","blocking"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4.0"
env_logger = "0.9.0"
open = "3.0.3"
```

## Example
Test code for main.rs

```rust,no_run
use open;
use std::io;

mod Shurjopay;  // comment this if you include Shurjopay from crate.io
use Shurjopay::ShurjopayPlugin;


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
```
## Requirements

Rust 1.64

On Linux:
- OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers (see https://github.com/sfackler/rust-openssl)

On Windows and macOS:
- Nothing.
Reqwest uses [rust-native-tls](https://github.com/sfackler/rust-native-tls),
which will use the operating system TLS framework if available, meaning Windows
and macOS. On Linux, it will use OpenSSL 1.1.


## For Open SSL Error in Ubuntu or WSL
Run the following command
```bash
apt install pkg-config
sudo apt install libssl-dev
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)