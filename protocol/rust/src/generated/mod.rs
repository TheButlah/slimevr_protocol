// Automatically generated by the Flatbuffers compiler. Do not modify.
// @generated
pub mod solarxr_protocol {
  use super::*;
  pub mod data_feed {
    use super::*;
    pub mod device_data {
      use super::*;
      mod device_data_mask_generated;
      pub use self::device_data_mask_generated::*;
      mod device_data_generated;
      pub use self::device_data_generated::*;
    } // device_data
    pub mod tracker {
      use super::*;
      mod tracker_data_generated;
      pub use self::tracker_data_generated::*;
      mod tracker_data_mask_generated;
      pub use self::tracker_data_mask_generated::*;
      mod tracker_info_generated;
      pub use self::tracker_info_generated::*;
    } // tracker
    mod data_feed_message_generated;
    pub use self::data_feed_message_generated::*;
    mod bone_generated;
    pub use self::bone_generated::*;
    mod data_feed_message_header_generated;
    pub use self::data_feed_message_header_generated::*;
    mod poll_data_feed_generated;
    pub use self::poll_data_feed_generated::*;
    mod start_data_feed_generated;
    pub use self::start_data_feed_generated::*;
    mod data_feed_update_generated;
    pub use self::data_feed_update_generated::*;
    mod data_feed_config_generated;
    pub use self::data_feed_config_generated::*;
  } // data_feed
  pub mod datatypes {
    use super::*;
    pub mod hardware_info {
      use super::*;
      mod mcu_type_generated;
      pub use self::mcu_type_generated::*;
      mod imu_type_generated;
      pub use self::imu_type_generated::*;
      mod hardware_address_generated;
      pub use self::hardware_address_generated::*;
      mod hardware_info_generated;
      pub use self::hardware_info_generated::*;
      mod hardware_status_generated;
      pub use self::hardware_status_generated::*;
      mod firmware_status_mask_generated;
      pub use self::firmware_status_mask_generated::*;
    } // hardware_info
    pub mod math {
      use super::*;
      mod quat_generated;
      pub use self::quat_generated::*;
      mod vec_3f_generated;
      pub use self::vec_3f_generated::*;
    } // math
    mod firmware_error_code_generated;
    pub use self::firmware_error_code_generated::*;
    mod filtering_type_generated;
    pub use self::filtering_type_generated::*;
    mod tracker_role_generated;
    pub use self::tracker_role_generated::*;
    mod body_part_generated;
    pub use self::body_part_generated::*;
    mod tracker_status_generated;
    pub use self::tracker_status_generated::*;
    mod hz_f32_generated;
    pub use self::hz_f32_generated::*;
    mod transaction_id_generated;
    pub use self::transaction_id_generated::*;
    mod device_id_generated;
    pub use self::device_id_generated::*;
    mod tracker_id_generated;
    pub use self::tracker_id_generated::*;
    mod log_data_generated;
    pub use self::log_data_generated::*;
    mod temperature_generated;
    pub use self::temperature_generated::*;
    mod ipv_4_address_generated;
    pub use self::ipv_4_address_generated::*;
    mod bytes_generated;
    pub use self::bytes_generated::*;
    mod string_table_generated;
    pub use self::string_table_generated::*;
  } // datatypes
  pub mod pub_sub {
    use super::*;
    mod topic_generated;
    pub use self::topic_generated::*;
    mod pub_sub_union_generated;
    pub use self::pub_sub_union_generated::*;
    mod payload_generated;
    pub use self::payload_generated::*;
    mod topic_id_generated;
    pub use self::topic_id_generated::*;
    mod topic_handle_generated;
    pub use self::topic_handle_generated::*;
    mod topic_mapping_generated;
    pub use self::topic_mapping_generated::*;
    mod topic_handle_request_generated;
    pub use self::topic_handle_request_generated::*;
    mod subscription_request_generated;
    pub use self::subscription_request_generated::*;
    mod pub_sub_header_generated;
    pub use self::pub_sub_header_generated::*;
    mod message_generated;
    pub use self::message_generated::*;
    mod key_values_generated;
    pub use self::key_values_generated::*;
  } // pub_sub
  pub mod rpc {
    use super::*;
    pub mod settings {
      use super::*;
      mod model_toggles_generated;
      pub use self::model_toggles_generated::*;
      mod model_ratios_generated;
      pub use self::model_ratios_generated::*;
      mod leg_tweaks_settings_generated;
      pub use self::leg_tweaks_settings_generated::*;
      mod model_settings_generated;
      pub use self::model_settings_generated::*;
    } // settings
    mod rpc_message_generated;
    pub use self::rpc_message_generated::*;
    mod reset_type_generated;
    pub use self::reset_type_generated::*;
    mod reset_status_generated;
    pub use self::reset_status_generated::*;
    mod arms_mounting_reset_mode_generated;
    pub use self::arms_mounting_reset_mode_generated::*;
    mod skeleton_bone_generated;
    pub use self::skeleton_bone_generated::*;
    mod wifi_provisioning_status_generated;
    pub use self::wifi_provisioning_status_generated::*;
    mod auto_bone_process_type_generated;
    pub use self::auto_bone_process_type_generated::*;
    mod status_data_generated;
    pub use self::status_data_generated::*;
    mod computer_directory_generated;
    pub use self::computer_directory_generated::*;
    mod rpc_message_header_generated;
    pub use self::rpc_message_header_generated::*;
    mod heartbeat_request_generated;
    pub use self::heartbeat_request_generated::*;
    mod heartbeat_response_generated;
    pub use self::heartbeat_response_generated::*;
    mod reset_request_generated;
    pub use self::reset_request_generated::*;
    mod reset_response_generated;
    pub use self::reset_response_generated::*;
    mod assign_tracker_request_generated;
    pub use self::assign_tracker_request_generated::*;
    mod clear_drift_compensation_request_generated;
    pub use self::clear_drift_compensation_request_generated::*;
    mod settings_request_generated;
    pub use self::settings_request_generated::*;
    mod settings_response_generated;
    pub use self::settings_response_generated::*;
    mod change_settings_request_generated;
    pub use self::change_settings_request_generated::*;
    mod steam_vrtrackers_setting_generated;
    pub use self::steam_vrtrackers_setting_generated::*;
    mod filtering_settings_generated;
    pub use self::filtering_settings_generated::*;
    mod drift_compensation_settings_generated;
    pub use self::drift_compensation_settings_generated::*;
    mod oscrouter_settings_generated;
    pub use self::oscrouter_settings_generated::*;
    mod vrcoscsettings_generated;
    pub use self::vrcoscsettings_generated::*;
    mod vmcoscsettings_generated;
    pub use self::vmcoscsettings_generated::*;
    mod oscsettings_generated;
    pub use self::oscsettings_generated::*;
    mod osctrackers_setting_generated;
    pub use self::osctrackers_setting_generated::*;
    mod tap_detection_settings_generated;
    pub use self::tap_detection_settings_generated::*;
    mod resets_settings_generated;
    pub use self::resets_settings_generated::*;
    mod tap_detection_setup_notification_generated;
    pub use self::tap_detection_setup_notification_generated::*;
    mod record_bvhrequest_generated;
    pub use self::record_bvhrequest_generated::*;
    mod record_bvhstatus_generated;
    pub use self::record_bvhstatus_generated::*;
    mod skeleton_part_generated;
    pub use self::skeleton_part_generated::*;
    mod skeleton_config_request_generated;
    pub use self::skeleton_config_request_generated::*;
    mod skeleton_config_response_generated;
    pub use self::skeleton_config_response_generated::*;
    mod skeleton_reset_all_request_generated;
    pub use self::skeleton_reset_all_request_generated::*;
    mod change_skeleton_config_request_generated;
    pub use self::change_skeleton_config_request_generated::*;
    mod serial_device_generated;
    pub use self::serial_device_generated::*;
    mod open_serial_request_generated;
    pub use self::open_serial_request_generated::*;
    mod close_serial_request_generated;
    pub use self::close_serial_request_generated::*;
    mod set_wifi_request_generated;
    pub use self::set_wifi_request_generated::*;
    mod serial_update_response_generated;
    pub use self::serial_update_response_generated::*;
    mod serial_tracker_reboot_request_generated;
    pub use self::serial_tracker_reboot_request_generated::*;
    mod serial_tracker_get_info_request_generated;
    pub use self::serial_tracker_get_info_request_generated::*;
    mod serial_tracker_factory_reset_request_generated;
    pub use self::serial_tracker_factory_reset_request_generated::*;
    mod serial_devices_request_generated;
    pub use self::serial_devices_request_generated::*;
    mod serial_devices_response_generated;
    pub use self::serial_devices_response_generated::*;
    mod new_serial_device_response_generated;
    pub use self::new_serial_device_response_generated::*;
    mod start_wifi_provisioning_request_generated;
    pub use self::start_wifi_provisioning_request_generated::*;
    mod stop_wifi_provisioning_request_generated;
    pub use self::stop_wifi_provisioning_request_generated::*;
    mod wifi_provisioning_status_response_generated;
    pub use self::wifi_provisioning_status_response_generated::*;
    mod auto_bone_process_request_generated;
    pub use self::auto_bone_process_request_generated::*;
    mod auto_bone_process_status_response_generated;
    pub use self::auto_bone_process_status_response_generated::*;
    mod auto_bone_epoch_response_generated;
    pub use self::auto_bone_epoch_response_generated::*;
    mod auto_bone_settings_generated;
    pub use self::auto_bone_settings_generated::*;
    mod height_request_generated;
    pub use self::height_request_generated::*;
    mod height_response_generated;
    pub use self::height_response_generated::*;
    mod auto_bone_apply_request_generated;
    pub use self::auto_bone_apply_request_generated::*;
    mod auto_bone_stop_recording_request_generated;
    pub use self::auto_bone_stop_recording_request_generated::*;
    mod auto_bone_cancel_recording_request_generated;
    pub use self::auto_bone_cancel_recording_request_generated::*;
    mod overlay_display_mode_request_generated;
    pub use self::overlay_display_mode_request_generated::*;
    mod overlay_display_mode_change_request_generated;
    pub use self::overlay_display_mode_change_request_generated::*;
    mod overlay_display_mode_response_generated;
    pub use self::overlay_display_mode_response_generated::*;
    mod server_infos_request_generated;
    pub use self::server_infos_request_generated::*;
    mod server_infos_response_generated;
    pub use self::server_infos_response_generated::*;
    mod leg_tweaks_tmp_change_generated;
    pub use self::leg_tweaks_tmp_change_generated::*;
    mod leg_tweaks_tmp_clear_generated;
    pub use self::leg_tweaks_tmp_clear_generated::*;
    mod status_tracker_reset_generated;
    pub use self::status_tracker_reset_generated::*;
    mod status_tracker_error_generated;
    pub use self::status_tracker_error_generated::*;
    mod status_steam_vrdisconnected_generated;
    pub use self::status_steam_vrdisconnected_generated::*;
    mod status_system_request_generated;
    pub use self::status_system_request_generated::*;
    mod status_system_response_generated;
    pub use self::status_system_response_generated::*;
    mod status_system_update_generated;
    pub use self::status_system_update_generated::*;
    mod status_system_fixed_generated;
    pub use self::status_system_fixed_generated::*;
    mod status_message_generated;
    pub use self::status_message_generated::*;
    mod set_pause_tracking_request_generated;
    pub use self::set_pause_tracking_request_generated::*;
    mod clear_mounting_reset_request_generated;
    pub use self::clear_mounting_reset_request_generated::*;
    mod save_file_notification_generated;
    pub use self::save_file_notification_generated::*;
    mod tracking_pause_state_request_generated;
    pub use self::tracking_pause_state_request_generated::*;
    mod tracking_pause_state_response_generated;
    pub use self::tracking_pause_state_response_generated::*;
    mod unknown_device_handshake_notification_generated;
    pub use self::unknown_device_handshake_notification_generated::*;
    mod add_unknown_device_request_generated;
    pub use self::add_unknown_device_request_generated::*;
    mod forget_device_request_generated;
    pub use self::forget_device_request_generated::*;
  } // rpc
  mod message_bundle_generated;
  pub use self::message_bundle_generated::*;
} // solarxr_protocol
