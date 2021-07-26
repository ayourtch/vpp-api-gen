/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
use crate::ip_types::*; 
use crate::tunnel_types::*; 
use crate::interface_types::*; 
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipTunnel { 
	pub instance : u32, 
	pub src : Address, 
	pub dst : Address, 
	pub sw_if_index : InterfaceIndex, 
	pub table_id : u32, 
	pub flags : TunnelEncapDecapFlags, 
	pub mode : TunnelMode, 
	pub dscp : IpDscp, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipAddTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub tunnel : IpipTunnel, 
} 
impl IpipAddTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_add_tunnel_a9decfcd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipAddTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpipAddTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_add_tunnel_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpipDelTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_del_tunnel_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpipDelTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_del_tunnel_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ipip6rdAddTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip6_table_id : u32, 
	pub ip4_table_id : u32, 
	pub ip6_prefix : Ip6Prefix, 
	pub ip4_prefix : Ip4Prefix, 
	pub ip4_src : Ip4Address, 
	pub security_check : bool, 
	pub tc_tos : u8, 
} 
impl Ipip6rdAddTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_6rd_add_tunnel_56e93cc0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ipip6rdAddTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Ipip6rdAddTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_6rd_add_tunnel_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ipip6rdDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl Ipip6rdDelTunnel { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_6rd_del_tunnel_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ipip6rdDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl Ipip6rdDelTunnelReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_6rd_del_tunnel_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl IpipTunnelDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_tunnel_dump_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpipTunnelDetails { 
	pub context : u32, 
	pub tunnel : IpipTunnel, 
} 
impl IpipTunnelDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("ipip_tunnel_details_53236d75") 
	 } 
} 
