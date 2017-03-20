// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TransferResult {
    pub status_code: StatusCode,
    pub available_sequence_numbers: Option<Vec<UInt32>>,
}

impl MessageInfo for TransferResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::TransferResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TransferResult> for TransferResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.available_sequence_numbers);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.available_sequence_numbers)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let available_sequence_numbers: Option<Vec<UInt32>> = read_array(stream)?;
        Ok(TransferResult {
            status_code: status_code,
            available_sequence_numbers: available_sequence_numbers,
        })
    }
}
