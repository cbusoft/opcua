// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildInfo {
    pub product_uri: UAString,
    pub manufacturer_name: UAString,
    pub product_name: UAString,
    pub software_version: UAString,
    pub build_number: UAString,
    pub build_date: DateTime,
}

impl MessageInfo for BuildInfo {
    fn object_id(&self) -> ObjectId {
        ObjectId::BuildInfo_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BuildInfo> for BuildInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.product_uri.byte_len();
        size += self.manufacturer_name.byte_len();
        size += self.product_name.byte_len();
        size += self.software_version.byte_len();
        size += self.build_number.byte_len();
        size += self.build_date.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.product_uri.encode(stream)?;
        size += self.manufacturer_name.encode(stream)?;
        size += self.product_name.encode(stream)?;
        size += self.software_version.encode(stream)?;
        size += self.build_number.encode(stream)?;
        size += self.build_date.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let product_uri = UAString::decode(stream)?;
        let manufacturer_name = UAString::decode(stream)?;
        let product_name = UAString::decode(stream)?;
        let software_version = UAString::decode(stream)?;
        let build_number = UAString::decode(stream)?;
        let build_date = DateTime::decode(stream)?;
        Ok(BuildInfo {
            product_uri: product_uri,
            manufacturer_name: manufacturer_name,
            product_name: product_name,
            software_version: software_version,
            build_number: build_number,
            build_date: build_date,
        })
    }
}
