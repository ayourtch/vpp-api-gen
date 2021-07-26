/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FibMplsLabel { 
	pub is_uniform : u8, 
	pub label : u32, 
	pub ttl : u8, 
	pub exp : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FibPathNh { 
	pub address : AddressUnion, 
	pub via_label : u32, 
	pub obj_id : u32, 
	pub classify_table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FibPath { 
	pub sw_if_index : u32, 
	pub table_id : u32, 
	pub rpf_id : u32, 
	pub weight : u8, 
	pub preference : u8, 
	pub typ : FibPathType, 
	pub flags : FibPathFlags, 
	pub proto : FibPathNhProto, 
	pub nh : FibPathNh, 
	pub n_labels : u8, 
	pub label_stack : FibMplsLabel, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathNhProto { 
	 FIB_API_PATH_NH_PROTO_IP4=0, 
	 FIB_API_PATH_NH_PROTO_IP6=1, 
	 FIB_API_PATH_NH_PROTO_MPLS=2, 
	 FIB_API_PATH_NH_PROTO_ETHERNET=3, 
	 FIB_API_PATH_NH_PROTO_BIER=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathFlags { 
	 FIB_API_PATH_FLAG_NONE=0, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_ATTACHED=1, 
	 FIB_API_PATH_FLAG_RESOLVE_VIA_HOST=2, 
	 FIB_API_PATH_FLAG_POP_PW_CW=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum FibPathType { 
	 FIB_API_PATH_TYPE_NORMAL=0, 
	 FIB_API_PATH_TYPE_LOCAL=1, 
	 FIB_API_PATH_TYPE_DROP=2, 
	 FIB_API_PATH_TYPE_UDP_ENCAP=3, 
	 FIB_API_PATH_TYPE_BIER_IMP=4, 
	 FIB_API_PATH_TYPE_ICMP_UNREACH=5, 
	 FIB_API_PATH_TYPE_ICMP_PROHIBIT=6, 
	 FIB_API_PATH_TYPE_SOURCE_LOOKUP=7, 
	 FIB_API_PATH_TYPE_DVR=8, 
	 FIB_API_PATH_TYPE_INTERFACE_RX=9, 
	 FIB_API_PATH_TYPE_CLASSIFY=10, 
} 