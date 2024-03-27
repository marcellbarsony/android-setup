extern crate adb_client;

pub mod setup {
    use adb_client::AdbTcpConnection;
    use std::net::Ipv4Addr;

    pub fn settings() {
        let vectors: Vec<Vec<&str>> = vec![
            // {{{ NETWORK
            // WiFi
            vec!["svc", "wifi", "enable"],
            // Bluetooth
            vec!["cmd", "bluetooth_manager", "disable"],
            vec!["settings", "put", "global", "bluetooth_cast_mode", "0"],
            vec!["settings", "put", "global", "bluetooth_name", "marci"],
            // NFC
            vec!["svc", "nfc", "disable"],
            // Ultra-wideband (UWB)
            vec!["settings", "put", "global", "uwb_enabled", "0"],
            // Airplane
            vec!["settings", "put", "global", "airplane_mode_on", "0"],
            vec!["settings", "put", "global", "airplane_mode_radios", "cell,bluetooth,uwb,wifi,wimax"],
            vec!["settings", "put", "global", "airplane_mode_toggleable_radios", "bluetooth,wifi"],
            // DNS [TODO]
            // vec!["settings", "put", "global", "private_dns_mode", "hostname"],
            // vec!["settings", "put", "global", "private_dns_specifier", "dns.adguard.com"],
            // Data roaming
            vec!["settings", "put", "global", "data_roaming", "1"],
            // More connection settings
            // Nearby device scanning
            vec!["settings", "put", "system", "nearby_scanning_enabled", "0"],
            // }}}

            // {{{ SOUNDS
            // Touch interactions
            vec!["settings", "put", "system", "sound_effects_enabled", "0"],
            // Dialing keypad
            vec!["settings", "put", "system", "dtmf_tone", "0"],
            // Samsung Keyboard
            vec!["settings", "put", "system", "sip_key_feedback_sound", "0"],
            // Charging
            vec!["settings", "put", "secure", "charging_sounds_enabled", "0"],
            // Lockscreen sounds
            vec!["settings", "put", "system", "lockscreen_sounds_enabled", "0"],
            // Use Volume buttons for media
            vec!["settings", "put", "system", "adjust_media_volume_only", "1"],
            // Audio volume warning
            vec!["settings", "put", "global", "audio_safe_volume_state", "0"],
            // Multi audio focus
            vec!["settings", "put", "system", "multi_audio_focus_enabled", "1"],
            // }}}

            // {{{ VIBRATION
            // Sytem vibration
            vec!["settings", "put", "system", "VIB_FEEDBACK_MAGNITUDE", "0"],
            // Call vibration
            vec!["settings", "put", "system", "VIB_RECVCALL_MAGNITUDE", "5"],
            // Touch interactions
            vec!["settings", "put", "system", "haptic_feedback_enabled", "0"],
            // Dialing keypad
            vec!["settings", "put", "system", "dialing_keypad_vibrate", "0"],
            // Samsung Keyboard
            vec!["settings", "put", "system", "sip_key_feedback_vibration", "0"],
            // Charging
            vec!["settings", "put", "secure", "charging_vibration_enabled", "0"],
            // Navigation gestures
            vec!["settings", "put", "system", "navigation_gestures_vibrate", "0"],
            // Camera feedback
            vec!["settings", "put", "system", "camera_feedback_vibrate", "0"],
            // Vibration sound for incoming calls
            vec!["settings", "put", "system", "vibration_sound_enabled", "0"],
            // }}}

            // {{{ NOTIFICATIONS
            // Sort notifications
            vec!["settings", "put", "global", "notification_sort_order", "0"],
            // Lock screen notifications
            vec!["settings", "put", "secure", "lock_screen_show_notifications", "0"],
            vec!["settings", "put", "secure", "lock_screen_allow_private_notifications", "0"],
            vec!["settings", "put", "secure", "lock_screen_allow_private_notifications_when_unsecure", "1"],
            vec!["settings", "put", "secure", "lock_screen_show_silent_notifications", "0"],
            // Notification popup style (Edge lighting)
            vec!["settings", "put", "system", "edge_lighting", "1"],
            vec!["settings", "put", "system", "edge_lighting_style_type_str", "preload/eclipse"],
            vec!["settings", "put", "system", "edge_lighting_basic_color_index", "3"],
            vec!["settings", "put", "system", "edge_lighting_basic_color_type", "0"],
            vec!["settings", "put", "system", "edge_lighting_show_condition", "0"],
            vec!["settings", "put", "system", "edge_lighting_duration", "1"],
            vec!["settings", "put", "system", "edge_lighting_transparency", "30"],
            // Do not disturb
            vec!["settings", "put", "global", "zen_mode", "0"],
            // Floating notifications
            vec!["settings", "put", "secure", "notification_bubbles", "0"],
            // Show app icon in notifications
            vec!["settings", "put", "system", "show_notification_app_icon", "1"],
            // Suggest actions and replies for notifications
            // TODO
            // Show snooze button
            vec!["settings", "put", "secure", "show_notification_snooze", "1"],
            // Repeat notification alerts
            vec!["settings", "put", "system", "notification_reminder_selectable", "1"],
            // App icon badges
            vec!["settings", "put", "secure", "notification_badging", "0"],
            // Manage notefication categories for each app
            vec!["settings", "put", "secure", "show_notification_category_setting", "1"],
            // Wireless emergency alerts
            // TODO
            // Find my mobile
            vec!["settings", "put", "system", "fmm_notification_other", "0"],
            // }}}

            // {{{ DISPLAY
            // Dark mode
            vec!["cmd", "uimode", "night", "yes"],
            vec!["settings", "put", "system", "display_night_theme", "1"],
            vec!["settings", "put", "system", "display_night_theme_scheduled", "0"],
            // Adaptive brightness
            vec!["settings", "put", "system", "screen_brightness_mode", "0"],
            vec!["settings", "put", "system", "shown_max_brightness_dialog", "1"],
            // Extra brightness
            vec!["settings", "put", "secure", "screen_extra_brightness", "0"],
            // Motion smoothness
            vec!["settings", "put", "secure", "refresh_rate_mode", "1"],
            // Eye comfort shield
            vec!["settings", "put", "system", "blue_light_filter", "1"],
            vec!["settings", "put", "system", "blue_light_filter_adaptive_mode", "0"],
            vec!["settings", "put", "system", "blue_light_filter_night_dim", "0"],
            vec!["settings", "put", "system", "blue_light_filter_scheduled", "0"],
            vec!["settings", "put", "system", "blue_light_filter_type", "0"],
            vec!["settings", "put", "system", "blue_light_filter_opacity", "8"],
            // Adaptive color tone
            vec!["settings", "put", "system", "ead_enabled", "0"],
            // Screen mode
            vec!["settings", "put", "system", "screen_mode_setting", "4"],
            // Font size and style
            vec!["settings", "put", "global", "flip_font_style", "3"],
            vec!["settings", "put", "global", "font_size", "0"],
            vec!["settings", "put", "system", "font_scale", "1"],
            // Screen zoom [TODO]
            // vec!["settings", "put", "secure", "display_density_forced", "280"],
            // Easy mode
            vec!["settings", "put", "system", "easy_mode_switch", "1"],
            // Edge panel
            vec!["settings", "put", "secure", "edge_enable", "1"],
            vec!["settings", "put", "secure", "edge_handle_size_percent", "20"],
            vec!["settings", "put", "secure", "edge_handle_transparency", "100"],
            vec!["settings", "put", "global", "edge_panel_height", "2000"],
            vec!["settings", "put", "global", "edge_panel_width", "410"],
            vec!["settings", "put", "global", "edge_setting_copyed_for_mum", "1"],
            vec!["settings", "put", "system", "active_edge_area", "1"],
            vec!["settings", "put", "system", "edge_handler_position_percent", "13"],
            // Screen timeout (ms)
            vec!["settings", "put", "system", "screen_off_timeout", "600000"], // TODO (60000)
            // Navigation bar
            vec!["cmd", "overlay", "enable", "com.android.internal.systemui.navbar.gestural"],
            vec!["settings", "put", "global", "navigation_bar_button_to_hide_keyboard", "0"],
            vec!["settings", "put", "global", "navigation_bar_gesture_hint", "1"],
            vec!["settings", "put", "global", "navigation_bar_gesture_while_hidden", "1"],
            vec!["settings", "put", "global", "navigationbar_switch_apps_when_hint_hidden", "1"],
            vec!["settings", "put", "global", "bottom_gesture_inset_scale", "1.0"],
            vec!["settings", "put", "secure", "back_gesture_inset_scale_left", "1.0"],
            vec!["settings", "put", "secure", "back_gesture_inset_scale_right", "1.0"],
            vec!["settings", "put", "secure", "touch_and_hold_to_search", "1"],
            // Accidental touch protection
            vec!["settings", "put", "system", "screen_off_pocket", "0"],
            // Touch sensitivity
            vec!["settings", "put", "system", "auto_adjust_touch", "0"],
            // Show charging information
            vec!["settings", "put", "system", "charging_info_always", "1"],
            // Screen saver
            vec!["settings", "put", "secure", "screensaver_enabled", "0"],
            vec!["settings", "put", "secure", "screensaver_components", "null"],
            vec!["settings", "put", "secure", "screensaver_activate_on_dock", "1"],
            vec!["settings", "put", "secure", "screensaver_activate_on_sleep", "0"],
            // Show recommendsd apps
            // vec!["settings", "put", "system", "show_recent_apps", "1"],
            vec!["settings", "put", "secure", "task_changer_key_suggested_apps", "0"],
            vec!["settings", "put", "secure", "task_changer_key_mini_mode", "0"],
            // Burn in protection
            vec!["settings", "put", "global", "burn_in_protection", "1"],
            // }}}

            // {{{ BATTERY
            // Battery protection
            vec!["settings", "put", "global", "protect_battery", "1"],
            vec!["settings", "put", "global", "prev_protect_battery", "-1"],
            vec!["settings", "put", "global", "battery_protection_threshold", "85"],
            vec!["settings", "put", "global", "battery_protection_recharge_level", "95"],
            // Charging settings
            vec!["settings", "put", "system", "charging_info_always", "1"],
            vec!["settings", "put", "system", "adaptive_fast_charging", "0"],
            vec!["settings", "put", "system", "wireless_fast_charging", "0"],
            // Show battery percentage
            vec!["settings", "put", "system", "display_battery_percentage", "1"],
            // }}}

            // {{{ WALLPAPER AND STYLE
            // Notification details (lock screen)
            vec!["settings", "put", "system", "lockscreen_minimizing_notification", "0"],
            vec!["settings", "put", "system", "lock_noticard_opacity", "50"],
            vec!["settings", "put", "system", "lock_noticard_opacity_dark_mode", "50"],
            // Dim wallpaper when Dark mode is on
            vec!["settings", "put", "system", "display_night_theme_wallpaper", "1"],
            // Apply palette to app icons
            vec!["settings", "put", "global", "colortheme_app_icon", "0"],
            // }}}

            // {{{ LOCK SCREEN
            // Contact information (Wallpapers and style)
            vec!["settings", "put", "secure", "lock_screen_owner_info_enabled", "1"],
            // Enable notifications
            vec!["settings", "put", "secure", "lock_screen_show_notifications", "1"],
            // TODO
            // adb shell settings put system charging_info_always 1
            // adb shell settings put system add_info_music_control 1
            // }}}

            // {{{ AOD
            // Enable
            vec!["settings", "put", "system", "aod_mode", "1"],
            // Show lock screen wallpaper
            vec!["settings", "put", "system", "aod_show_lockscreen_wallpaper", "0"],
            // When to show
            vec!["settings", "put", "system", "aod_tap_to_show_mode", "1"],
            // Charging mode
            vec!["settings", "put", "system", "aod_charging_mode", "1"],
            vec!["settings", "put", "system", "aod_show_state", "0"],
            // }}}

            // {{{ SECURITY AND PRIVACY
            // Biometrics
            vec!["settings", "put", "secure", "fingerprint_always_on", "0"],
            vec!["settings", "put", "secure", "fingerprint_on_screen_tips", "1"],
            vec!["settings", "put", "secure", "fingerprint_screen_lock", "1"],
            vec!["settings", "put", "secure", "fingerprint_screen_off_icon_aod", "2"],
            vec!["settings", "put", "secure", "fingerprint_settings_create", "0"],
            vec!["settings", "put", "system", "fingerprint_guide_shown", "1"],
            // More security settings
            // Make passwords visible
            vec!["settings", "put", "system", "show_password", "0"],
            vec!["settings", "put", "system", "send_security_reports", "0"],
            // More privacy settings
            // Customization Service on this phone
            vec!["settings", "put", "global", "dbsc_consent_customized_service_agree_value", "false"],
            // Get news and special offers on this phone
            vec!["settings", "put", "global", "dbsc_consent_marketing_agree_value", "false"],
            // Improve personalized ads based on data from this phone
            vec!["settings", "put", "global", "dbsc_consent_marketing_data_processing_agree_value", "false"],
            // Send diagnostic data
            vec!["settings", "put", "system", "samsung_errorlog_agree", "0"],
            // Android personalization service
            vec!["settings", "put", "secure", "content_capture_enabled", "0"],
            vec!["settings", "put", "global", "content_capture_service_explicitly_enabled", "default"],
            // Usage & diagnostics
            vec!["settings", "put", "global", "multi_cb", "0"],
            // Secure folder
            vec!["settings", "put", "secure", "hide_secure_folder_flag", "0"],
            // SPAM Calls
            vec!["settings", "put", "system", "block_unwanted_call", "1"],
            vec!["settings", "put", "system", "block_unwanted_call_type", "1"],
            vec!["settings", "put", "global", "spam_call_enable", "1"],
            vec!["settings", "put", "global", "spam_call_mute_first_ring", "null"],
            // }}}

            // {{{ LOCATION
            vec!["settings", "put", "secure", "location_mode", "1"],
            vec!["settings", "put", "secure", "location_changer", "0"],
            vec!["settings", "put", "global", "network_recommendations_enabled", "0"],
            // Location secvices
            vec!["settings", "put", "global", "ble_scan_always_enabled", "0"],
            vec!["settings", "put", "global", "wifi_scan_always_enabled", "0"],
            // }}}

            // {{{ APPS
            // Digital assistant apps
            vec!["settings", "put", "secure", "assistant", "null"],
            vec!["settings", "put", "secure", "voice_interaction_service", "null"],
            vec!["settings", "put", "secure", "assist_screenshot_enabled", "0"],
            vec!["settings", "put", "secure", "assist_structure_enabled", "0"],
            // }}}

            // {{{ SAFETY AND EMERGENCY
            vec!["settings", "put", "global", "lock_screen_medical_info_access", "1"],
            // }}}

            // {{{ SAMSUNG
            // EULA
            vec!["settings", "put", "system", "samsung_eula_agree", "0"],
            vec!["settings", "put", "system", "samsung_eula_agree_hqm", "0"],
            // Bixby permissions
            vec!["settings", "put", "global", "bixby_pregranted_permissions", "0"],
            // Quick settings
            vec!["settings", "put", "secure", "brightness_on_top", "1"],
            vec!["settings", "put", "global", "swipe_directly_to_quick_setting", "1"],
            vec!["settings", "put", "global", "swipe_directly_to_quick_setting_area", "75"],
            vec!["settings", "put", "global", "swipe_directly_to_quick_setting_position", "right"],
            // }}}

            // {{{ GOOGLE
            // GMS tweak
            vec!["settings", "put", "global", "google_core_control", "0"],
            // {{{ TODO
            // https://www.macrodroidforum.com/index.php?threads/disable-google-high-accuracy-with-secure-settings.2497/
            // https://www.reddit.com/r/tasker/comments/b154a6/location_modes_on_android_9/
            vec!["settings", "put", "global", "assisted_gps_enabled", "0"],
            // TODO
            // adb shell settings put secure default_voice_input_method 0
            // adb shell settings put secure smartspace 0
            // adb shell settings put secure voice_recognition_service 0
            // adb shell settings put global hotword_detection_enabled 0
            // }}}
            // }}}

            // {{{ ADVANCED FEATURES
            // Labs
            vec!["settings", "put", "global", "force_resizable_activities", "1"],
            vec!["settings", "put", "global", "photo_ambient_wallpaper_enable", "0"],
            // Side button
            vec!["settings", "put", "global", "side_button", "1"],
            vec!["settings", "put", "global", "function_key_config_doublepress_type", "0"],
            vec!["settings", "put", "global", "function_key_config_longpress_type", "1"],
            vec!["settings", "put", "global", "power_button_long_press", "1"],
            // Multi window
            vec!["settings", "put", "global", "freeform_corner_gesture_level", "2"],
            vec!["settings", "put", "global", "freeform_handler_help_popup_count", "1"],
            vec!["settings", "put", "global", "multi_split_quick_options_help_count", "1"],
            vec!["settings", "put", "global", "open_in_pop_up_view", "1"],
            vec!["settings", "put", "global", "open_in_split_screen_view", "1"],
            // Motions and gestures
            vec!["settings", "put", "system", "lift_to_wake", "0"],
            vec!["settings", "put", "system", "double_tab_to_wake_up", "1"],
            vec!["settings", "put", "system", "double_tap_to_sleep", "1"],
            vec!["settings", "put", "system", "motion_pick_up", "1"],
            vec!["settings", "put", "system", "motion_merged_mute_pause", "1"],
            vec!["settings", "put", "system", "motion_overturn", "1"],
            vec!["settings", "put", "system", "surface_palm_touch", "1"],
            vec!["settings", "put", "system", "surface_palm_swipe", "0"],
            // One-handed mode
            vec!["settings", "put", "system", "any_screen_enabled", "0"],
            vec!["settings", "put", "system", "any_screen_running", "0"],
            // Screenshots and screen recordings
            vec!["settings", "put", "system", "enable_smart_capture", "1"],
            vec!["settings", "put", "system", "delete_shared_screenshots", "0"],
            vec!["settings", "put", "system", "exclude_systemui_screenshot", "0"],
            vec!["settings", "put", "system", "save_original_screenshots", "1"],
            vec!["settings", "put", "system", "smart_capture_screenshot_format", "PNG"],
            vec!["settings", "put", "global", "video_quality", "1080"],
            // Show contacts when sharing content
            vec!["settings", "put", "system", "direct_share", "0"],
            // Video brightness
            vec!["settings", "put", "system", "hdr_effect", "0"],
            // }}}

            // {{{ GENERAL MANAGEMENT
            // Language
            vec!["settings", "put", "system", "system_locales", "en-US"],
            // Date and time
            vec!["settings", "put", "global", "auto_time", "1"],
            vec!["settings", "put", "global", "auto_time_zone", "1"],
            vec!["settings", "put", "secure", "location_time_zone_detection_enabled", "0"],
            vec!["settings", "put", "system", "time_12_24", "12"],
            // Passwords, passkeys, and autofill
            vec!["settings", "put", "secure", "autofill_service", "null"],
            vec!["settings", "put", "secure", "credential_service", "com.samsung.android.samsungpassautofill/com.samsung.android.samsungpassautofill.CredentialManagerService"],
            vec!["settings", "put", "secure", "credential_service_primary", ""],
            // Keyboard
            vec!["settings", "put", "secure", "key_layout_type", "1"],
            vec!["settings", "put", "secure", "key_toggle_plugin", "1"],
            vec!["settings", "put", "secure", "show_ime_with_hard_keyboard", "0"],
            // Keyboard list and default
            vec!["settings", "put", "secure", "enabled_input_methods", "com.samsung.android.honeyboard/.service.HoneyBoardService;65537"],
            vec!["settings", "put", "secure", "show_keyboard_button", "0"],
            // }}}

            // {{{ SOFTWARE UPDATE
            vec!["settings", "put", "system", "SOFTWARE_UPDATE_WIFI_ONLY2", "1"],
            // }}}

            // {{{ REMOTE SUPPORT
            vec!["settings", "put", "global", "online_manual_url", "0"],
            vec!["settings", "put", "system", "remote_control", "0"],
            // }}}

            // {{{ ABOUT PHONE
            // Device name
            vec!["settings", "put", "global", "device_name", "marci"],
            vec!["settings", "put", "global", "default_device_name", "marci"],
            vec!["settings", "put", "global", "synced_account_name", "marci"],
            vec!["settings", "put", "global", "bug_report", "0"],
            // }}}

            // {{{ DEVELOPER OTIONS
            // Drawing
            vec!["settings", "put", "global", "animator_duration_scale", "0.5"],
            vec!["settings", "put", "global", "transition_animation_scale", "0.5"],
            vec!["settings", "put", "global", "window_animation_scale", "0.5"],
            // }}}

            // {{{ MISC
            // Accelerometer rotation
            vec!["settings", "put", "system", "accelerometer_rotation", "1"],
            // Zram
            vec!["settings", "put", "global", "zram_enabled", "0"],
            // }}}
        ];

        adb_shell(vectors);
    }

    pub fn adb_shell(vectors: Vec<Vec<&str>>) {
        let mut connection = AdbTcpConnection::new(Ipv4Addr::from([127, 0, 0, 1]), 5037).unwrap();

        for vector in vectors.iter() {
            connection.shell_command(&None, vector).unwrap();
            println!("[+] {:?}", vector);
        }
    }
}
