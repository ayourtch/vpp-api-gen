#![allow(
    dead_code,
    unused_mut,
    unused_variables,
    unused_must_use,
    non_camel_case_types,
    unused_imports
)]
use env_logger::filter;
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use regex::Regex;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;

use crate::alias::VppJsApiAlias;
use crate::basetypes::{maxSizeUnion, sizeof_alias, sizeof_struct};
use crate::enums::VppJsApiEnum;
use crate::file_schema::VppJsApiFile;
use crate::message::VppJsApiMessage;
use crate::parser_helper::{camelize_ident, get_ident, get_type};
use crate::types::VppJsApiType;
use crate::types::{VppJsApiFieldSize, VppJsApiMessageFieldDef};

impl VppJsApiType {
    pub fn generate_code(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!(
            "#[derive(Debug, Clone, Serialize, Deserialize)] \n"
        ));
        code.push_str(&format!(
            "pub struct {} {{ \n",
            camelize_ident(&self.type_name)
        ));
        for x in 0..self.fields.len() {
            code.push_str(&format!(
                "\tpub {} : {}, \n",
                get_ident(&self.fields[x].name),
                get_type(&self.fields[x].ctype)
            ));
        }
        code.push_str("} \n");
        code
    }
    pub fn generate_code_union(&self, apifile: &VppJsApiFile) -> String {
        let mut code = String::new();
        let unionsize = maxSizeUnion(&self, &apifile);
        code.push_str(&format!(
            "pub type {} = [u8;{}]; \n",
            camelize_ident(&self.type_name),
            unionsize
        ));
        code
    }
}
impl VppJsApiAlias {
    pub fn generate_code(&self, name: &str) -> String {
        let mut code = String::new();
        code.push_str(&format!("pub type {}=", camelize_ident(&get_ident(&name))));
        match self.length {
            Some(len) => {
                let newtype = get_type(&self.ctype);
                code.push_str(&format!("[{};{}]; \n", newtype, len));
            }
            _ => code.push_str(&format!("{}; \n", get_type(&self.ctype))),
        }
        code
    }
    // Handling Vector of Alias
    pub fn handle_alias_vec(aliases: &LinkedHashMap<String,VppJsApiAlias>, api_definition: &mut Vec<(String, String)>, name: &str) -> String {
        aliases
        .keys()
        .filter(|x| {
            for j in 0..api_definition.len() {
                if &api_definition[j].0 == *x {
                    return false;
                }
            }
            api_definition.push((x.clone().to_string(), name.to_string().clone()));
            return true;
        })
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&aliases[x].generate_code(x));
            acc
        })
    }
}
impl VppJsApiEnum {
    pub fn generate_code(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!(
            "#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] \n"
        ));
        match &self.info.enumtype {
            Some(len) => code.push_str(&format!("#[repr({})]\n", &len)),
            _ => code.push_str(&format!("#[repr(u32)]\n")),
        }
        code.push_str(&format!("pub enum {} {{ \n", camelize_ident(&self.name)));
        for x in 0..self.values.len() {
            code.push_str(&format!(
                "\t {}={}, \n",
                get_ident(&self.values[x].name),
                self.values[x].value
            ));
        }
        code.push_str("} \n");
        code
    }
}
impl VppJsApiMessage {
    pub fn generate_code(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!(
            "#[derive(Debug, Clone, Serialize, Deserialize)] \n"
        ));
        code.push_str(&format!("pub struct {} {{ \n", camelize_ident(&self.name)));
        for x in 0..self.fields.len() {
            if self.fields[x].name == "_vl_msg_id" {
                // panic!("Something wrong");
            } else if self.fields[x].ctype == "string" {
                match &self.fields[x].maybe_size {
                    Some(cont) => match cont {
                        VppJsApiFieldSize::Fixed(len) => code.push_str(&format!(
                            "\tpub {} : FixedSizeString<U{}>, \n",
                            get_ident(&self.fields[x].name),
                            len
                        )),
                        VppJsApiFieldSize::Variable(None) => code.push_str(&format!(
                            "\tpub {} : VariableSizeString, \n",
                            get_ident(&self.fields[x].name)
                        )),
                        _ => code
                            .push_str(&format!("\tpub {} : , \n", get_ident(&self.fields[x].name))),
                    },
                    _ => code.push_str(&format!("\tpub {} :, \n", get_ident(&self.fields[x].name))),
                }
            } else {
                code.push_str(&format!(
                    "\tpub {} : {}, \n",
                    get_ident(&self.fields[x].name),
                    get_type(&self.fields[x].ctype)
                ));
            }
        }
        code.push_str("} \n");
        self.gen_impl_messages(&mut code);
        code
    }
    pub fn gen_impl_messages(&self, file: &mut String) {
        file.push_str(&format!("impl {} {{ \n", camelize_ident(&self.name)));
        file.push_str(&format!(
            "\t pub fn get_message_name_and_crc() -> String {{ \n"
        ));
        file.push_str(&format!(
            "\t \t String::from(\"{}_{}\") \n",
            self.name,
            self.info.crc.trim_start_matches("0x")
        ));
        file.push_str(&format!("\t }} \n"));
        file.push_str(&format!("}} \n"));
    }
}

