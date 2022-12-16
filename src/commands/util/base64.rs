use clap::Parser;

use crate::commands::util::types::CharacterSet;

#[derive(Debug, Clone, Parser)]
pub struct UtilBase64Args {
    #[clap(subcommand)]
    pub subcommand: Operations,
}

#[derive(Debug, Clone, Parser)]
pub enum Operations {
    /// Encode a string to base64
    Encode(EncodingArgs),
    /// Decode a base64 string
    Decode(DecodingArgs),
}

pub fn exec(args: &UtilBase64Args) {
    let UtilBase64Args { subcommand } = args;

    match subcommand {
        Operations::Encode(encoding_args) => encode(encoding_args),
        Operations::Decode(decoding_args) => decode(decoding_args),
    }
}

#[derive(Debug, Clone, Parser)]
pub struct EncodingArgs {
    /// The string to encode
    pub string: String,
    /// Possibles
    #[clap(short, long, value_enum, default_value_t=CharacterSet::Standard)]
    pub character_set: CharacterSet,
}

#[derive(Debug, Clone, Parser)]
pub struct DecodingArgs {
    /// The string to decode
    pub string: String,
    /// Possibles
    #[clap(short, long, value_enum, default_value_t=CharacterSet::Standard)]
    pub character_set: CharacterSet,
}

fn encode(args: &EncodingArgs) {
    let EncodingArgs {
        string,
        character_set,
    } = args;

    // encode the string to base64 with the format
    let encoded_string = base64::encode_engine(string.as_bytes(), &character_set.create_engine());
    println!("{}", encoded_string);
}

fn decode(args: &DecodingArgs) {
    let DecodingArgs {
        string,
        character_set,
    } = args;

    // decode the string from base64 with the format
    let decoded = base64::decode_engine(string.as_bytes(), &character_set.create_engine())
        .expect("Failed to decode the string");
    let decoded_string =
        String::from_utf8(decoded).expect("Failed to convert the bytes to a string");
    println!("{}", decoded_string);
}
