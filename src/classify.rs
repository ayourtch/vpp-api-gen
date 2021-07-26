/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum ClassifyAction { 
	 CLASSIFY_API_ACTION_NONE=0, 
	 CLASSIFY_API_ACTION_SET_IP4_FIB_INDEX=1, 
	 CLASSIFY_API_ACTION_SET_IP6_FIB_INDEX=2, 
	 CLASSIFY_API_ACTION_SET_METADATA=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum PolicerClassifyTable { 
	 POLICER_CLASSIFY_API_TABLE_IP4=1, 
	 POLICER_CLASSIFY_API_TABLE_IP6=2, 
	 POLICER_CLASSIFY_API_TABLE_L2=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum FlowClassifyTable { 
	 FLOW_CLASSIFY_API_TABLE_IP4=1, 
	 FLOW_CLASSIFY_API_TABLE_IP6=2, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyAddDelTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub del_chain : bool, 
	pub table_index : u32, 
	pub nbuckets : u32, 
	pub memory_size : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub next_table_index : u32, 
	pub miss_next_index : u32, 
	pub current_data_flag : u8, 
	pub current_data_offset : i16, 
	pub mask_len : u32, 
	pub mask : u8, 
} 
impl ClassifyAddDelTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_add_del_table_6849e39e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyAddDelTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub new_table_index : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
} 
impl ClassifyAddDelTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_add_del_table_reply_05486349") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyAddDelSession { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub table_index : u32, 
	pub hit_next_index : u32, 
	pub opaque_index : u32, 
	pub advance : i32, 
	pub action : ClassifyAction, 
	pub metadata : u32, 
	pub match_len : u32, 
	pub match : u8, 
} 
impl ClassifyAddDelSession { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_add_del_session_f20879f0") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyAddDelSessionReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl ClassifyAddDelSessionReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_add_del_session_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerClassifySetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
impl PolicerClassifySetInterface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("policer_classify_set_interface_de7ad708") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerClassifySetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl PolicerClassifySetInterfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("policer_classify_set_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerClassifyDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : PolicerClassifyTable, 
	pub sw_if_index : InterfaceIndex, 
} 
impl PolicerClassifyDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("policer_classify_dump_6bfe6603") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PolicerClassifyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
impl PolicerClassifyDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("policer_classify_details_dfd08765") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableIds { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ClassifyTableIds { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_ids_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableIdsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub ids : u32, 
} 
impl ClassifyTableIdsReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_ids_reply_d1d20e1d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableByInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl ClassifyTableByInterface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_by_interface_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableByInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
	pub l2_table_id : u32, 
	pub ip4_table_id : u32, 
	pub ip6_table_id : u32, 
} 
impl ClassifyTableByInterfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_by_interface_reply_ed4197db") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableInfo { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
} 
impl ClassifyTableInfo { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_info_0cca2cd9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTableInfoReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_id : u32, 
	pub nbuckets : u32, 
	pub match_n_vectors : u32, 
	pub skip_n_vectors : u32, 
	pub active_sessions : u32, 
	pub next_table_index : u32, 
	pub miss_next_index : u32, 
	pub mask_length : u32, 
	pub mask : u8, 
} 
impl ClassifyTableInfoReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_table_info_reply_4a573c0e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
} 
impl ClassifySessionDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_session_dump_0cca2cd9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySessionDetails { 
	pub context : u32, 
	pub retval : i32, 
	pub table_id : u32, 
	pub hit_next_index : u32, 
	pub advance : i32, 
	pub opaque_index : u32, 
	pub match_length : u32, 
	pub match : u8, 
} 
impl ClassifySessionDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_session_details_60e3ef94") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FlowClassifySetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub is_add : bool, 
} 
impl FlowClassifySetInterface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("flow_classify_set_interface_b6192f1c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FlowClassifySetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl FlowClassifySetInterfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("flow_classify_set_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FlowClassifyDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : FlowClassifyTable, 
	pub sw_if_index : InterfaceIndex, 
} 
impl FlowClassifyDump { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("flow_classify_dump_8a6ad43d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct FlowClassifyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
impl FlowClassifyDetails { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("flow_classify_details_dfd08765") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySetInterfaceIpTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_ipv6 : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
impl ClassifySetInterfaceIpTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_set_interface_ip_table_e0b097c7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySetInterfaceIpTableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl ClassifySetInterfaceIpTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_set_interface_ip_table_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySetInterfaceL2Tables { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub other_table_index : u32, 
	pub is_input : bool, 
} 
impl ClassifySetInterfaceL2Tables { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_set_interface_l2_tables_5a6ddf65") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifySetInterfaceL2TablesReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl ClassifySetInterfaceL2TablesReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_set_interface_l2_tables_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct InputAclSetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
impl InputAclSetInterface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("input_acl_set_interface_de7ad708") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct InputAclSetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl InputAclSetInterfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("input_acl_set_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OutputAclSetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
impl OutputAclSetInterface { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("output_acl_set_interface_de7ad708") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct OutputAclSetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl OutputAclSetInterfaceReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("output_acl_set_interface_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapLookupTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub mask_len : u32, 
	pub mask : u8, 
} 
impl ClassifyPcapLookupTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_lookup_table_e1b4cc6b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapLookupTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
impl ClassifyPcapLookupTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_lookup_table_reply_9c6c6773") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapSetTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
	pub sort_masks : bool, 
} 
impl ClassifyPcapSetTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_set_table_006051b3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapSetTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
impl ClassifyPcapSetTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_set_table_reply_9c6c6773") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapGetTables { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl ClassifyPcapGetTables { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_get_tables_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyPcapGetTablesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub indices : u32, 
} 
impl ClassifyPcapGetTablesReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_pcap_get_tables_reply_5f5bc9e6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceLookupTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub mask_len : u32, 
	pub mask : u8, 
} 
impl ClassifyTraceLookupTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_lookup_table_3f7b72e4") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceLookupTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
impl ClassifyTraceLookupTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_lookup_table_reply_9c6c6773") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceSetTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_index : u32, 
	pub sort_masks : bool, 
} 
impl ClassifyTraceSetTable { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_set_table_3909b55a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceSetTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
impl ClassifyTraceSetTableReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_set_table_reply_9c6c6773") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceGetTables { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl ClassifyTraceGetTables { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_get_tables_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ClassifyTraceGetTablesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub indices : u32, 
} 
impl ClassifyTraceGetTablesReply { 
	 pub fn get_message_name_and_crc() -> String { 
	 	 String::from("classify_trace_get_tables_reply_5f5bc9e6") 
	 } 
} 