pub fn check_implementation(
    types: &VppJsApiType,
    api_definition: &mut Vec<(String, String)>,
) -> bool {
    for j in 0..api_definition.len() {
        if &api_definition[j].0 == &types.type_name {
            return false;
        }
    }
    return false;
}

pub fn gen_code(code: &VppJsApiFile, name: &str, api_definition: &mut Vec<(String, String)>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/[a-z_0-9]*.api.json").unwrap();
        static ref IE: Regex = Regex::new(r"/[a-z_0-9]*.api").unwrap();
    }
    let mut preamble: String = format!("/* Autogenerated data. Do not edit */\n");
    preamble.push_str("#![allow(non_camel_case_types)]\n");
    preamble.push_str("use serde::{de::DeserializeOwned, Deserialize, Serialize};\n");
    preamble.push_str("use vpp_api_encoding::typ::*;\n");
    preamble.push_str("use vpp_api_transport::*;\n");
    preamble.push_str("use serde_repr::{Serialize_repr, Deserialize_repr};\n");
    preamble.push_str("use typenum::{U10, U24, U256, U32, U64};\n");
    // Do imports
    let importTable: Vec<String> = vec![];
    for x in 0..code.imports.len() {
        let mut count: u8 = 0;
        let check = IE
            .find(&code.imports[x])
            .unwrap()
            .as_str()
            .trim_start_matches("/")
            .trim_end_matches(".api");
        for j in &importTable {
            if j == check {
                count = count + 1;
                break;
            }
        }
        if count == 0 {
            preamble.push_str(&format!("use crate::{}::*; \n", check));
        }
    }
    let structString = code
        .types
        .iter()
        .filter(|x| {
            for j in 0..api_definition.len() {
                if &api_definition[j].0 == &x.type_name {
                    return false;
                }
            }
            api_definition.push((x.type_name.clone(), name.to_string().clone()));
            return true;
        })
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x.generate_code());
            acc
        });
    preamble.push_str(&structString);

    let unionString = code
        .unions
        .iter()
        .filter(|x| {
            for j in 0..api_definition.len() {
                if &api_definition[j].0 == &x.type_name {
                    return false;
                }
            }
            api_definition.push((x.type_name.clone(), name.to_string().clone()));
            return true;
        })
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x.generate_code_union(&code));
            acc
        });
    preamble.push_str(&unionString);
    let enumString = code
        .enums
        .iter()
        .filter(|x| {
            for j in 0..api_definition.len() {
                if &api_definition[j].0 == &x.name {
                    return false;
                }
            }
            api_definition.push((x.name.clone(), name.to_string().clone()));
            return true;
        })
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x.generate_code());
            acc
        });
    preamble.push_str(&enumString);
    // Generating Code for all the aliases
    preamble.push_str(&VppJsApiAlias::handle_alias_vec(&code.aliases, api_definition, name));
    let messageString = code.messages.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(&x.generate_code());
        acc
    });
    preamble.push_str(&messageString);
    /* for x in 0..code.messages.len() {
        gen_messages(&code.messages[x], &mut preamble);
    } */
    println!("{}", name);
    let fileName = RE
        .find(&name)
        .unwrap()
        .as_str()
        .trim_end_matches(".api.json");
    println!("{}", fileName);
    let mut file = File::create(format!("src/{}.rs", fileName)).unwrap();
    file.write_all(preamble.as_bytes()).unwrap();

    println!("Generated code for {}", fileName);
}
