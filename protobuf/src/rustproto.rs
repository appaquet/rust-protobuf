// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `rustproto.proto`

/// Extension fields
pub mod exts {

    pub const expose_oneof_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17035, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:N\n\x13generat\
    e_getter_all\x18\xed\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOp\
    tionsR\x11generateGetterAll:b\n\x1ecarllerche_bytes_for_bytes_all\x18\
    \xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1acarlle\
    rcheBytesForBytesAll:d\n\x1fcarllerche_bytes_for_string_all\x18\xf4\x84\
    \x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1bcarllercheByte\
    sForStringAll:`\n\x1dsingular_field_option_box_all\x18\x80\x85\x01\x20\
    \x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x19singularFieldOptionBox\
    All:Y\n\x19singular_field_option_all\x18\x81\x85\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x16singularFieldOptionAll:H\n\x10serde\
    _derive_all\x18\x86\x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOpt\
    ionsR\x0eserdeDeriveAll:O\n\x14serde_derive_cfg_all\x18\x87\x85\x01\x20\
    \x01(\t\x12\x1c.google.protobuf.FileOptionsR\x11serdeDeriveCfgAll:H\n\
    \x10lite_runtime_all\x18\x8b\x85\x01\x20\x01(\x08\x12\x1c.google.protobu\
    f.FileOptionsR\x0eliteRuntimeAll:D\n\x0cexpose_oneof\x18\xe9\x84\x01\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bexposeOneof:F\n\rex\
    pose_fields\x18\xeb\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.Message\
    OptionsR\x0cexposeFields:P\n\x12generate_accessors\x18\xec\x84\x01\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x11generateAccessors:J\
    \n\x0fgenerate_getter\x18\xed\x84\x01\x20\x01(\x08\x12\x1f.google.protob\
    uf.MessageOptionsR\x0egenerateGetter:^\n\x1acarllerche_bytes_for_bytes\
    \x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes_for_string\x18\xf4\
    \x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x18carller\
    cheBytesForString:\\\n\x19singular_field_option_box\x18\x80\x85\x01\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x16singularFieldOption\
    Box:U\n\x15singular_field_option\x18\x81\x85\x01\x20\x01(\x08\x12\x1f.go\
    ogle.protobuf.MessageOptionsR\x13singularFieldOption:D\n\x0cserde_derive\
    \x18\x86\x85\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x0bserdeDerive:K\n\x10serde_derive_cfg\x18\x87\x85\x01\x20\x01(\t\x12\
    \x1f.google.protobuf.MessageOptionsR\x0eserdeDeriveCfg:O\n\x13expose_fie\
    lds_field\x18\xeb\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOpti\
    onsR\x11exposeFieldsField:Y\n\x18generate_accessors_field\x18\xec\x84\
    \x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x16generateAcces\
    sorsField:S\n\x15generate_getter_field\x18\xed\x84\x01\x20\x01(\x08\x12\
    \x1d.google.protobuf.FieldOptionsR\x13generateGetterField:g\n\x20carller\
    che_bytes_for_bytes_field\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.pr\
    otobuf.FieldOptionsR\x1ccarllercheBytesForBytesField:i\n!carllerche_byte\
    s_for_string_field\x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.\
    FieldOptionsR\x1dcarllercheBytesForStringField:e\n\x1fsingular_field_opt\
    ion_box_field\x18\x80\x85\x01\x20\x01(\x08\x12\x1d.google.protobuf.Field\
    OptionsR\x1bsingularFieldOptionBoxField:^\n\x1bsingular_field_option_fie\
    ld\x18\x81\x85\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\
    \x18singularFieldOptionFieldJ\xa5\x20\n\x06\x12\x04\0\0P\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0*\n\xe5\x01\n\x01\x02\
    \x12\x03\n\0\x122^\x20see\x20https://github.com/gogo/protobuf/blob/maste\
    r/gogoproto/gogo.proto\n\x20for\x20the\x20original\x20idea\n2{\x20Genera\
    ted\x20files\x20can\x20be\x20customized\x20using\x20this\x20proto\n\x20o\
    r\x20using\x20`Customize`\x20struct\x20when\x20codegen\x20is\x20invoked\
    \x20programmatically.\n\n\t\n\x01\x07\x12\x04\x0c\0&\x01\n7\n\x02\x07\0\
    \x12\x03\x0e\x04+\x1a,\x20When\x20true,\x20oneof\x20field\x20is\x20gener\
    ated\x20public\n\n\n\n\x03\x07\0\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\0\
    \x04\x12\x03\x0e\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0e\r\x11\n\n\n\x03\
    \x07\0\x01\x12\x03\x0e\x12\"\n\n\n\x03\x07\0\x03\x12\x03\x0e%*\nI\n\x02\
    \x07\x01\x12\x03\x10\x04,\x1a>\x20When\x20true\x20all\x20fields\x20are\
    \x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\x01\
    \x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\x10\x04\x0c\n\n\n\
    \x03\x07\x01\x05\x12\x03\x10\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\x10\x12\
    #\n\n\n\x03\x07\x01\x03\x12\x03\x10&+\nP\n\x02\x07\x02\x12\x03\x12\x041\
    \x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acces\
    sors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\x0c\x07\
    \"\n\n\n\x03\x07\x02\x04\x12\x03\x12\x04\x0c\n\n\n\x03\x07\x02\x05\x12\
    \x03\x12\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x12\x12(\n\n\n\x03\x07\x02\
    \x03\x12\x03\x12+0\nL\n\x02\x07\x03\x12\x03\x14\x04.\x1aA\x20When\x20fal\
    se,\x20`get_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\
    \x20\"proto2\"`\n\n\n\n\x03\x07\x03\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\
    \x03\x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\
    \n\x03\x07\x03\x01\x12\x03\x14\x12%\n\n\n\x03\x07\x03\x03\x12\x03\x14(-\
    \n2\n\x02\x07\x04\x12\x03\x16\x049\x1a'\x20Use\x20`bytes::Bytes`\x20for\
    \x20`bytes`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\
    \x03\x07\x04\x04\x12\x03\x16\x04\x0c\n\n\n\x03\x07\x04\x05\x12\x03\x16\r\
    \x11\n\n\n\x03\x07\x04\x01\x12\x03\x16\x120\n\n\n\x03\x07\x04\x03\x12\
    \x03\x1638\n3\n\x02\x07\x05\x12\x03\x18\x04:\x1a(\x20Use\x20`bytes::Byte\
    s`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x05\x02\x12\x03\x0c\x07\
    \"\n\n\n\x03\x07\x05\x04\x12\x03\x18\x04\x0c\n\n\n\x03\x07\x05\x05\x12\
    \x03\x18\r\x11\n\n\n\x03\x07\x05\x01\x12\x03\x18\x121\n\n\n\x03\x07\x05\
    \x03\x12\x03\x1849\nM\n\x02\x07\x06\x12\x03\x1a\x048\x1aB\x20Use\x20`std\
    ::Option<std::Box<T>>`\x20to\x20store\x20singular\x20messages\x20fields\
    \n\n\n\n\x03\x07\x06\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x06\x04\x12\x03\
    \x1a\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x1a\r\x11\n\n\n\x03\x07\x06\
    \x01\x12\x03\x1a\x12/\n\n\n\x03\x07\x06\x03\x12\x03\x1a27\n\x93\x01\n\
    \x02\x07\x07\x12\x03\x1d\x044\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20\
    to\x20store\x20singular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\
    \x20possible\x20to\x20have\x20recursive\x20messages\x20with\x20this\x20o\
    ption\x20enabled.\n\n\n\n\x03\x07\x07\x02\x12\x03\x0c\x07\"\n\n\n\x03\
    \x07\x07\x04\x12\x03\x1d\x04\x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1d\r\x11\
    \n\n\n\x03\x07\x07\x01\x12\x03\x1d\x12+\n\n\n\x03\x07\x07\x03\x12\x03\
    \x1d.3\nJ\n\x02\x07\x08\x12\x03\x20\x04+\x1a?\x20Use\x20`serde_derive`\
    \x20to\x20implement\x20`Serialize`\x20and\x20`Deserialize`\n\n\n\n\x03\
    \x07\x08\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x08\x04\x12\x03\x20\x04\x0c\
    \n\n\n\x03\x07\x08\x05\x12\x03\x20\r\x11\n\n\n\x03\x07\x08\x01\x12\x03\
    \x20\x12\"\n\n\n\x03\x07\x08\x03\x12\x03\x20%*\n3\n\x02\x07\t\x12\x03\"\
    \x041\x1a(\x20Guard\x20serde\x20annotations\x20with\x20cfg\x20attr.\n\n\
    \n\n\x03\x07\t\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\t\x04\x12\x03\"\x04\
    \x0c\n\n\n\x03\x07\t\x05\x12\x03\"\r\x13\n\n\n\x03\x07\t\x01\x12\x03\"\
    \x14(\n\n\n\x03\x07\t\x03\x12\x03\"+0\nN\n\x02\x07\n\x12\x03%\x04+\x1aC\
    \x20When\x20true,\x20will\x20only\x20generate\x20codes\x20that\x20works\
    \x20with\x20lite\x20runtime.\n\n\n\n\x03\x07\n\x02\x12\x03\x0c\x07\"\n\n\
    \n\x03\x07\n\x04\x12\x03%\x04\x0c\n\n\n\x03\x07\n\x05\x12\x03%\r\x11\n\n\
    \n\x03\x07\n\x01\x12\x03%\x12\"\n\n\n\x03\x07\n\x03\x12\x03%%*\n\t\n\x01\
    \x07\x12\x04(\0>\x01\n7\n\x02\x07\x0b\x12\x03*\x04'\x1a,\x20When\x20true\
    ,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\x0b\x02\
    \x12\x03(\x07%\n\n\n\x03\x07\x0b\x04\x12\x03*\x04\x0c\n\n\n\x03\x07\x0b\
    \x05\x12\x03*\r\x11\n\n\n\x03\x07\x0b\x01\x12\x03*\x12\x1e\n\n\n\x03\x07\
    \x0b\x03\x12\x03*!&\nI\n\x02\x07\x0c\x12\x03,\x04(\x1a>\x20When\x20true\
    \x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20genera\
    ted\n\n\n\n\x03\x07\x0c\x02\x12\x03(\x07%\n\n\n\x03\x07\x0c\x04\x12\x03,\
    \x04\x0c\n\n\n\x03\x07\x0c\x05\x12\x03,\r\x11\n\n\n\x03\x07\x0c\x01\x12\
    \x03,\x12\x1f\n\n\n\x03\x07\x0c\x03\x12\x03,\"'\nP\n\x02\x07\r\x12\x03.\
    \x04-\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20\
    accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\r\x02\x12\x03(\x07%\
    \n\n\n\x03\x07\r\x04\x12\x03.\x04\x0c\n\n\n\x03\x07\r\x05\x12\x03.\r\x11\
    \n\n\n\x03\x07\r\x01\x12\x03.\x12$\n\n\n\x03\x07\r\x03\x12\x03.',\nL\n\
    \x02\x07\x0e\x12\x030\x04*\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\
    \x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\
    \x07\x0e\x02\x12\x03(\x07%\n\n\n\x03\x07\x0e\x04\x12\x030\x04\x0c\n\n\n\
    \x03\x07\x0e\x05\x12\x030\r\x11\n\n\n\x03\x07\x0e\x01\x12\x030\x12!\n\n\
    \n\x03\x07\x0e\x03\x12\x030$)\n2\n\x02\x07\x0f\x12\x032\x045\x1a'\x20Use\
    \x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x0f\x02\
    \x12\x03(\x07%\n\n\n\x03\x07\x0f\x04\x12\x032\x04\x0c\n\n\n\x03\x07\x0f\
    \x05\x12\x032\r\x11\n\n\n\x03\x07\x0f\x01\x12\x032\x12,\n\n\n\x03\x07\
    \x0f\x03\x12\x032/4\n3\n\x02\x07\x10\x12\x034\x046\x1a(\x20Use\x20`bytes\
    ::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x10\x02\x12\x03(\
    \x07%\n\n\n\x03\x07\x10\x04\x12\x034\x04\x0c\n\n\n\x03\x07\x10\x05\x12\
    \x034\r\x11\n\n\n\x03\x07\x10\x01\x12\x034\x12-\n\n\n\x03\x07\x10\x03\
    \x12\x03405\nM\n\x02\x07\x11\x12\x036\x044\x1aB\x20Use\x20`std::Option<s\
    td::Box<T>>`\x20to\x20store\x20singular\x20messages\x20fields\n\n\n\n\
    \x03\x07\x11\x02\x12\x03(\x07%\n\n\n\x03\x07\x11\x04\x12\x036\x04\x0c\n\
    \n\n\x03\x07\x11\x05\x12\x036\r\x11\n\n\n\x03\x07\x11\x01\x12\x036\x12+\
    \n\n\n\x03\x07\x11\x03\x12\x036.3\n\x93\x01\n\x02\x07\x12\x12\x039\x040\
    \x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20store\x20singular\x20me\
    ssages\x20fields.\n\x20Note,\x20it's\x20not\x20possible\x20to\x20have\
    \x20recursive\x20messages\x20with\x20this\x20option\x20enabled.\n\n\n\n\
    \x03\x07\x12\x02\x12\x03(\x07%\n\n\n\x03\x07\x12\x04\x12\x039\x04\x0c\n\
    \n\n\x03\x07\x12\x05\x12\x039\r\x11\n\n\n\x03\x07\x12\x01\x12\x039\x12'\
    \n\n\n\x03\x07\x12\x03\x12\x039*/\nJ\n\x02\x07\x13\x12\x03;\x04'\x1a?\
    \x20Use\x20`serde_derive`\x20to\x20implement\x20`Serialize`\x20and\x20`D\
    eserialize`\n\n\n\n\x03\x07\x13\x02\x12\x03(\x07%\n\n\n\x03\x07\x13\x04\
    \x12\x03;\x04\x0c\n\n\n\x03\x07\x13\x05\x12\x03;\r\x11\n\n\n\x03\x07\x13\
    \x01\x12\x03;\x12\x1e\n\n\n\x03\x07\x13\x03\x12\x03;!&\n3\n\x02\x07\x14\
    \x12\x03=\x04-\x1a(\x20Guard\x20serde\x20annotations\x20with\x20cfg\x20a\
    ttr.\n\n\n\n\x03\x07\x14\x02\x12\x03(\x07%\n\n\n\x03\x07\x14\x04\x12\x03\
    =\x04\x0c\n\n\n\x03\x07\x14\x05\x12\x03=\r\x13\n\n\n\x03\x07\x14\x01\x12\
    \x03=\x14$\n\n\n\x03\x07\x14\x03\x12\x03=',\n\t\n\x01\x07\x12\x04@\0P\
    \x01\nI\n\x02\x07\x15\x12\x03B\x04.\x1a>\x20When\x20true\x20all\x20field\
    s\x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\
    \x07\x15\x02\x12\x03@\x07#\n\n\n\x03\x07\x15\x04\x12\x03B\x04\x0c\n\n\n\
    \x03\x07\x15\x05\x12\x03B\r\x11\n\n\n\x03\x07\x15\x01\x12\x03B\x12%\n\n\
    \n\x03\x07\x15\x03\x12\x03B(-\nP\n\x02\x07\x16\x12\x03D\x043\x1aE\x20Whe\
    n\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\
    \x20not\x20generated\n\n\n\n\x03\x07\x16\x02\x12\x03@\x07#\n\n\n\x03\x07\
    \x16\x04\x12\x03D\x04\x0c\n\n\n\x03\x07\x16\x05\x12\x03D\r\x11\n\n\n\x03\
    \x07\x16\x01\x12\x03D\x12*\n\n\n\x03\x07\x16\x03\x12\x03D-2\nL\n\x02\x07\
    \x17\x12\x03F\x040\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20gene\
    rated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\x17\
    \x02\x12\x03@\x07#\n\n\n\x03\x07\x17\x04\x12\x03F\x04\x0c\n\n\n\x03\x07\
    \x17\x05\x12\x03F\r\x11\n\n\n\x03\x07\x17\x01\x12\x03F\x12'\n\n\n\x03\
    \x07\x17\x03\x12\x03F*/\n2\n\x02\x07\x18\x12\x03H\x04;\x1a'\x20Use\x20`b\
    ytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x18\x02\x12\x03\
    @\x07#\n\n\n\x03\x07\x18\x04\x12\x03H\x04\x0c\n\n\n\x03\x07\x18\x05\x12\
    \x03H\r\x11\n\n\n\x03\x07\x18\x01\x12\x03H\x122\n\n\n\x03\x07\x18\x03\
    \x12\x03H5:\n3\n\x02\x07\x19\x12\x03J\x04<\x1a(\x20Use\x20`bytes::Bytes`\
    \x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x19\x02\x12\x03@\x07#\n\n\
    \n\x03\x07\x19\x04\x12\x03J\x04\x0c\n\n\n\x03\x07\x19\x05\x12\x03J\r\x11\
    \n\n\n\x03\x07\x19\x01\x12\x03J\x123\n\n\n\x03\x07\x19\x03\x12\x03J6;\nM\
    \n\x02\x07\x1a\x12\x03L\x04:\x1aB\x20Use\x20`std::Option<std::Box<T>>`\
    \x20to\x20store\x20singular\x20messages\x20fields\n\n\n\n\x03\x07\x1a\
    \x02\x12\x03@\x07#\n\n\n\x03\x07\x1a\x04\x12\x03L\x04\x0c\n\n\n\x03\x07\
    \x1a\x05\x12\x03L\r\x11\n\n\n\x03\x07\x1a\x01\x12\x03L\x121\n\n\n\x03\
    \x07\x1a\x03\x12\x03L49\n\x93\x01\n\x02\x07\x1b\x12\x03O\x046\x1a\x87\
    \x01\x20Use\x20`std::Option<T>`\x20to\x20store\x20singular\x20messages\
    \x20fields.\n\x20Note,\x20it's\x20not\x20possible\x20to\x20have\x20recur\
    sive\x20messages\x20with\x20this\x20option\x20enabled.\n\n\n\n\x03\x07\
    \x1b\x02\x12\x03@\x07#\n\n\n\x03\x07\x1b\x04\x12\x03O\x04\x0c\n\n\n\x03\
    \x07\x1b\x05\x12\x03O\r\x11\n\n\n\x03\x07\x1b\x01\x12\x03O\x12-\n\n\n\
    \x03\x07\x1b\x03\x12\x03O05\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::LazyV2<crate::reflect::FileDescriptor> = crate::rt::LazyV2::INIT;
    file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        deps.push(crate::descriptor::file_descriptor());
        let mut messages = ::std::vec::Vec::new();
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::FileDescriptor::new(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    })
}
