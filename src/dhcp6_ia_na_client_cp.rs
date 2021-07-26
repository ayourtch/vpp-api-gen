/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Dhcp6ClientEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
impl Dhcp6ClientEnableDisable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("dhcp6_client_enable_disable_ae6cfcfb") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Dhcp6ClientEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Dhcp6ClientEnableDisableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("dhcp6_client_enable_disable_reply_e8d4e804") 
	 } 
} 
