/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SyslogSeverity { 
	 SYSLOG_API_SEVERITY_EMERG=0, 
	 SYSLOG_API_SEVERITY_ALERT=1, 
	 SYSLOG_API_SEVERITY_CRIT=2, 
	 SYSLOG_API_SEVERITY_ERR=3, 
	 SYSLOG_API_SEVERITY_WARN=4, 
	 SYSLOG_API_SEVERITY_NOTICE=5, 
	 SYSLOG_API_SEVERITY_INFO=6, 
	 SYSLOG_API_SEVERITY_DBG=7, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogSetSender { 
	pub client_index : u32, 
	pub context : u32, 
	pub src_address : Ip4Address, 
	pub collector_address : Ip4Address, 
	pub collector_port : u16, 
	pub vrf_id : u32, 
	pub max_msg_size : u32, 
} 
impl SyslogSetSender { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_set_sender_bb641285") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogSetSenderReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SyslogSetSenderReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_set_sender_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogGetSender { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SyslogGetSender { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_get_sender_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogGetSenderReply { 
	pub context : u32, 
	pub retval : i32, 
	pub src_address : Ip4Address, 
	pub collector_address : Ip4Address, 
	pub collector_port : u16, 
	pub vrf_id : u32, 
	pub max_msg_size : u32, 
} 
impl SyslogGetSenderReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_get_sender_reply_d3da60ac") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogSetFilter { 
	pub client_index : u32, 
	pub context : u32, 
	pub severity : SyslogSeverity, 
} 
impl SyslogSetFilter { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_set_filter_571348c3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogSetFilterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SyslogSetFilterReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_set_filter_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogGetFilter { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SyslogGetFilter { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_get_filter_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SyslogGetFilterReply { 
	pub context : u32, 
	pub retval : i32, 
	pub severity : SyslogSeverity, 
} 
impl SyslogGetFilterReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("syslog_get_filter_reply_eb1833f8") 
	 } 
} 
