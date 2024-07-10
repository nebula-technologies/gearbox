// ContentType::Json5
pub(crate) const EXAMPLE_JSON5_SERIALIZE: &str = r#"{"unquoted":"and you can quote me on that","singleQuotes":"I can use \"double quotes\" here","lineBreaks":"Look, Mom! No \\n's!","hexadecimal":912559,"leadingDecimalPoint":0.8675309,"andTrailing":8675309,"positiveSign":1,"trailingComma":"in objects","andIn":["arrays","arrays-2"],"backwardsCompatible":"with JSON"}"#;
pub(crate) const EXAMPLE_JSON5_DESERIALIZE: &str = r#"
{
  // comments
  unquoted: 'and you can quote me on that',
  singleQuotes: 'I can use "double quotes" here',
  lineBreaks: "Look, Mom! \
No \\n's!",
  hexadecimal: 0xdecaf,
  leadingDecimalPoint: .8675309, andTrailing: 8675309.,
  positiveSign: +1,
  trailingComma: 'in objects', andIn: ['arrays','arrays-2',],
  "backwardsCompatible": "with JSON",
}"#;

// ContentType::Json
pub(crate) const EXAMPLE_JSON_SERIALIZE: &str = r#"{"unquoted":"and you can quote me on that","singleQuotes":"I can use \"double quotes\" here","lineBreaks":"Look, Mom! No \\n's!","hexadecimal":912559,"leadingDecimalPoint":0.8675309,"andTrailing":8675309.0,"positiveSign":1,"trailingComma":"in objects","andIn":["arrays","arrays-2"],"backwardsCompatible":"with JSON"}"#;
pub(crate) const EXAMPLE_JSON_DESERIALIZE: &str = r#"{
"unquoted": "and you can quote me on that",
"singleQuotes": "I can use \"double quotes\" here",
"lineBreaks": "Look, Mom! No \\n's!",
"hexadecimal": 912559,
"leadingDecimalPoint": 0.8675309,
"andTrailing": 8675309.0,
"positiveSign": 1,
"trailingComma": "in objects",
"andIn": ["arrays", "arrays-2"],
"backwardsCompatible": "with JSON" }
"#;

// ContentType::Yaml
pub(crate) const EXAMPLE_YAML_SERIALIZE: &str = r#"unquoted: and you can quote me on that
singleQuotes: I can use "double quotes" here
lineBreaks: Look, Mom! No \n's!
hexadecimal: 912559
leadingDecimalPoint: 0.8675309
andTrailing: 8675309.0
positiveSign: 1
trailingComma: in objects
andIn:
- arrays
- arrays-2
backwardsCompatible: with JSON
"#;

pub(crate) const EXAMPLE_YAML_DESERIALIZE: &str = r#"unquoted: and you can quote me on that
singleQuotes: I can use "double quotes" here
lineBreaks: Look, Mom! No \n's!
hexadecimal: 912559
leadingDecimalPoint: 0.8675309
andTrailing: 8675309.0
positiveSign: 1
trailingComma: in objects
andIn:
- arrays
- arrays-2
backwardsCompatible: with JSON"#;

// ContentType::Xml
pub(crate) const XML_SERIALIZE: &str = r#"<?xml version="1.0" encoding="UTF-8"?><MyStruct><unquoted>and you can quote me on that</unquoted><singleQuotes>I can use "double quotes" here</singleQuotes><lineBreaks>Look, Mom! No \n's!</lineBreaks><hexadecimal>912559</hexadecimal><leadingDecimalPoint>0.8675309</leadingDecimalPoint><andTrailing>8675309</andTrailing><positiveSign>1</positiveSign><trailingComma>in objects</trailingComma><andIn>arrays</andIn><andIn>arrays-2</andIn><backwardsCompatible>with JSON</backwardsCompatible></MyStruct>"#;
pub(crate) const XML_DESERIALIZE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<MyStruct>
    <unquoted>and you can quote me on that</unquoted>
    <singleQuotes>I can use "double quotes" here</singleQuotes>
    <lineBreaks>Look, Mom! No \n's!</lineBreaks>
    <hexadecimal>912559</hexadecimal>
    <leadingDecimalPoint>0.8675309</leadingDecimalPoint>
    <andTrailing>8675309</andTrailing>
    <positiveSign>1</positiveSign>
    <trailingComma>in objects</trailingComma>
    <andIn>arrays</andIn>
    <andIn>arrays-2</andIn>
    <backwardsCompatible>with JSON</backwardsCompatible>
