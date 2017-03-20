// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AxisInformation {
    pub engineering_units: EUInformation,
    pub eu_range: Range,
    pub title: LocalizedText,
    pub axis_scale_type: AxisScaleEnumeration,
    pub axis_steps: Option<Vec<Double>>,
}

impl MessageInfo for AxisInformation {
    fn object_id(&self) -> ObjectId {
        ObjectId::AxisInformation_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AxisInformation> for AxisInformation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.engineering_units.byte_len();
        size += self.eu_range.byte_len();
        size += self.title.byte_len();
        size += self.axis_scale_type.byte_len();
        size += byte_len_array(&self.axis_steps);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.engineering_units.encode(stream)?;
        size += self.eu_range.encode(stream)?;
        size += self.title.encode(stream)?;
        size += self.axis_scale_type.encode(stream)?;
        size += write_array(stream, &self.axis_steps)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let engineering_units = EUInformation::decode(stream)?;
        let eu_range = Range::decode(stream)?;
        let title = LocalizedText::decode(stream)?;
        let axis_scale_type = AxisScaleEnumeration::decode(stream)?;
        let axis_steps: Option<Vec<Double>> = read_array(stream)?;
        Ok(AxisInformation {
            engineering_units: engineering_units,
            eu_range: eu_range,
            title: title,
            axis_scale_type: axis_scale_type,
            axis_steps: axis_steps,
        })
    }
}
