/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IoamExportIp6EnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_disable : bool, 
	pub collector_address : Ip4Address, 
	pub src_address : Ip4Address, 
} 
impl IoamExportIp6EnableDisable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ioam_export_ip6_enable_disable_e4d4ebfa") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IoamExportIp6EnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IoamExportIp6EnableDisableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ioam_export_ip6_enable_disable_reply_e8d4e804") 
	 } 
} 