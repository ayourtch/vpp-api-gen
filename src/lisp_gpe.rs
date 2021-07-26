/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
use crate::lisp_types::*; 
use crate::interface_types::*; 
use crate::ethernet_types::*; 
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeLocator { 
	pub weight : u8, 
	pub addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntry { 
	pub fwd_entry_index : u32, 
	pub dp_table : u32, 
	pub leid : Eid, 
	pub reid : Eid, 
	pub vni : u32, 
	pub action : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeNativeFwdRpath { 
	pub fib_index : u32, 
	pub nh_sw_if_index : InterfaceIndex, 
	pub nh_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelFwdEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub rmt_eid : Eid, 
	pub lcl_eid : Eid, 
	pub vni : u32, 
	pub dp_table : u32, 
	pub action : u8, 
	pub loc_num : u32, 
	pub locs : GpeLocator, 
} 
impl GpeAddDelFwdEntry { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_fwd_entry_de6df50f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelFwdEntryReply { 
	pub context : u32, 
	pub retval : i32, 
	pub fwd_entry_index : u32, 
} 
impl GpeAddDelFwdEntryReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_fwd_entry_reply_efe5f176") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
impl GpeEnableDisable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_enable_disable_c264d7bf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GpeEnableDisableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_enable_disable_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelIface { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_l2 : bool, 
	pub dp_table : u32, 
	pub vni : u32, 
} 
impl GpeAddDelIface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_iface_3ccff273") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelIfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GpeAddDelIfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_iface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntryVnisGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GpeFwdEntryVnisGet { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entry_vnis_get_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntryVnisGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub vnis : u32, 
} 
impl GpeFwdEntryVnisGetReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entry_vnis_get_reply_aa70da20") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
impl GpeFwdEntriesGet { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entries_get_8d1f2fe9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : GpeFwdEntry, 
} 
impl GpeFwdEntriesGetReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entries_get_reply_f9f53f1b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntryPathDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub fwd_entry_index : u32, 
} 
impl GpeFwdEntryPathDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entry_path_dump_39bce980") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeFwdEntryPathDetails { 
	pub context : u32, 
	pub lcl_loc : GpeLocator, 
	pub rmt_loc : GpeLocator, 
} 
impl GpeFwdEntryPathDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_fwd_entry_path_details_ee80b19a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeSetEncapMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_vxlan : bool, 
} 
impl GpeSetEncapMode { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_set_encap_mode_bd819eac") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeSetEncapModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GpeSetEncapModeReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_set_encap_mode_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeGetEncapMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GpeGetEncapMode { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_get_encap_mode_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeGetEncapModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub encap_mode : u8, 
} 
impl GpeGetEncapModeReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_get_encap_mode_reply_36e3f7ca") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelNativeFwdRpath { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub table_id : u32, 
	pub nh_sw_if_index : InterfaceIndex, 
	pub nh_addr : Address, 
} 
impl GpeAddDelNativeFwdRpath { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_native_fwd_rpath_812da2f2") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeAddDelNativeFwdRpathReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GpeAddDelNativeFwdRpathReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_add_del_native_fwd_rpath_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeNativeFwdRpathsGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_ip4 : bool, 
} 
impl GpeNativeFwdRpathsGet { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_native_fwd_rpaths_get_f652ceb4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GpeNativeFwdRpathsGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : GpeNativeFwdRpath, 
} 
impl GpeNativeFwdRpathsGetReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("gpe_native_fwd_rpaths_get_reply_79d54eb9") 
	 } 
} 
