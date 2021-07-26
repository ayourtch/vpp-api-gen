/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AdlInterfaceEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable_disable : bool, 
} 
impl AdlInterfaceEnableDisable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("adl_interface_enable_disable_5501adee") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AdlInterfaceEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AdlInterfaceEnableDisableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("adl_interface_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AdlAllowlistEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub fib_id : u32, 
	pub ip4 : bool, 
	pub ip6 : bool, 
	pub default_adl : bool, 
} 
impl AdlAllowlistEnableDisable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("adl_allowlist_enable_disable_ea88828d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AdlAllowlistEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl AdlAllowlistEnableDisableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("adl_allowlist_enable_disable_reply_e8d4e804") 
	 } 
} 
