/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
use crate::ethernet_types::*; 
use crate::pci_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VirtioFlags { 
	 VIRTIO_API_FLAG_GSO=1, 
	 VIRTIO_API_FLAG_CSUM_OFFLOAD=2, 
	 VIRTIO_API_FLAG_GRO_COALESCE=4, 
	 VIRTIO_API_FLAG_PACKED=8, 
	 VIRTIO_API_FLAG_IN_ORDER=16, 
	 VIRTIO_API_FLAG_BUFFERING=32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub gso_enabled : bool, 
	pub checksum_offload_enabled : bool, 
	pub features : u64, 
} 
impl VirtioPciCreate { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_create_a9f1370c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciCreateReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_create_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub virtio_flags : VirtioFlags, 
	pub features : u64, 
} 
impl VirtioPciCreateV2 { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_create_v2_5d096e1a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciCreateV2Reply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_create_v2_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciDelete { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_delete_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VirtioPciDeleteReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("virtio_pci_delete_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceVirtioPciDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SwInterfaceVirtioPciDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_virtio_pci_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceVirtioPciDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub pci_addr : PciAddress, 
	pub mac_addr : MacAddress, 
	pub tx_ring_sz : u16, 
	pub rx_ring_sz : u16, 
	pub features : u64, 
} 
impl SwInterfaceVirtioPciDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("sw_interface_virtio_pci_details_16187f3a") 
	 } 
} 