</MyStruct>"#;

// ContentType::Bson
pub(crate) const BSON_SERIALIZE: &[u8] = &[
    69, 1, 0, 0, 2, 117, 110, 113, 117, 111, 116, 101, 100, 0, 29, 0, 0, 0, 97, 110, 100, 32, 121,
    111, 117, 32, 99, 97, 110, 32, 113, 117, 111, 116, 101, 32, 109, 101, 32, 111, 110, 32, 116,
    104, 97, 116, 0, 2, 115, 105, 110, 103, 108, 101, 81, 117, 111, 116, 101, 115, 0, 31, 0, 0, 0,
    73, 32, 99, 97, 110, 32, 117, 115, 101, 32, 34, 100, 111, 117, 98, 108, 101, 32, 113, 117, 111,
    116, 101, 115, 34, 32, 104, 101, 114, 101, 0, 2, 108, 105, 110, 101, 66, 114, 101, 97, 107,
    115, 0, 20, 0, 0, 0, 76, 111, 111, 107, 44, 32, 77, 111, 109, 33, 32, 78, 111, 32, 92, 110, 39,
    115, 33, 0, 16, 104, 101, 120, 97, 100, 101, 99, 105, 109, 97, 108, 0, 175, 236, 13, 0, 1, 108,
    101, 97, 100, 105, 110, 103, 68, 101, 99, 105, 109, 97, 108, 80, 111, 105, 110, 116, 0, 78,
    159, 120, 41, 208, 194, 235, 63, 1, 97, 110, 100, 84, 114, 97, 105, 108, 105, 110, 103, 0, 0,
    0, 0, 160, 253, 139, 96, 65, 16, 112, 111, 115, 105, 116, 105, 118, 101, 83, 105, 103, 110, 0,
    1, 0, 0, 0, 2, 116, 114, 97, 105, 108, 105, 110, 103, 67, 111, 109, 109, 97, 0, 11, 0, 0, 0,
    105, 110, 32, 111, 98, 106, 101, 99, 116, 115, 0, 4, 97, 110, 100, 73, 110, 0, 35, 0, 0, 0, 2,
    48, 0, 7, 0, 0, 0, 97, 114, 114, 97, 121, 115, 0, 2, 49, 0, 9, 0, 0, 0, 97, 114, 114, 97, 121,
    115, 45, 50, 0, 0, 2, 98, 97, 99, 107, 119, 97, 114, 100, 115, 67, 111, 109, 112, 97, 116, 105,
    98, 108, 101, 0, 10, 0, 0, 0, 119, 105, 116, 104, 32, 74, 83, 79, 78, 0, 0,
];

