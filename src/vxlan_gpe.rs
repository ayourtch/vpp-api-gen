/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeAddDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub local : Address, 
	pub remote : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub protocol : IpProto, 
	pub vni : u32, 
	pub is_add : bool, 
} 
impl VxlanGpeAddDelTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gpe_add_del_tunnel_7c6da6ae") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeAddDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanGpeAddDelTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gpe_add_del_tunnel_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanGpeTunnelDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gpe_tunnel_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanGpeTunnelDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local : Address, 
	pub remote : Address, 
	pub vni : u32, 
	pub protocol : IpProto, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub is_ipv6 : bool, 
} 
impl VxlanGpeTunnelDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_gpe_tunnel_details_57712346") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanGpeBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
impl SwInterfaceSetVxlanGpeBypass { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_gpe_bypass_65247409") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanGpeBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SwInterfaceSetVxlanGpeBypassReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_gpe_bypass_reply_e8d4e804") 
	 } 
} 
