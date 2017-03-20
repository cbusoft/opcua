var _ = require("lodash");
var fs = require("fs");
var xml2js = require("xml2js");

var settings = require("./settings");

var types_xml = `${settings.schema_dir}/Opc.Ua.Types.bsd.xml`;

/// This code parses the OPC UA Binary types definitions and creates a generated types.rs file
/// Fields are converted to snake case as they are written. Code for serializing the struct is also generated

/// Any handwritten types are stripped from the output

var ignored_types =
    [
        "ExtensionObject", "DataValue", "LocalizedText", "QualifiedName", "DiagnosticInfo", "Variant",
        "ExpandedNodeId", "NodeId", "ByteStringNodeId", "GuidNodeId", "StringNodeId", "NumericNodeId",
        "FourByteNodeId", "TwoByteNodeId", "XmlElement", "Union", "RequestHeader", "ResponseHeader", "ExtensionObject",
    ];

function convertFieldName(name) {
    // Convert field name to snake case
    return _.snakeCase(name);
}

var parser = new xml2js.Parser();
fs.readFile(types_xml, function (err, data) {
    parser.parseString(data, function (err, result) {
        var data = {
            structured_types: []
        };

        var structured_types = result["opc:TypeDictionary"]["opc:StructuredType"];
        _.each(structured_types, function (structured_type_element) {

            var name = structured_type_element["$"]["Name"];
            // if name in ignored_types, do nothing
            if (!_.includes(ignored_types, name)) {
                var fields_to_add = [];
                var fields_to_hide = [];
                _.each(structured_type_element["opc:Field"], function (field) {
                    // Convert field name to snake case
                    var field_name = convertFieldName(field["$"]["Name"]);

                    // Strip namespace off the type
                    var type = field["$"]["TypeName"].split(":")[1];

                    // Replace String with UAString
                    if (type === "String") {
                        type = "UAString";
                    }

                    // Look for arrays
                    if (_.has(field["$"], "LengthField")) {
                        fields_to_add.push({
                            name: field_name,
                            type: `Option<Vec<${type}>>`,
                            inner_type: type,
                            is_array: true,
                        })
                        fields_to_hide.push(convertFieldName(field["$"]["LengthField"]));
                    }
                    else {
                        fields_to_add.push({
                            name: field_name,
                            type: type,
                        })
                    }
                })

                var structured_type = {
                    name: name,
                    fields_to_add: fields_to_add,
                    fields_to_hide: fields_to_hide,
                }
                if (_.has(structured_type_element, "opc:Documentation")) {
                    structured_type.documentation = structured_type_element["opc:Documentation"];
                }
                if (_.has(structured_type_element["$"], "BaseType")) {
                    structured_type.base_type = structured_type_element["$"]["BaseType"];
                }
                data.structured_types.push(structured_type)
            }

        });
        generate_types(data);
    });
});

function generate_types(data) {
    // Output structured types
    _.each(data.structured_types, function (structured_type) {
        generate_structured_type_file(structured_type);
    });
}

function generate_structured_type_file(structured_type) {
    var file_name = _.snakeCase(structured_type.name) + ".rs";
    var file_path = `${settings.rs_dir}/types/generated/${file_name}`

    console.log("Creating structured type file - " + file_path);

    var contents = `// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

`;
    if (_.has(structured_type, "documentation")) {
        contents += `/// ${structured_type.documentation}\n`;
    }
    contents += `#[derive(Debug, Clone, PartialEq)]
pub struct ${structured_type.name} {
`;
    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            contents += `    pub ${field.name}: ${field.type},\n`;
        }
    });
    contents += `}

`;

    if (_.has(structured_type, "base_type") && structured_type.base_type === "ua:ExtensionObject") {
        contents += `impl MessageInfo for ${structured_type.name} {
    fn object_id(&self) -> ObjectId {
        ObjectId::${structured_type.name}_Encoding_DefaultBinary
    }
}

`;
    }

    contents += `impl BinaryEncoder<${structured_type.name}> for ${structured_type.name} {
    fn byte_len(&self) -> usize {
`;
    if (structured_type.fields_to_add.length > 0) {
        contents += `        let mut size = 0;\n`;

        _.each(structured_type.fields_to_add, function (field) {
            if (!_.includes(structured_type.fields_to_hide, field.name)) {
                if (_.has(field, 'is_array')) {
                    contents += `        size += byte_len_array(&self.${field.name});\n`;
                }
                else {
                    contents += `        size += self.${field.name}.byte_len();\n`;
                }
            }
        });

        contents += `        size\n`;
    }
    else {
        contents += `        0\n`;
    }

    contents += `    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
`;

    if (structured_type.fields_to_add.length > 0) {
        contents += `        let mut size = 0;\n`;

        _.each(structured_type.fields_to_add, function (field) {
            if (!_.includes(structured_type.fields_to_hide, field.name)) {
                if (_.has(field, 'is_array')) {
                    contents += `        size += write_array(stream, &self.${field.name})?;\n`;
                }
                else {
                    contents += `        size += self.${field.name}.encode(stream)?;\n`;
                }
            }
        });

        contents += `        Ok(size)\n`;
    }
    else {
        contents += `        Ok(0)\n`;
    }

    contents += `    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
`;

    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            if (_.has(field, 'is_array')) {
                contents += `        let ${field.name}: ${field.type} = read_array(stream)?;\n`;
            }
            else {
                contents += `        let ${field.name} = ${field.type}::decode(stream)?;\n`;
            }
        }
    });

    contents += `        Ok(${structured_type.name} {
`;

    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            contents += `            ${field.name}: ${field.name},\n`;
        }
    });

    contents += `        })
    }
}
`;

    var buffer = new Buffer(contents);
    var fd = fs.openSync(file_path, 'w');
    fs.writeSync(fd, buffer, 0, buffer.length, null);
    fs.closeSync(fd);
}