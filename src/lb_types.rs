/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct LbVip { 
	pub pfx : AddressWithPrefix, 
	pub protocol : IpProto, 
	pub port : u16, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbSrvType { 
	 LB_API_SRV_TYPE_CLUSTERIP=0, 
	 LB_API_SRV_TYPE_NODEPORT=1, 
	 LB_API_SRV_N_TYPES=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbEncapType { 
	 LB_API_ENCAP_TYPE_GRE4=0, 
	 LB_API_ENCAP_TYPE_GRE6=1, 
	 LB_API_ENCAP_TYPE_L3DSR=2, 
	 LB_API_ENCAP_TYPE_NAT4=3, 
	 LB_API_ENCAP_TYPE_NAT6=4, 
	 LB_API_ENCAP_N_TYPES=5, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbLkpTypeT { 
	 LB_API_LKP_SAME_IP_PORT=0, 
	 LB_API_LKP_DIFF_IP_PORT=1, 
	 LB_API_LKP_ALL_PORT_IP=2, 
	 LB_API_LKP_N_TYPES=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbVipType { 
	 LB_API_VIP_TYPE_IP6_GRE6=0, 
	 LB_API_VIP_TYPE_IP6_GRE4=1, 
	 LB_API_VIP_TYPE_IP4_GRE6=2, 
	 LB_API_VIP_TYPE_IP4_GRE4=3, 
	 LB_API_VIP_TYPE_IP4_L3DSR=4, 
	 LB_API_VIP_TYPE_IP4_NAT4=5, 
	 LB_API_VIP_TYPE_IP6_NAT6=6, 
	 LB_API_VIP_N_TYPES=7, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbNatProtocol { 
	 LB_API_NAT_PROTOCOL_UDP=6, 
	 LB_API_NAT_PROTOCOL_TCP=23, 
	 LB_API_NAT_PROTOCOL_ANY=4294967295, 
} 