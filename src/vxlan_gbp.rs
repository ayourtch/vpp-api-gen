/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGbpTunnel { 
	pub instance : u32, 
	pub src : Address, 
	pub dst : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_table_id : u32, 
	pub vni : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub mode : VxlanGbpApiTunnelMode, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VxlanGbpApiTunnelMode { 
	 VXLAN_GBP_API_TUNNEL_MODE_L2=1, 
	 VXLAN_GBP_API_TUNNEL_MODE_L3=2, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGbpTunnelAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub tunnel : VxlanGbpTunnel, 
} 
impl VxlanGbpTunnelAddDel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gbp_tunnel_add_del_8c819166") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGbpTunnelAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanGbpTunnelAddDelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gbp_tunnel_add_del_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGbpTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanGbpTunnelDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gbp_tunnel_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGbpTunnelDetails { 
	pub context : u32, 
	pub tunnel : VxlanGbpTunnel, 
} 
impl VxlanGbpTunnelDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gbp_tunnel_details_1da24016") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanGbpBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
impl SwInterfaceSetVxlanGbpBypass { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_gbp_bypass_65247409") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanGbpBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SwInterfaceSetVxlanGbpBypassReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_gbp_bypass_reply_e8d4e804") 
	 } 
} 
