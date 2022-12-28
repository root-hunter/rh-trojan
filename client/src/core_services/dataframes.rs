use websocket::dataframe::{DataFrame, Opcode};

pub struct ChunkDetail{
    chunk_id: u8,
    tx_id: [u8; 36],
    service: [u8; 256]
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

pub fn create_dataframe(tx_id_string: &String, service: &String, i: u8, mut data: Vec<u8>) -> DataFrame {
    let chunk_detail = set_chunk_details(tx_id_string.as_str(), service.as_str(), i);
    let bytes: &[u8] = unsafe { any_as_u8_slice(&chunk_detail) };
    
    println!("DATI CHUNK: {:?}", bytes);

    let vect = &mut Vec::<u8>::from(bytes);
    data.append(vect);
    
    let dataframe = DataFrame::new(true, Opcode::Binary, data);
    dataframe
}

pub fn set_chunk_details(tx_id: &str, service: &str, index: u8) -> ChunkDetail {
    let mut chunk_detail = ChunkDetail {
        chunk_id: index,
        tx_id: [0; 36],
        service: [0; 256]
    };
    
    let service = service.as_bytes();
    let tx_id = tx_id.as_bytes();
  
    let mut k = 0;
    for x in tx_id{
        chunk_detail.tx_id[k] = *x;
        k += 1;
    }
    let mut k = 0;
    for x in service{
        chunk_detail.service[k] = *x;
        k += 1;
    }
    chunk_detail
}