// ContentType::Cbor
pub(crate) const CBOR_SERIALIZE: &[u8] = &[
    170, 104, 117, 110, 113, 117, 111, 116, 101, 100, 120, 28, 97, 110, 100, 32, 121, 111, 117, 32,
    99, 97, 110, 32, 113, 117, 111, 116, 101, 32, 109, 101, 32, 111, 110, 32, 116, 104, 97, 116,
    108, 115, 105, 110, 103, 108, 101, 81, 117, 111, 116, 101, 115, 120, 30, 73, 32, 99, 97, 110,
    32, 117, 115, 101, 32, 34, 100, 111, 117, 98, 108, 101, 32, 113, 117, 111, 116, 101, 115, 34,
    32, 104, 101, 114, 101, 106, 108, 105, 110, 101, 66, 114, 101, 97, 107, 115, 115, 76, 111, 111,
    107, 44, 32, 77, 111, 109, 33, 32, 78, 111, 32, 92, 110, 39, 115, 33, 107, 104, 101, 120, 97,
    100, 101, 99, 105, 109, 97, 108, 26, 0, 13, 236, 175, 115, 108, 101, 97, 100, 105, 110, 103,
    68, 101, 99, 105, 109, 97, 108, 80, 111, 105, 110, 116, 251, 63, 235, 194, 208, 41, 120, 159,
    78, 107, 97, 110, 100, 84, 114, 97, 105, 108, 105, 110, 103, 250, 75, 4, 95, 237, 108, 112,
    111, 115, 105, 116, 105, 118, 101, 83, 105, 103, 110, 1, 109, 116, 114, 97, 105, 108, 105, 110,
    103, 67, 111, 109, 109, 97, 106, 105, 110, 32, 111, 98, 106, 101, 99, 116, 115, 101, 97, 110,
    100, 73, 110, 130, 102, 97, 114, 114, 97, 121, 115, 104, 97, 114, 114, 97, 121, 115, 45, 50,
    115, 98, 97, 99, 107, 119, 97, 114, 100, 115, 67, 111, 109, 112, 97, 116, 105, 98, 108, 101,
    105, 119, 105, 116, 104, 32, 74, 83, 79, 78,
];

// ContentType::Ron
pub(crate) const RON_SERIALIZE: &str = r#"(unquoted:"and you can quote me on that",singleQuotes:"I can use \"double quotes\" here",lineBreaks:"Look, Mom! No \\n\'s!",hexadecimal:912559,leadingDecimalPoint:0.8675309,andTrailing:8675309.0,positiveSign:1,trailingComma:"in objects",andIn:["arrays","arrays-2"],backwardsCompatible:"with JSON")"#;
pub(crate) const RON_DESERIALIZE: &str = r#"(
    unquoted: "and you can quote me on that",
    singleQuotes: "I can use \"double quotes\" here",
    lineBreaks: "Look, Mom! No \\n\'s!",
    hexadecimal: 912559,
    leadingDecimalPoint: 0.8675309,
    andTrailing: 8675309,
    positiveSign: 1,
    trailingComma: "in objects",
    andIn: ["arrays","arrays-2"],
    backwardsCompatible: "with JSON"
)
"#;

// ContentType::Toml
pub(crate) const TOML_SERIALIZE: &str = r#"unquoted = "and you can quote me on that"
singleQuotes = 'I can use "double quotes" here'
lineBreaks = '''Look, Mom! No \n's!'''
hexadecimal = 912559
leadingDecimalPoint = 0.8675309
andTrailing = 8675309.0
positiveSign = 1
trailingComma = "in objects"
andIn = ["arrays", "arrays-2"]
backwardsCompatible = "with JSON"
"#;

