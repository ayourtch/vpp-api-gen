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
pub struct Ikev2Id { 
	pub typ : u8, 
	pub data_len : u8, 
	pub data : String, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Ts { 
	pub sa_index : u32, 
	pub child_sa_index : u32, 
	pub is_local : bool, 
	pub protocol_id : u8, 
	pub start_port : u16, 
	pub end_port : u16, 
	pub start_addr : Address, 
	pub end_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Auth { 
	pub method : u8, 
	pub hex : u8, 
	pub data_len : u32, 
	pub data : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Responder { 
	pub sw_if_index : InterfaceIndex, 
	pub addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2IkeTransforms { 
	pub crypto_alg : u8, 
	pub crypto_key_size : u32, 
	pub integ_alg : u8, 
	pub dh_group : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2EspTransforms { 
	pub crypto_alg : u8, 
	pub crypto_key_size : u32, 
	pub integ_alg : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Profile { 
	pub name : String, 
	pub loc_id : Ikev2Id, 
	pub rem_id : Ikev2Id, 
	pub loc_ts : Ikev2Ts, 
	pub rem_ts : Ikev2Ts, 
	pub responder : Ikev2Responder, 
	pub ike_ts : Ikev2IkeTransforms, 
	pub esp_ts : Ikev2EspTransforms, 
	pub lifetime : u64, 
	pub lifetime_maxdata : u64, 
	pub lifetime_jitter : u32, 
	pub handover : u32, 
	pub ipsec_over_udp_port : u16, 
	pub tun_itf : u32, 
	pub udp_encap : bool, 
	pub natt_disabled : bool, 
	pub auth : Ikev2Auth, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaTransform { 
	pub transform_type : u8, 
	pub transform_id : u16, 
	pub key_len : u16, 
	pub key_trunc : u16, 
	pub block_size : u16, 
	pub dh_group : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Keys { 
	pub sk_d : u8, 
	pub sk_d_len : u8, 
	pub sk_ai : u8, 
	pub sk_ai_len : u8, 
	pub sk_ar : u8, 
	pub sk_ar_len : u8, 
	pub sk_ei : u8, 
	pub sk_ei_len : u8, 
	pub sk_er : u8, 
	pub sk_er_len : u8, 
	pub sk_pi : u8, 
	pub sk_pi_len : u8, 
	pub sk_pr : u8, 
	pub sk_pr_len : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ChildSa { 
	pub sa_index : u32, 
	pub child_sa_index : u32, 
	pub i_spi : u32, 
	pub r_spi : u32, 
	pub keys : Ikev2Keys, 
	pub encryption : Ikev2SaTransform, 
	pub integrity : Ikev2SaTransform, 
	pub esn : Ikev2SaTransform, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaStats { 
	pub n_keepalives : u16, 
	pub n_rekey_req : u16, 
	pub n_sa_init_req : u16, 
	pub n_sa_auth_req : u16, 
	pub n_retransmit : u16, 
	pub n_init_sa_retransmit : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Sa { 
	pub sa_index : u32, 
	pub profile_index : u32, 
	pub ispi : u64, 
	pub rspi : u64, 
	pub iaddr : Address, 
	pub raddr : Address, 
	pub keys : Ikev2Keys, 
	pub i_id : Ikev2Id, 
	pub r_id : Ikev2Id, 
	pub encryption : Ikev2SaTransform, 
	pub integrity : Ikev2SaTransform, 
	pub prf : Ikev2SaTransform, 
	pub dh : Ikev2SaTransform, 
	pub stats : Ikev2SaStats, 
} 