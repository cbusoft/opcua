// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ServerStatusDataType {
    pub start_time: DateTime,
    pub current_time: DateTime,
    pub state: ServerState,
    pub build_info: BuildInfo,
    pub seconds_till_shutdown: UInt32,
    pub shutdown_reason: LocalizedText,
}

impl MessageInfo for ServerStatusDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ServerStatusDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ServerStatusDataType> for ServerStatusDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.start_time.byte_len();
        size += self.current_time.byte_len();
        size += self.state.byte_len();
        size += self.build_info.byte_len();
        size += self.seconds_till_shutdown.byte_len();
        size += self.shutdown_reason.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.start_time.encode(stream)?;
        size += self.current_time.encode(stream)?;
        size += self.state.encode(stream)?;
        size += self.build_info.encode(stream)?;
        size += self.seconds_till_shutdown.encode(stream)?;
        size += self.shutdown_reason.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let start_time = DateTime::decode(stream)?;
        let current_time = DateTime::decode(stream)?;
        let state = ServerState::decode(stream)?;
        let build_info = BuildInfo::decode(stream)?;
        let seconds_till_shutdown = UInt32::decode(stream)?;
        let shutdown_reason = LocalizedText::decode(stream)?;
        Ok(ServerStatusDataType {
            start_time: start_time,
            current_time: current_time,
            state: state,
            build_info: build_info,
            seconds_till_shutdown: seconds_till_shutdown,
            shutdown_reason: shutdown_reason,
        })
    }
}
