use serde::{Deserialize, Serialize};
extern crate strum;
use crate::alias::VppJsApiAlias;
use crate::enums::VppJsApiEnum;
use crate::message::VppJsApiMessage;
use crate::services::{VppJsApiOptions, VppJsApiService};
use crate::types::VppJsApiType;
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VppJsApiCounterElement {
    pub name: String,
    pub severity: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub units: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VppJsApiCounter {
    pub name: String,
    pub elements: Vec<VppJsApiCounterElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VppJsApiPath {
    pub path: String,
    pub counter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VppJsApiFile {
    pub types: Vec<VppJsApiType>,
    pub messages: Vec<VppJsApiMessage>,
    pub unions: Vec<VppJsApiType>,
    pub enums: Vec<VppJsApiEnum>,
    #[serde(default)]
    pub enumflags: Vec<VppJsApiEnum>,
    pub services: LinkedHashMap<String, VppJsApiService>,
    pub options: VppJsApiOptions,
    pub aliases: LinkedHashMap<String, VppJsApiAlias>,
    pub vl_api_version: String,
    pub imports: Vec<String>,
    pub counters: Vec<VppJsApiCounter>,
    pub paths: Vec<Vec<VppJsApiPath>>,
}

impl VppJsApiFile {
    pub fn verify_data(data: &str, jaf: &VppJsApiFile) {
        use serde_json::Value;
        /*
         * Here we verify that we are not dropping anything during the
         * serialization/deserialization. To do that we use the typeless
         * serde:
         *
         * string_data -> json_deserialize -> json_serialize_pretty -> good_json
         *
         * string_data -> VPPJAF_deserialize -> VPPJAF_serialize ->
         *             -> json_deserialize -> json_serialize_pretty -> test_json
         *
         * Then we compare the two values for being identical and panic if they
         * aren't.
         */

        let good_val: Value = serde_json::from_str(data).unwrap();
        let good_json = serde_json::to_string_pretty(&good_val).unwrap();

        let jaf_serialized_json = serde_json::to_string_pretty(jaf).unwrap();
        let test_val: Value = serde_json::from_str(&jaf_serialized_json).unwrap();
        let test_json = serde_json::to_string_pretty(&test_val).unwrap();

        if good_json != test_json {
            eprintln!("{}", good_json);
            println!("{}", test_json);
            panic!("Different javascript in internal sanity self-test");
        }
    }

    pub fn from_str(data: &str) -> std::result::Result<VppJsApiFile, serde_json::Error> {
        // use serde_json::Value;
        let res = serde_json::from_str::<VppJsApiFile>(&data);
        res
    }

    pub fn generate_header() -> String {
        let mut header = String::new();
        header.push_str("/*\n");
        header.push_str("   Autogenerated Data, Do not Edit! \n");
        // header.push_str("   Author: @felixfaisal \n");
        header.push_str("*/\n");
        header
    }

    pub fn generate_code(&self, name: &str, api_definition: &mut Vec<(String, String)>) -> String {
        lazy_static! {
            static ref IE: Regex = Regex::new(r"/[a-z_0-9]*.api").unwrap();
            static ref RE: Regex = Regex::new(r"/[a-z_0-9]*.api.json").unwrap();
        }
        let mut preamble: String = String::new();
        preamble.push_str(&VppJsApiFile::generate_header());
        preamble.push_str("#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]\n");
        preamble.push_str("use vpp_api_macros::{VppMessage,VppUnionIdent}; \n");
        preamble.push_str("use std::convert::TryInto; \n");
        preamble.push_str("use serde::{de::DeserializeOwned, Deserialize, Serialize};\n");
        preamble.push_str("use vpp_api_encoding::typ::*;\n");
        preamble.push_str("use serde_repr::{Serialize_repr, Deserialize_repr};\n");
        preamble.push_str("use typenum;\n");
        let mut import_table: Vec<(String, Vec<String>)> = vec![];
        let typstructs = VppJsApiType::iter_and_generate_code(
            &self.types,
            api_definition,
            name,
            &mut import_table,
        );
        let typunions = VppJsApiType::iter_and_generate_code_union(
            &self.unions,
            api_definition,
            name,
            &self,
            &mut import_table,
        );
        let typenum = VppJsApiEnum::iter_and_generate_code(
            &self.enums,
            api_definition,
            name,
            &mut import_table,
        );
        let typenumflags = VppJsApiEnum::iter_and_generate_code(
            &self.enumflags,
            api_definition,
            name,
            &mut import_table,
        );
        let typalias = VppJsApiAlias::iter_and_generate_code(
            &self.aliases,
            api_definition,
            name,
            &mut import_table,
        );
        let typmessage = VppJsApiMessage::iter_and_generate_code(&self.messages);

        for x in 0..import_table.len() {
            let name = &import_table[x].0;
            let fileName = RE
                .find(&name)
                .unwrap()
                .as_str()
                .trim_end_matches(".api.json")
                .trim_start_matches("/");
            preamble.push_str(&format!("use crate::{}::*; \n", fileName));
        }
        preamble.push_str(&typstructs);
        preamble.push_str(&typunions);
        preamble.push_str(&typenum);
        preamble.push_str(&typenumflags);
        preamble.push_str(&typalias);
        preamble.push_str(&typmessage);
        preamble
    }
}
