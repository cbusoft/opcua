// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteMonitoredItemsRequest {
    pub request_header: RequestHeader,
    pub subscription_id: UInt32,
    pub monitored_item_ids: Option<Vec<UInt32>>,
}

impl MessageInfo for DeleteMonitoredItemsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteMonitoredItemsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteMonitoredItemsRequest> for DeleteMonitoredItemsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += byte_len_array(&self.monitored_item_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += write_array(stream, &self.monitored_item_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let monitored_item_ids: Option<Vec<UInt32>> = read_array(stream)?;
        Ok(DeleteMonitoredItemsRequest {
            request_header: request_header,
            subscription_id: subscription_id,
            monitored_item_ids: monitored_item_ids,
        })
    }
}
