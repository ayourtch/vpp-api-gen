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
pub struct VxlanAddDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub instance : u32, 
	pub src_address : Address, 
	pub dst_address : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
impl VxlanAddDelTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_add_del_tunnel_a35dc8f5") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanAddDelTunnelV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub instance : u32, 
	pub src_address : Address, 
	pub dst_address : Address, 
	pub src_port : u16, 
	pub dst_port : u16, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
impl VxlanAddDelTunnelV2 { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_add_del_tunnel_v2_4f223f40") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanAddDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanAddDelTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_add_del_tunnel_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanAddDelTunnelV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanAddDelTunnelV2Reply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_add_del_tunnel_v2_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanTunnelDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_tunnel_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanTunnelV2Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VxlanTunnelV2Dump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_tunnel_v2_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanTunnelDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub instance : u32, 
	pub src_address : Address, 
	pub dst_address : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
impl VxlanTunnelDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_tunnel_details_e782f70f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanTunnelV2Details { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub instance : u32, 
	pub src_address : Address, 
	pub dst_address : Address, 
	pub src_port : u16, 
	pub dst_port : u16, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_next_index : u32, 
	pub vni : u32, 
} 
impl VxlanTunnelV2Details { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_tunnel_v2_details_d3bdd4d9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
impl SwInterfaceSetVxlanBypass { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_bypass_65247409") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceSetVxlanBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SwInterfaceSetVxlanBypassReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_set_vxlan_bypass_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanOffloadRx { 
	pub client_index : u32, 
	pub context : u32, 
	pub hw_if_index : InterfaceIndex, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
impl VxlanOffloadRx { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_offload_rx_89a1564b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VxlanOffloadRxReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VxlanOffloadRxReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("vxlan_offload_rx_reply_e8d4e804") 
	 } 
} 
