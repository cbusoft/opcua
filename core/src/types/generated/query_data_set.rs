// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDataSet {
    pub node_id: ExpandedNodeId,
    pub type_definition_node: ExpandedNodeId,
    pub values: Option<Vec<Variant>>,
}

impl MessageInfo for QueryDataSet {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryDataSet_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryDataSet> for QueryDataSet {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.type_definition_node.byte_len();
        size += byte_len_array(&self.values);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.type_definition_node.encode(stream)?;
        size += write_array(stream, &self.values)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = ExpandedNodeId::decode(stream)?;
        let type_definition_node = ExpandedNodeId::decode(stream)?;
        let values: Option<Vec<Variant>> = read_array(stream)?;
        Ok(QueryDataSet {
            node_id: node_id,
            type_definition_node: type_definition_node,
            values: values,
        })
    }
}