// ContentType::FlexBuffers
pub(crate) const FLEXBUFFERS_SERIALIZE: &[u8] = &[
    117, 110, 113, 117, 111, 116, 101, 100, 0, 28, 97, 110, 100, 32, 121, 111, 117, 32, 99, 97,
    110, 32, 113, 117, 111, 116, 101, 32, 109, 101, 32, 111, 110, 32, 116, 104, 97, 116, 0, 115,
    105, 110, 103, 108, 101, 81, 117, 111, 116, 101, 115, 0, 30, 73, 32, 99, 97, 110, 32, 117, 115,
    101, 32, 34, 100, 111, 117, 98, 108, 101, 32, 113, 117, 111, 116, 101, 115, 34, 32, 104, 101,
    114, 101, 0, 108, 105, 110, 101, 66, 114, 101, 97, 107, 115, 0, 19, 76, 111, 111, 107, 44, 32,
    77, 111, 109, 33, 32, 78, 111, 32, 92, 110, 39, 115, 33, 0, 104, 101, 120, 97, 100, 101, 99,
    105, 109, 97, 108, 0, 108, 101, 97, 100, 105, 110, 103, 68, 101, 99, 105, 109, 97, 108, 80,
    111, 105, 110, 116, 0, 97, 110, 100, 84, 114, 97, 105, 108, 105, 110, 103, 0, 112, 111, 115,
    105, 116, 105, 118, 101, 83, 105, 103, 110, 0, 116, 114, 97, 105, 108, 105, 110, 103, 67, 111,
    109, 109, 97, 0, 10, 105, 110, 32, 111, 98, 106, 101, 99, 116, 115, 0, 97, 110, 100, 73, 110,
    0, 6, 97, 114, 114, 97, 121, 115, 0, 8, 97, 114, 114, 97, 121, 115, 45, 50, 0, 2, 18, 11, 20,
    20, 98, 97, 99, 107, 119, 97, 114, 100, 115, 67, 111, 109, 112, 97, 116, 105, 98, 108, 101, 0,
    9, 119, 105, 116, 104, 32, 74, 83, 79, 78, 0, 0, 10, 0, 63, 0, 116, 0, 38, 0, 152, 0, 142, 0,
    188, 0, 114, 0, 237, 0, 105, 0, 24, 1, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 253, 139, 96, 65,
    79, 0, 0, 0, 0, 0, 0, 0, 175, 236, 13, 0, 0, 0, 0, 0, 78, 159, 120, 41, 208, 194, 235, 63, 0,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 59, 1, 0, 0, 0, 0, 0, 0, 188, 0, 0, 0, 0, 0, 0, 0,
    118, 1, 0, 0, 0, 0, 0, 0, 40, 15, 20, 7, 15, 20, 7, 20, 20, 20, 90, 39, 1,
];
// ContentType::Lexpr
pub(crate) const LEXPR_SERIALIZE: &str = r#"((unquoted . "and you can quote me on that") (singleQuotes . "I can use \"double quotes\" here") (lineBreaks . "Look, Mom! No \\n's!") (hexadecimal . 912559) (leadingDecimalPoint . 0.8675309) (andTrailing . 8675309.0) (positiveSign . 1) (trailingComma . "in objects") (andIn "arrays" "arrays-2") (backwardsCompatible . "with JSON"))"#;
pub(crate) const LEXPR_DESERIALIZE: &str = r#"(
(unquoted . "and you can quote me on that")
(singleQuotes . "I can use \"double quotes\" here")
(lineBreaks . "Look, Mom! No \\n's!")
(hexadecimal . 912559)
(leadingDecimalPoint . 0.8675309)
(andTrailing . 8675309.0)
(positiveSign . 1)
(trailingComma . "in objects")
(andIn "arrays" "arrays-2")
(backwardsCompatible . "with JSON"))
"#;
// ContentType::MessagePack
pub(crate) const MESSAGEPACK_SERIALIZE: &[u8] = &[
    154, 188, 97, 110, 100, 32, 121, 111, 117, 32, 99, 97, 110, 32, 113, 117, 111, 116, 101, 32,
    109, 101, 32, 111, 110, 32, 116, 104, 97, 116, 190, 73, 32, 99, 97, 110, 32, 117, 115, 101, 32,
    34, 100, 111, 117, 98, 108, 101, 32, 113, 117, 111, 116, 101, 115, 34, 32, 104, 101, 114, 101,
    179, 76, 111, 111, 107, 44, 32, 77, 111, 109, 33, 32, 78, 111, 32, 92, 110, 39, 115, 33, 206,
    0, 13, 236, 175, 203, 63, 235, 194, 208, 41, 120, 159, 78, 203, 65, 96, 139, 253, 160, 0, 0, 0,
    1, 170, 105, 110, 32, 111, 98, 106, 101, 99, 116, 115, 146, 166, 97, 114, 114, 97, 121, 115,
    168, 97, 114, 114, 97, 121, 115, 45, 50, 169, 119, 105, 116, 104, 32, 74, 83, 79, 78,
];
// ContentType::Pickle
pub(crate) const PICKLE_SERIALIZE: &[u8] = &[
    128, 3, 125, 40, 88, 8, 0, 0, 0, 117, 110, 113, 117, 111, 116, 101, 100, 88, 28, 0, 0, 0, 97,
    110, 100, 32, 121, 111, 117, 32, 99, 97, 110, 32, 113, 117, 111, 116, 101, 32, 109, 101, 32,
    111, 110, 32, 116, 104, 97, 116, 88, 12, 0, 0, 0, 115, 105, 110, 103, 108, 101, 81, 117, 111,
    116, 101, 115, 88, 30, 0, 0, 0, 73, 32, 99, 97, 110, 32, 117, 115, 101, 32, 34, 100, 111, 117,
    98, 108, 101, 32, 113, 117, 111, 116, 101, 115, 34, 32, 104, 101, 114, 101, 88, 10, 0, 0, 0,
    108, 105, 110, 101, 66, 114, 101, 97, 107, 115, 88, 19, 0, 0, 0, 76, 111, 111, 107, 44, 32, 77,
    111, 109, 33, 32, 78, 111, 32, 92, 110, 39, 115, 33, 88, 11, 0, 0, 0, 104, 101, 120, 97, 100,
    101, 99, 105, 109, 97, 108, 74, 175, 236, 13, 0, 88, 19, 0, 0, 0, 108, 101, 97, 100, 105, 110,
    103, 68, 101, 99, 105, 109, 97, 108, 80, 111, 105, 110, 116, 71, 63, 235, 194, 208, 41, 120,
    159, 78, 88, 11, 0, 0, 0, 97, 110, 100, 84, 114, 97, 105, 108, 105, 110, 103, 71, 65, 96, 139,
    253, 160, 0, 0, 0, 88, 12, 0, 0, 0, 112, 111, 115, 105, 116, 105, 118, 101, 83, 105, 103, 110,
    74, 1, 0, 0, 0, 88, 13, 0, 0, 0, 116, 114, 97, 105, 108, 105, 110, 103, 67, 111, 109, 109, 97,
    88, 10, 0, 0, 0, 105, 110, 32, 111, 98, 106, 101, 99, 116, 115, 88, 5, 0, 0, 0, 97, 110, 100,
    73, 110, 93, 40, 88, 6, 0, 0, 0, 97, 114, 114, 97, 121, 115, 88, 8, 0, 0, 0, 97, 114, 114, 97,
    121, 115, 45, 50, 101, 88, 19, 0, 0, 0, 98, 97, 99, 107, 119, 97, 114, 100, 115, 67, 111, 109,
    112, 97, 116, 105, 98, 108, 101, 88, 9, 0, 0, 0, 119, 105, 116, 104, 32, 74, 83, 79, 78, 117,
    46,
];
// ContentType::Postcard
pub(crate) const POSTCARD_SERIALIZE: &[u8] = &[
    28, 97, 110, 100, 32, 121, 111, 117, 32, 99, 97, 110, 32, 113, 117, 111, 116, 101, 32, 109,
    101, 32, 111, 110, 32, 116, 104, 97, 116, 30, 73, 32, 99, 97, 110, 32, 117, 115, 101, 32, 34,
    100, 111, 117, 98, 108, 101, 32, 113, 117, 111, 116, 101, 115, 34, 32, 104, 101, 114, 101, 19,
    76, 111, 111, 107, 44, 32, 77, 111, 109, 33, 32, 78, 111, 32, 92, 110, 39, 115, 33, 222, 178,
    111, 78, 159, 120, 41, 208, 194, 235, 63, 0, 0, 0, 160, 253, 139, 96, 65, 2, 10, 105, 110, 32,
    111, 98, 106, 101, 99, 116, 115, 2, 6, 97, 114, 114, 97, 121, 115, 8, 97, 114, 114, 97, 121,
    115, 45, 50, 9, 119, 105, 116, 104, 32, 74, 83, 79, 78,
];
// ContentType::Url
pub(crate) const URL_SERIALIZE: &str = r#"unquoted=and+you+can+quote+me+on+that&singleQuotes=I+can+use+%22double+quotes%22+here&lineBreaks=Look%2C+Mom%21+No+%5Cn%27s%21&hexadecimal=912559&leadingDecimalPoint=0.8675309&andTrailing=8675309&positiveSign=1&trailingComma=in+objects&andIn[0]=arrays&andIn[1]=arrays-2&backwardsCompatible=with+JSON"#;
