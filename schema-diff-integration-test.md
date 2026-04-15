# Schema Diff Report

**Generated:** 2026-04-15 17:55:41

**Base:**  `192.168.35.15:48007`

**Check:** `192.168.35.140:65035`

---

## Summary

- **Databases compared:** 17
- **Databases only in base:** 0, only in check: 1, changed: 12

## Database: `billing-mars`

### Tables

#### ⚠ Changed: `t_subject_biz_consumption_detail`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `idx_dev_sn` | ✖ only in check |  |  (dev_sn) [BTREE] |

#### ⚠ Changed: `t_subject_resource_owned`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `idx_resource_effect` | ✖ only in check |  |  (subject_id, res_effect_time, res_expire_time) [BTREE] |

## Database: `datacenter-mars`

### Tables

#### ⚠ Changed: `t_battery_info`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_DEV_SN_OID` | ✖ only in check |  |  (oid, dev_sn) [BTREE] |

#### ⚠ Changed: `t_device_online_hour_detail1`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `idx_sn_hour` | ✖ only in check |  |  (dev_sn, online_hour) [BTREE] |
| `t_device_online_hour_detail1_dev_sn_IDX` | ✚ only in base |  (dev_sn, online_hour) [BTREE] |  |

#### ⚠ Changed: `t_exception_info`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `exception_id` | ⚠ changed | bigint NOT NULL auto_increment | bigint NOT NULL DEFAULT 0 |

#### ⚠ Changed: `t_statistic_app0`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app1`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app10`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app11`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app12`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app13`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app14`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app15`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app2`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app3`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app4`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app5`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app6`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app7`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app8`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app9`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_collection_date` | ✖ only in check |  |  (collection_date) [BTREE] |

#### ⚠ Changed: `t_statistic_app_daily`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_OID_SUMMARY_DATE` | ✖ only in check |  |  (oid, summary_date) [BTREE] |
| `i_oid` | ✖ only in check |  |  (oid) [BTREE] |

#### ✚ Only in base: `t_statistic_app_month`

| Column | Type |
|--------|------|
| `statistic_app_month_id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `merchant_id` | bigint NULL |
| `reseller_id` | bigint NULL |
| `os` | varchar(32) NOT NULL |
| `model_code` | varchar(16) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `app_usage` | bigint NOT NULL DEFAULT 0 |
| `cellular_type` | varchar(8) NULL |
| `traffic_cellular` | bigint NOT NULL DEFAULT 0 |
| `traffic_wifi` | bigint NOT NULL DEFAULT 0 |
| `traffic_lan` | bigint NOT NULL DEFAULT 0 |
| `install_count` | bigint NOT NULL DEFAULT 0 |
| `update_count` | bigint NOT NULL DEFAULT 0 |
| `summary_month` | bigint NOT NULL |
| `accumulate_date` | varchar(8) NULL |

#### ⚠ Changed: `t_statistic_dev_daily`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_oid` | ✖ only in check |  |  (oid) [BTREE] |

#### ⚠ Changed: `t_statistic_operator_daily`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `IDX_OID_SUMMARY_DATE` | ✖ only in check |  |  (oid, summary_date) [BTREE] |

#### ⚠ Changed: `t_statistic_operator_merchant_daily`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `IDX_OID` | ✖ only in check |  |  (oid) [BTREE] |

#### ⚠ Changed: `t_statistic_organization_hour`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `idx_operator_org_hour` | ✖ only in check |  | UNIQUE (operator_id, org_id, summary_hour) [BTREE] |

#### ⚠ Changed: `t_status_device`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `adb_enabled` | ⚠ changed | tinyint NULL | tinyint NULL |
| `admin_app` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `basebank_version` | ⚠ changed | varchar(32) NULL DEFAULT  | varchar(32) NULL DEFAULT  |
| `battery_available` | ⚠ changed | varchar(3) NULL DEFAULT 0 | varchar(3) NULL DEFAULT 0 |
| `battery_cycles` | ⚠ changed | bigint NULL | bigint NULL |
| `battery_health` | ⚠ changed | int NULL | int NULL |
| `battery_operation_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `battery_production_date` | ⚠ changed | varchar(16) NULL | varchar(16) NULL |
| `battery_quality` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `battery_replace_date` | ⚠ changed | bigint NULL | bigint NULL |
| `battery_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `battery_verified_state` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `bluetooth_status` | ⚠ changed | varchar(3) NULL DEFAULT  | varchar(3) NULL DEFAULT  |
| `build_manufacturer` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `build_serial` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `camera_back_model` | ⚠ changed | varchar(12) NULL DEFAULT  | varchar(12) NULL DEFAULT  |
| `camera_front_model` | ⚠ changed | varchar(12) NULL DEFAULT  | varchar(12) NULL DEFAULT  |
| `camera_opened_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `charge_status` | ⚠ changed | tinyint NULL | tinyint NULL |
| `contact_count` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `contactless_count` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `customer_id` | ⚠ changed | varchar(16) NULL DEFAULT  | varchar(16) NULL DEFAULT  |
| `default_apn` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `developer_mode` | ⚠ changed | tinyint NULL | tinyint NULL |
| `device_lock_status` | ⚠ changed | varchar(3) NULL DEFAULT  | varchar(3) NULL DEFAULT  |
| `device_manufacturer` | ⚠ changed | varchar(12) NULL DEFAULT  | varchar(12) NULL DEFAULT  |
| `device_model` | ⚠ changed | varchar(16) NULL DEFAULT  | varchar(16) NULL DEFAULT  |
| `device_name` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `device_on_time` | ⚠ changed | bigint NULL | bigint NULL |
| `device_on_time_str` | ⚠ changed | varchar(16) NULL | varchar(16) NULL |
| `device_screen_timeout` | ⚠ changed | int NULL DEFAULT -1 | int NULL DEFAULT -1 |
| `device_up_time` | ⚠ changed | varchar(12) NULL | varchar(12) NULL |
| `gps_status` | ⚠ changed | tinyint NULL | tinyint NULL |
| `hardware_attack_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `hardware_attack_time` | ⚠ changed | bigint NULL | bigint NULL |
| `hardware_config` | ⚠ changed | varchar(132) NULL DEFAULT  | varchar(132) NULL DEFAULT  |
| `hardware_id` | ⚠ changed | varchar(12) NULL DEFAULT  | varchar(12) NULL DEFAULT  |
| `imei` | ⚠ changed | varchar(64) NULL DEFAULT  | varchar(64) NULL DEFAULT  |
| `imei2` | ⚠ changed | varchar(32) NULL DEFAULT  | varchar(32) NULL DEFAULT  |
| `last_valid_gps_time` | ⚠ changed | bigint NULL | bigint NULL |
| `lat_lng` | ⚠ changed | varchar(25) NULL | varchar(25) NULL |
| `location_id` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `location_source` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `network_ipv4` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `network_type` | ⚠ changed | int NULL | int NULL |
| `os_version` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `powerbutton_pressed_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `print_counts` | ✖ only in check |  | bigint NULL |
| `printer_status` | ⚠ changed | int NULL | tinyint NULL |
| `root_status` | ⚠ changed | tinyint NULL | tinyint NULL |
| `screen_touched_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `secure_app_version` | ⚠ changed | varchar(18) NULL DEFAULT  | varchar(18) NULL DEFAULT  |
| `secure_boot_version` | ⚠ changed | varchar(20) NULL DEFAULT  | varchar(20) NULL DEFAULT  |
| `secure_fw_version` | ⚠ changed | varchar(32) NULL DEFAULT  | varchar(32) NULL DEFAULT  |
| `security_patch` | ⚠ changed | varchar(16) NULL | varchar(16) NULL |
| `security_tamper_reason` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `security_tamper_status` | ⚠ changed | tinyint NULL | tinyint NULL |
| `sim_card_sn` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `sim_card_sn2` | ⚠ changed | varchar(32) NULL | varchar(64) NULL |
| `sim_num` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `sim_num2` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `sim_operator_id` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `sim_operator_id2` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `sim_signal_strength` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `sim_signal_strength2` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `sim_slot_status` | ⚠ changed | varchar(8) NULL | varchar(8) NULL |
| `sim_slot_status2` | ⚠ changed | varchar(4) NULL | varchar(4) NULL |
| `software_attack_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `software_attack_time` | ⚠ changed | bigint NULL | bigint NULL |
| `ssid` | ⚠ changed | varchar(256) NULL DEFAULT  | varchar(256) NULL DEFAULT  |
| `storage_available` | ⚠ changed | bigint NULL DEFAULT 0 | bigint NULL DEFAULT 0 |
| `storage_brush_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `storage_total` | ⚠ changed | bigint NULL DEFAULT 0 | bigint NULL DEFAULT 0 |
| `swipe_count` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `sys_fw_id` | ⚠ changed | varchar(12) NULL DEFAULT  | varchar(12) NULL DEFAULT  |
| `time_zone` | ⚠ changed | varchar(16) NULL DEFAULT  | varchar(16) NULL DEFAULT  |
| `usb_operation_counts` | ⚠ changed | int NULL DEFAULT 0 | int NULL DEFAULT 0 |
| `wifi_connected_ip` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `wifi_connected_mac` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `wifi_status` | ⚠ changed | varchar(3) NULL DEFAULT  | varchar(3) NULL DEFAULT  |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `idx_oid_upd_epochsecond` | ⚠ changed |  (oid, upd_epochsecond) [BTREE] |  (upd_epochsecond, oid) [BTREE] |

## Database: `developer-mars`

### Tables

#### ⚠ Changed: `t_application`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `app_icon_url` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `appraise_count` | ⚠ changed | int NULL | int NULL |
| `auto_fit` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `bankcard_app` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `download_count` | ⚠ changed | bigint unsigned NOT NULL DEFAULT 0 | bigint unsigned NOT NULL DEFAULT 0 |
| `high_quality` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `is_private_app` | ⚠ changed | tinyint(1) NULL DEFAULT 0 | tinyint(1) NOT NULL DEFAULT 0 |
| `landscape` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `main_language` | ⚠ changed | varchar(8) NOT NULL | varchar(8) NOT NULL |
| `org_id` | ⚠ changed | bigint NULL DEFAULT 0 | bigint NULL DEFAULT 0 |
| `portrait` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `release_area` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `score_average` | ⚠ changed | decimal(4,2) NULL | decimal(4,2) NULL |
| `status` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `support_language` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `trial_days` | ⚠ changed | tinyint unsigned NOT NULL DEFAULT 0 | tinyint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `u_package_name` | ⚠ changed | UNIQUE (character_code, key_id, org_id, package_name) [BTREE] |  (character_code, key_id, org_id, package_name) [BTREE] |

#### ⚠ Changed: `t_application_model`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(128) NULL | varchar(256) NULL |

#### ⚠ Changed: `t_application_version`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `apk_digest` | ⚠ changed | varchar(1024) NULL | varchar(1024) NULL |
| `apk_md5` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `apk_sign` | ⚠ changed | varchar(512) NULL | varchar(512) NULL |
| `apk_sign_time` | ⚠ changed | bigint NULL | bigint NULL |
| `apk_signer` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `category_id` | ⚠ changed | bigint NULL | bigint NULL |
| `cert_cn` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `cert_sha1` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `cert_sha256` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `detection_report` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `detection_time` | ⚠ changed | bigint NULL | bigint NULL |
| `ori_app_version_id` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `release_area` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `sign_ukey` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `suspend_reason` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `test_account_info` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |

#### ⚠ Changed: `t_category`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `character_code` | ⚠ changed | varchar(12) NOT NULL | varchar(16) NOT NULL |
| `status` | ⚠ changed | tinyint NOT NULL | tinyint(1) NOT NULL DEFAULT 1 |

#### ⚠ Changed: `t_developer_device_apply`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `reject_reason` | ⚠ changed | varchar(256) NULL | varchar(128) NULL |

#### ⚠ Changed: `t_directional_app_version`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `apk_digest` | ⚠ changed | varchar(1024) NULL | varchar(1024) NULL |
| `apk_md5` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `apk_sign` | ⚠ changed | varchar(512) NULL | varchar(512) NULL |
| `apk_sign_time` | ⚠ changed | bigint NULL | bigint NULL |
| `apk_signer` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `category_id` | ⚠ changed | bigint NULL | bigint NULL |
| `cert_cn` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `cert_sha1` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `cert_sha256` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `detection_report` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `detection_time` | ⚠ changed | bigint NULL | bigint NULL |
| `ori_app_version_id` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `release_area` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `sign_ukey` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `suspend_reason` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `test_account_info` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |

#### ⚠ Changed: `t_directional_application`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `app_icon_url` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `appraise_count` | ⚠ changed | int NULL | int NULL |
| `auto_fit` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `bankcard_app` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `download_count` | ⚠ changed | bigint unsigned NOT NULL DEFAULT 0 | bigint unsigned NOT NULL DEFAULT 0 |
| `high_quality` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `landscape` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `main_language` | ⚠ changed | varchar(8) NOT NULL | varchar(8) NOT NULL |
| `org_id` | ⚠ changed | bigint NULL DEFAULT 0 | bigint NULL DEFAULT 0 |
| `portrait` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `release_area` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `score_average` | ⚠ changed | decimal(4,2) NULL | decimal(4,2) NULL |
| `status` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `support_language` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `trial_days` | ⚠ changed | tinyint unsigned NOT NULL DEFAULT 0 | tinyint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `u_package_name` | ⚠ changed | UNIQUE (character_code, key_id, org_id, package_name) [BTREE] |  (character_code, key_id, org_id, package_name) [BTREE] |

## Database: `fly-parameter`

### Tables

#### ⚠ Changed: `t_param_batch_operation_device`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_TARGET_ID` | ✚ only in base |  (target_id) [BTREE] |  |

#### ⚠ Changed: `t_param_value0`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value1`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value10`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value11`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value12`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value13`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value14`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value15`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value2`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value3`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value4`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value5`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value6`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value7`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value8`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_param_value9`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_REFERENCE` | ✖ only in check |  |  (value_type, ref_table_schema_id, ref_table_entity_value) [BTREE] |

#### ⚠ Changed: `t_temporary_data`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `batch_no` | ⚠ changed | varchar(32) NOT NULL | varchar(32) NOT NULL |
| `character_code` | ✖ only in check |  | varchar(16) NOT NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `key_id` | ✖ only in check |  | bigint NOT NULL |
| `plat_code` | ✖ only in check |  | varchar(32) NOT NULL |
| `row_unique_key` | ⚠ changed | varchar(32) NOT NULL | varchar(32) NOT NULL |

#### ✚ Only in base: `temp_operator`

| Column | Type |
|--------|------|
| `operator_id` | bigint NOT NULL |
| `oid` | varchar(64) NULL |

### Functions

#### ⚠ Changed: `queryChildTableRelations`

**Base Definition:**

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd =cast(parentTableSchemaId as CHAR); WHILE sTempChd is not null DO SET sTemp = concat(sTemp,',',sTempChd); SELECT group_concat(table_schema_id) INTO sTempChd FROM t_table_schema_relation where param_template_id = paramTemplateId and FIND_IN_SET(parent_table_schema_id,sTempChd)>0; END WHILE; RETURN sTemp; END
```

**Check Definition:**

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd =cast(parentTableSchemaId as CHAR); WHILE sTempChd is not null DO SET sTemp = concat(sTemp,',',sTempChd); SELECT group_concat(table_schema_id) INTO sTempChd FROM t_table_schema_relation where param_template_id = paramTemplateId and FIND_IN_SET(parent_table_schema_id,sTempChd)>0; END WHILE; RETURN sTemp; END
```

#### ⚠ Changed: `queryParentTableRelations`

**Base Definition:**

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd = cast(tableSchemaId as CHAR); WHILE sTempChd is not null DO SET sTemp = concat(sTemp,',',sTempChd); SELECT group_concat(parent_table_schema_id) INTO sTempChd FROM t_table_schema_relation WHERE param_template_id = paramTemplateId and FIND_IN_SET(table_schema_id,sTempChd)>0; END WHILE; RETURN sTemp; END
```

**Check Definition:**

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd = cast(tableSchemaId as CHAR); WHILE sTempChd is not null DO SET sTemp = concat(sTemp,',',sTempChd); SELECT group_concat(parent_table_schema_id) INTO sTempChd FROM t_table_schema_relation WHERE param_template_id = paramTemplateId and FIND_IN_SET(table_schema_id,sTempChd)>0; END WHILE; RETURN sTemp; END
```

## Database: `flyiot`

### Tables

#### ✖ Only in check: `t_device_bind`

| Column | Type |
|--------|------|
| `dev_bind_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `host_dev_sn` | varchar(32) NOT NULL |
| `dev_sn` | varchar(32) NULL |
| `dev_label` | varchar(32) NOT NULL |
| `dev_model_code` | varchar(32) NOT NULL |
| `dev_logo_url` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | int NOT NULL |

#### ⚠ Changed: `t_message_device`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CHARACTER_CODE` | ✚ only in base |  (character_code, key_id) [BTREE] |  |
| `I_CRE_TIME_CHARACTER` | ✚ only in base |  (cre_time, key_id, character_code) [BTREE] |  |
| `I_ORG_CHARACTER` | ✚ only in base |  (org_id, character_code, key_id) [BTREE] |  |
| `I_ORG_CRE_TIME` | ✖ only in check |  |  (org_id, cre_time) [BTREE] |

#### ✖ Only in check: `t_subscribe_terminal`

| Column | Type |
|--------|------|
| `subscribe_uuid` | varchar(32) NOT NULL |
| `bus_type` | varchar(16) NOT NULL |
| `father_object` | varchar(32) NOT NULL |
| `subscriber` | varchar(32) NOT NULL |
| `services` | varchar(2048) NOT NULL |
| `subscribe_time` | bigint NOT NULL |
| `upd_time` | binary(12) NULL |

## Database: `gcluster-mars`

### Tables

#### ✚ Only in base: `t_nginx_server`

| Column | Type |
|--------|------|
| `nginx_id` | varchar(32) NOT NULL |
| `nginx_name` | varchar(64) NULL |
| `nginx_outer_ip` | varchar(50) NULL |
| `nginx_inner_ip` | varchar(16) NULL |
| `ws_port` | int NULL |
| `nginx_port` | int NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `domain` | varchar(128) NULL |
| `max_connections` | int NOT NULL DEFAULT 200000 |
| `recommend` | int NOT NULL DEFAULT 1 |

#### ⚠ Changed: `t_node_server`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `max_connections` | ⚠ changed | int NOT NULL DEFAULT 60000 | int NOT NULL DEFAULT 62000 |
| `nginx_id` | ✚ only in base | varchar(32) NOT NULL DEFAULT  |  |

## Database: `merchant-mars`

### Tables

#### ⚠ Changed: `t_function_config`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `U_FUNCTION_CONFIG` | ✖ only in check |  | UNIQUE (param_key, param_value, mrch_id) [BTREE] |

#### ✚ Only in base: `t_order0`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order1`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order10`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order11`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order12`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order13`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order14`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order15`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order2`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order3`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order4`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order5`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order6`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order7`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order8`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order9`

| Column | Type |
|--------|------|
| `order_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `mid` | varchar(32) NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `dev_tid` | varchar(32) NULL |
| `app_name` | varchar(256) NULL |
| `user_id` | bigint NOT NULL |
| `cashier` | varchar(128) NULL |
| `order_no` | varchar(64) NOT NULL |
| `amount_total` | bigint NOT NULL |
| `amount_ori` | bigint NULL |
| `amount_tip_percent` | int NULL |
| `amount_tip` | bigint NULL |
| `amount_tax_percent` | int NULL |
| `amount_tax` | bigint NULL |
| `amount_other` | bigint NULL |
| `amount_discount` | bigint NULL |
| `amount_discount_percent` | int NULL |
| `currency_Code` | varchar(12) NOT NULL |
| `refund_balance` | bigint NOT NULL |
| `order_description` | varchar(1024) NULL |
| `order_status` | tinyint NOT NULL |
| `schedule_task_flag` | tinyint(1) NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order_goods0`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods1`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods10`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods11`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods12`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods13`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods14`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods15`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods2`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods3`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods4`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods5`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods6`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods7`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods8`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_order_goods9`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `service_duration_time` | int NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_receipt_image0`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image1`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image10`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image11`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image12`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image13`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image14`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image15`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image2`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image3`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image4`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image5`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image6`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image7`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image8`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_image9`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_token0`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token1`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token10`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token11`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token12`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token13`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token14`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token15`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token2`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token3`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token4`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token5`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token6`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token7`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token8`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_token9`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `type` | int NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ⚠ Changed: `t_statistics_day3`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_UTC` | ✚ only in base |  (time_utc, key_id) [BTREE] |  |

#### ✚ Only in base: `t_statistics_day4`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_day5`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_day6`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_day7`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_day8`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_day9`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `statistic_day` | int NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(8) NOT NULL |

#### ✚ Only in base: `t_statistics_key_status_test`

| Column | Type |
|--------|------|
| `statistics_key_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ⚠ Changed: `t_statistics_month0`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CHARACTER` | ✚ only in base |  (key_id, statistic_month, key_code, org_id, mrch_id) [BTREE] |  |

#### ✚ Only in base: `t_statistics_month1`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month10`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month11`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month12`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month13`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month14`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month15`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month2`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month3`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month4`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month5`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month6`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month7`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month8`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_statistics_month9`

| Column | Type |
|--------|------|
| `statistics_uuid` | varchar(32) NOT NULL |
| `statistic_year` | int NULL |
| `statistic_month` | int NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `mrch_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `app_id` | varchar(128) NOT NULL |
| `key_code` | varchar(36) NOT NULL |
| `key_value` | decimal(14,4) NULL |
| `record_value` | decimal(14,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `time_utc` | varchar(6) NOT NULL |

#### ✚ Only in base: `t_transaction0`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction1`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction10`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction11`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction12`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction13`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction14`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction15`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction2`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction3`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction4`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction5`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction6`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction7`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction8`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction9`

| Column | Type |
|--------|------|
| `trans_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `source_id` | varchar(32) NOT NULL |
| `mid` | varchar(32) NULL |
| `tid` | varchar(32) NULL |
| `app_name` | varchar(128) NULL |
| `order_no` | varchar(32) NOT NULL |
| `trans_token` | varchar(32) NOT NULL |
| `dev_uuid` | varchar(32) NULL |
| `payment_method` | varchar(16) NOT NULL |
| `payment_card` | varchar(16) NULL |
| `payment_platform` | varchar(32) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `country_code` | varchar(3) NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `aid` | varchar(32) NULL |
| `tvr` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `trans_time_local` | bigint NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `error_description` | varchar(1024) NULL |
| `e_receipt_flag` | tinyint(1) NOT NULL |
| `e_receipt_image_upload` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(32) NULL |
| `ori_trans_token` | varchar(32) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction_statistic0`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic1`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic10`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic11`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic12`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic13`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic14`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic15`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic2`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic3`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic4`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic5`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic6`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic7`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic8`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction_statistic9`

| Column | Type |
|--------|------|
| `statistic_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `trans_type` | varchar(64) NOT NULL |
| `channel_mid` | varchar(100) NOT NULL |
| `sum_amount_total` | bigint NOT NULL |
| `sum_trans_count` | bigint NOT NULL |
| `average_trans_amount` | bigint NOT NULL |
| `payment_method` | varchar(45) NOT NULL |
| `read_card_type` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `trans_month` | int NULL |
| `trans_date` | int NOT NULL |
| `statistic_utc` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

## Database: `message-center-mars`

### Tables

#### ✚ Only in base: `t_terminal_message_target`

| Column | Type |
|--------|------|
| `terminal_msg_target_id` | bigint NOT NULL auto_increment |
| `terminal_msg_id` | bigint NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `send_time` | bigint NOT NULL |
| `reply_content` | varchar(512) NULL |
| `dev_msg_status` | int NOT NULL |
| `send_fail_reason` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `reply_time` | bigint NULL |

## ✖ Database only in check: `nacos`

### Tables

#### ✖ Only in check: `roles`

| Column | Type |
|--------|------|
| `username` | varchar(50) NOT NULL |
| `role` | varchar(50) NOT NULL |

#### ✖ Only in check: `his_config_info`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL |
| `nid` | bigint unsigned NOT NULL auto_increment |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(128) NOT NULL |
| `app_name` | varchar(128) NULL |
| `content` | longtext NOT NULL |
| `md5` | varchar(32) NULL |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `src_user` | text NULL |
| `src_ip` | varchar(50) NULL |
| `op_type` | char(10) NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |

#### ✖ Only in check: `config_tags_relation`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL |
| `tag_name` | varchar(128) NOT NULL |
| `tag_type` | varchar(64) NULL |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(128) NOT NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |
| `nid` | bigint NOT NULL auto_increment |

#### ✖ Only in check: `config_info`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(255) NULL |
| `content` | longtext NOT NULL |
| `md5` | varchar(32) NULL |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `src_user` | text NULL |
| `src_ip` | varchar(50) NULL |
| `app_name` | varchar(128) NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |
| `c_desc` | varchar(256) NULL |
| `c_use` | varchar(64) NULL |
| `effect` | varchar(64) NULL |
| `type` | varchar(64) NULL |
| `c_schema` | text NULL |

#### ✖ Only in check: `permissions`

| Column | Type |
|--------|------|
| `role` | varchar(50) NOT NULL |
| `resource` | varchar(255) NOT NULL |
| `action` | varchar(8) NOT NULL |

#### ✖ Only in check: `config_info_tag`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(128) NOT NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |
| `tag_id` | varchar(128) NOT NULL |
| `app_name` | varchar(128) NULL |
| `content` | longtext NOT NULL |
| `md5` | varchar(32) NULL |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `src_user` | text NULL |
| `src_ip` | varchar(50) NULL |

#### ✖ Only in check: `config_info_beta`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(128) NOT NULL |
| `app_name` | varchar(128) NULL |
| `content` | longtext NOT NULL |
| `beta_ips` | varchar(1024) NULL |
| `md5` | varchar(32) NULL |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `src_user` | text NULL |
| `src_ip` | varchar(50) NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |

#### ✖ Only in check: `config_info_aggr`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `data_id` | varchar(255) NOT NULL |
| `group_id` | varchar(255) NOT NULL |
| `datum_id` | varchar(255) NOT NULL |
| `content` | longtext NOT NULL |
| `gmt_modified` | datetime NOT NULL |
| `app_name` | varchar(128) NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |

#### ✖ Only in check: `tenant_info`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `kp` | varchar(128) NOT NULL |
| `tenant_id` | varchar(128) NULL DEFAULT  |
| `tenant_name` | varchar(128) NULL DEFAULT  |
| `tenant_desc` | varchar(256) NULL |
| `create_source` | varchar(32) NULL |
| `gmt_create` | bigint NOT NULL |
| `gmt_modified` | bigint NOT NULL |

#### ✖ Only in check: `tenant_capacity`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `tenant_id` | varchar(128) NOT NULL DEFAULT  |
| `quota` | int unsigned NOT NULL DEFAULT 0 |
| `usage` | int unsigned NOT NULL DEFAULT 0 |
| `max_size` | int unsigned NOT NULL DEFAULT 0 |
| `max_aggr_count` | int unsigned NOT NULL DEFAULT 0 |
| `max_aggr_size` | int unsigned NOT NULL DEFAULT 0 |
| `max_history_count` | int unsigned NOT NULL DEFAULT 0 |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |

#### ✖ Only in check: `users`

| Column | Type |
|--------|------|
| `username` | varchar(50) NOT NULL |
| `password` | varchar(500) NOT NULL |
| `enabled` | tinyint(1) NOT NULL |

#### ✖ Only in check: `group_capacity`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `group_id` | varchar(128) NOT NULL DEFAULT  |
| `quota` | int unsigned NOT NULL DEFAULT 0 |
| `usage` | int unsigned NOT NULL DEFAULT 0 |
| `max_size` | int unsigned NOT NULL DEFAULT 0 |
| `max_aggr_count` | int unsigned NOT NULL DEFAULT 0 |
| `max_aggr_size` | int unsigned NOT NULL DEFAULT 0 |
| `max_history_count` | int unsigned NOT NULL DEFAULT 0 |
| `gmt_create` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |
| `gmt_modified` | datetime NOT NULL DEFAULT CURRENT_TIMESTAMP DEFAULT_GENERATED |

## Database: `opennl-authorization`

### Tables

#### ⚠ Changed: `t_country`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `continent` | ⚠ changed | char(3) NULL | char(3) NULL |
| `default_lang` | ⚠ changed | varchar(8) NULL | varchar(8) NULL |
| `default_utc_time` | ⚠ changed | char(9) NULL | char(9) NULL |
| `status` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |

#### ✚ Only in base: `t_doc_center_config`

| Column | Type |
|--------|------|
| `id` | int NOT NULL auto_increment |
| `group_code` | varchar(16) NOT NULL |
| `cate_code` | varchar(16) NOT NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ⚠ Changed: `t_event`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `event_desc` | ⚠ changed | varchar(512) NULL | varchar(256) NULL |

#### ⚠ Changed: `t_operator`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `address` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `area_code` | ⚠ changed | varchar(8) NULL | varchar(8) NULL |
| `area_name` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `character_code` | ✖ only in check |  | varchar(16) NULL |
| `city_name` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `contract_expire_time` | ⚠ changed | bigint NULL | bigint NULL |
| `country_code` | ⚠ changed | char(3) NOT NULL | char(3) NOT NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `cre_user_id` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `email` | ⚠ changed | varchar(128) NOT NULL | varchar(128) NOT NULL |
| `expire_time` | ⚠ changed | bigint NULL | bigint NULL |
| `level_code` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `license` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `linkman` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `map_key` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `map_token` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `map_type` | ⚠ changed | varchar(16) NULL | varchar(16) NULL |
| `mobile` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `name` | ⚠ changed | varchar(256) NOT NULL | varchar(256) NOT NULL |
| `oid` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `operator_type` | ⚠ changed | varchar(16) NOT NULL | varchar(16) NOT NULL |
| `prov_name` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `region_code` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `remark` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `short_name` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `status` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `upd_user_id` | ⚠ changed | bigint NULL | bigint NULL |
| `utc_time` | ⚠ changed | varchar(6) NULL | varchar(6) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `operator_no_UNIQUE` | ✚ only in base | UNIQUE (operator_no) [BTREE] |  |

#### ⚠ Changed: `t_password_reset`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `error_times` | ⚠ changed | int NOT NULL | int NOT NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

#### ✖ Only in check: `t_sso_config`

| Column | Type |
|--------|------|
| `sso_config_id` | bigint NOT NULL auto_increment |
| `sso_plat_code` | varchar(64) NULL |
| `org_name` | varchar(128) NULL |
| `domain` | varchar(256) NULL |
| `client_id` | varchar(64) NULL |
| `client_secret` | varchar(512) NULL |
| `redirect_uri` | varchar(512) NULL |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `status` | int NULL |
| `cre_time` | bigint NULL |
| `cre_user_id` | bigint NULL |
| `upd_time` | bigint NULL |
| `upd_user_id` | bigint NULL |

#### ✖ Only in check: `t_sso_role`

| Column | Type |
|--------|------|
| `sso_role_id` | bigint NOT NULL auto_increment |
| `sso_plat_code` | varchar(64) NULL |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `locale_role_id` | bigint NULL |
| `sso_role_code` | varchar(256) NULL |
| `status` | int NULL |
| `cre_time` | bigint NULL |
| `cre_user_id` | bigint NULL |

#### ✖ Only in check: `t_sso_user`

| Column | Type |
|--------|------|
| `sso_user_id` | bigint NOT NULL auto_increment |
| `sso_plat_code` | varchar(64) NULL |
| `user_id` | bigint NULL |
| `sso_user_sub` | varchar(128) NULL |
| `cre_time` | bigint NULL |

#### ⚠ Changed: `t_user`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `address` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `country_code` | ⚠ changed | char(3) NULL | char(3) NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `email` | ⚠ changed | varchar(128) NOT NULL | varchar(128) NOT NULL |
| `gender` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `logo_url` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `mobile` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `password_modify_time` | ⚠ changed | bigint NULL | bigint NULL |
| `remark` | ⚠ changed | varchar(1024) NULL | varchar(1024) NULL |
| `status` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

## Database: `remote-command-mars`

### Tables

#### ⚠ Changed: `t_cmd`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD_TYPE_CODE` | ✚ only in base |  (cmd_type_code) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device0`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device1`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device10`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device11`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device12`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device13`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device14`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device15`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device2`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device3`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device4`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device5`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device6`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device7`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device8`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

#### ⚠ Changed: `t_cmd_device9`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `err_msg` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `username` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CMD` | ✚ only in base |  (cmd_id) [BTREE] |  |

## Database: `statistic-mars`

### Tables

#### ⚠ Changed: `t_chart_mode`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NULL | bigint NULL |
| `legend_position` | ⚠ changed | tinyint NULL | tinyint NULL |
| `table_split` | ⚠ changed | tinyint NULL | tinyint NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

#### ✚ Only in base: `t_statistic_upgrade_fail`

| Column | Type |
|--------|------|
| `statistic_upgrade_fail_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NULL |
| `org_id` | bigint NULL |
| `fail_type` | varchar(12) NULL |
| `statistic_day` | int NULL |
| `statistic_month` | int NULL |
| `fail_count` | bigint NULL |

#### ✚ Only in base: `t_upgrade_percentile_summary`

| Column | Type |
|--------|------|
| `upgrade_pecentile_summary_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |
| `cmd_code` | varchar(64) NOT NULL |
| `upgrade_method` | varchar(32) NOT NULL |
| `event_source_id` | varchar(64) NULL |
| `percentile` | varchar(16) NULL |
| `completion_percentage` | int NULL |
| `usage_time` | bigint NULL |
| `task_start_time` | bigint NULL |
| `task_end_time` | bigint NULL |
| `total_count` | bigint NOT NULL |
| `processed_terminal_count` | varchar(100) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `event_type` | tinyint NULL |

#### ✚ Only in base: `t_upgrade_task`

| Column | Type |
|--------|------|
| `upgrade_task_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |
| `cmd_code` | varchar(64) NOT NULL |
| `upgrade_method` | varchar(32) NOT NULL |
| `total_counts` | bigint NOT NULL |
| `task_start_time` | bigint NOT NULL |
| `task_cre_time` | bigint NOT NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `task_name` | varchar(128) NULL |
| `upgrade_task_uuid` | varchar(64) NULL |

#### ✚ Only in base: `t_upgrade_task_complete`

| Column | Type |
|--------|------|
| `upgrade_task_complete_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NULL |
| `oid` | varchar(64) NULL |
| `org_id` | bigint NULL |
| `upgrade_task_id` | varchar(64) NULL |
| `cmd_code` | varchar(64) NULL |
| `upgrade_method` | varchar(32) NULL |
| `time_point` | varchar(16) NULL |
| `percent_done` | int NULL |
| `done_count` | bigint NULL |
| `total_count` | bigint NULL |
| `task_time_point_value` | bigint NULL |
| `online_count` | bigint NULL |
| `processed_terminal_count` | bigint NULL |
| `task_start_date` | bigint NULL |
| `task_name` | varchar(128) NULL |

#### ✚ Only in base: `t_upgrade_task_trend`

| Column | Type |
|--------|------|
| `upgrade_task_id` | bigint NOT NULL auto_increment |
| `upgrade_method` | varchar(32) NULL |
| `event_source_id` | varchar(64) NULL |
| `cmd_code` | varchar(64) NULL |
| `app_file_id` | bigint NULL |
| `network_type` | tinyint NULL |
| `file_size` | bigint NULL |
| `os` | varchar(32) NULL |
| `model_code` | varchar(32) NULL |
| `success_count` | bigint NULL |
| `fail_count` | bigint NULL |
| `org_id` | bigint NULL |
| `total_use_time` | varchar(100) NULL |
| `file_download_time` | bigint NULL |
| `statistic_day` | int NULL |
| `operator_id` | bigint NULL |
| `oid` | varchar(64) NULL |
| `statistic_month` | int NULL |
| `total_count` | bigint NULL |
| `file_size_bucket` | varchar(32) NULL |
| `total_file_size` | decimal(10,0) NULL |
| `download_retries` | bigint NULL |

## Database: `toms-mars`

### Tables

#### ⚠ Changed: `t_advance_upg_target`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_RESULT` | ✖ only in check |  |  (upg_result) [BTREE] |
| `I_UPD_TIME` | ✖ only in check |  |  (upd_time) [BTREE] |

#### ⚠ Changed: `t_advance_upg_task`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `remark` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `upd_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |

#### ⚠ Changed: `t_app_device_model`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(128) NOT NULL | varchar(256) NOT NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_APPFILEIF` | ✖ only in check |  |  (app_file_id) [BTREE] |
| `U_APPFILEID_MODEL_CODE` | ✖ only in check |  | UNIQUE (app_file_id, model_code) [BTREE] |

#### ⚠ Changed: `t_app_file`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `file_upload_time` | ⚠ changed | bigint NULL | bigint NULL |
| `manufacturer_name` | ⚠ changed | varchar(128) NULL | varchar(256) NULL |
| `remark` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `shared_user_id` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `status` | ⚠ changed | tinyint unsigned NULL | tinyint unsigned NULL |
| `support_extend` | ⚠ changed | tinyint NOT NULL DEFAULT 0 | tinyint NOT NULL DEFAULT 0 |
| `sw_flag` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `version_name` | ⚠ changed | varchar(64) NULL | varchar(32) NULL |
| `version_style` | ⚠ changed | tinyint NOT NULL DEFAULT 1 | tinyint NOT NULL DEFAULT 1 |

#### ⚠ Changed: `t_app_file_lastest`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `file_upload_time` | ⚠ changed | bigint NULL | bigint NULL |
| `manufacturer_name` | ⚠ changed | varchar(128) NULL | varchar(256) NULL |
| `remark` | ⚠ changed | varchar(256) NULL | varchar(256) NULL |
| `shared_user_id` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `status` | ⚠ changed | tinyint unsigned NULL | tinyint unsigned NULL |
| `support_extend` | ⚠ changed | tinyint NOT NULL DEFAULT 0 | tinyint NOT NULL DEFAULT 0 |
| `sw_flag` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `version_name` | ⚠ changed | varchar(64) NULL | varchar(32) NULL |
| `version_style` | ⚠ changed | tinyint NOT NULL DEFAULT 1 | tinyint NOT NULL DEFAULT 1 |

#### ⚠ Changed: `t_batch_operation`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `i_batch_id` | ✖ only in check |  |  (batch_id) [BTREE] |

#### ⚠ Changed: `t_character_device_model`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(128) NULL | varchar(256) NULL |

#### ✖ Only in check: `t_common_device_batch`

| Column | Type |
|--------|------|
| `common_device_batch_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `qr_code` | varchar(64) NOT NULL |
| `total_amount` | bigint NOT NULL |
| `available_amount` | bigint NOT NULL |
| `batch_status` | tinyint NOT NULL DEFAULT 1 |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✖ Only in check: `t_common_device_batch_detail`

| Column | Type |
|--------|------|
| `batch_detail_id` | bigint NOT NULL auto_increment |
| `common_device_batch_id` | bigint NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `model_code` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |

#### ⚠ Changed: `t_deployment_model`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(120) NOT NULL | varchar(256) NOT NULL |

#### ⚠ Changed: `t_device`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(120) NULL | varchar(256) NULL |

#### ⚠ Changed: `t_device_app_param`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_OID_SN` | ✚ only in base |  (oid, dev_sn) [BTREE] |  |

#### ✖ Only in check: `t_device_app_tid`

| Column | Type |
|--------|------|
| `dev_app_tid_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `package_name` | varchar(128) NOT NULL |
| `dev_tid` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ⚠ Changed: `t_device_bind`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `cre_type` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `dev_mid` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `device_flag` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `geo_trace_flag` | ⚠ changed | tinyint(1) NULL | tinyint(1) NULL |
| `pre_status` | ⚠ changed | tinyint NULL | tinyint NULL |
| `status` | ⚠ changed | tinyint NOT NULL | tinyint NOT NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_AGENT_ID` | ✚ only in base |  (agent_id) [BTREE] |  |

#### ⚠ Changed: `t_device_cert`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CRE_TIME` | ✖ only in check |  |  (cre_time) [BTREE] |

#### ✚ Only in base: `t_device_download_event`

| Column | Type |
|--------|------|
| `dev_download_event_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `agent_id` | bigint NULL |
| `merchant_id` | bigint NULL |
| `model_code` | varchar(16) NOT NULL |
| `device_flag` | varchar(32) NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `dev_ksn` | varchar(32) NULL |
| `dev_tid` | varchar(32) NULL |
| `event_type` | tinyint NOT NULL |
| `description` | varchar(256) NULL |
| `event_start_time` | bigint NOT NULL |
| `event_status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `user_id` | bigint NULL |
| `operator` | varchar(64) NOT NULL |
| `event_source` | varchar(32) NULL |
| `event_key_id` | varchar(64) NULL |
| `target1` | varchar(128) NOT NULL DEFAULT  |
| `event_duration` | int NOT NULL DEFAULT 0 |
| `target2` | varchar(128) NULL |
| `target3` | varchar(128) NULL |
| `network_type` | tinyint NULL |
| `cmd_code` | varchar(64) NULL |
| `task_start_time` | bigint NULL |
| `download_retries` | int NULL |
| `event_end_time` | bigint NULL |

#### ✚ Only in base: `t_device_error`

| Column | Type |
|--------|------|
| `dev_error_id` | bigint NOT NULL auto_increment |
| `info` | varchar(512) NULL |
| `message` | varchar(512) NULL |
| `url` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |

#### ⚠ Changed: `t_device_event`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `event_duration` | ⚠ changed | int NOT NULL DEFAULT 0 | int NOT NULL DEFAULT 0 |
| `event_key_id` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `event_source` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |
| `target1` | ⚠ changed | varchar(128) NOT NULL DEFAULT  | varchar(128) NOT NULL DEFAULT  |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_DEV_TID` | ✖ only in check |  |  (operator_id, dev_tid) [BTREE] |
| `I_EVENT_TIME` | ✖ only in check |  |  (event_time) [BTREE] |
| `I_EVENT_TIME_OPERATOR` | ✖ only in check |  |  (operator_id, event_time) [BTREE] |
| `I_OPERATOR_EVENT_TYPE` | ✖ only in check |  |  (operator_id, event_type, event_time) [BTREE] |
| `I_OPERATOR_ID_EVENT_TYPE_TARGET1_TARGET3` | ✖ only in check |  |  (operator_id, event_type, target1, target3) [BTREE] |
| `I_OPERATOR_TYPE` | ✖ only in check |  |  (operator_id, event_type) [BTREE] |

#### ⚠ Changed: `t_device_event0`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event1`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event10`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event11`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event12`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event13`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event14`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event15`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event2`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event3`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event4`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event5`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event6`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event7`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event8`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_event9`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `download_retries` | ✚ only in base | int NULL |  |
| `external_sn` | ⚠ changed | varchar(32) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_device_hardware`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_DEV_SN` | ✖ only in check |  |  (dev_sn) [BTREE] |

#### ⚠ Changed: `t_device_info_app`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_MODULE_TYPE_CURRENT_VERSION` | ✖ only in check |  |  (module_type, model_code, current_version) [BTREE] |

#### ⚠ Changed: `t_device_language`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `U_DEVICE_FLAGE_MODEL_CODE_VALUE` | ✖ only in check |  | UNIQUE (model_code, device_flag, value) [BTREE] |
| `uidx_device_flage_model_code_value` | ✚ only in base | UNIQUE (device_flag, model_code, value) [BTREE] |  |

#### ⚠ Changed: `t_device_model`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(128) NOT NULL | varchar(256) NOT NULL |

#### ⚠ Changed: `t_device_schedule`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `begin_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `end_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `statistic_fail_num` | ⚠ changed | int NULL | int NULL |
| `statistic_suc_num` | ⚠ changed | int NULL | int NULL |
| `status` | ⚠ changed | tinyint(1) NULL DEFAULT 0 | tinyint(1) NULL DEFAULT 0 |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |
| `user_timezone` | ⚠ changed | varchar(8) NULL | varchar(8) NULL |

#### ⚠ Changed: `t_device_sign_error`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_DEV_SN` | ✖ only in check |  |  (dev_sn) [BTREE] |
| `ux_dev_sn` | ✚ only in base | UNIQUE (dev_sn) [BTREE] |  |

#### ⚠ Changed: `t_estate_batch`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_KEY_CHARACTER` | ✖ only in check |  |  (key_id, character_code) [BTREE] |
| `I_ORG` | ✖ only in check |  |  (org_id) [BTREE] |

#### ⚠ Changed: `t_flykey_device`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `inject_key_flag` | ⚠ changed | tinyint NULL DEFAULT 0 | tinyint NULL |
| `upd_time` | ⚠ changed | bigint NOT NULL | bigint NULL |

#### ⚠ Changed: `t_geo_administrative_region`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `level` | ⚠ changed | varchar(64) NULL DEFAULT  | varchar(64) NULL |

#### ⚠ Changed: `t_geo_latest_error_location`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `station` | ⚠ changed | varchar(2048) NULL | varchar(512) NULL |

#### ⚠ Changed: `t_geo_latest_location`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_UPD_TIME` | ✖ only in check |  |  (upd_time) [BTREE] |
| `i_sn_operator_id` | ✖ only in check |  |  (sn, operator_id) [BTREE] |

#### ⚠ Changed: `t_group_app`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `force_update` | ⚠ changed | tinyint NULL | tinyint NULL |
| `upd_time` | ⚠ changed | bigint NULL | bigint NULL |

#### ✖ Only in check: `t_map_use_record1`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record10`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record11`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record12`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record2`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record3`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record4`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record5`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record6`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record7`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record8`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ✖ Only in check: `t_map_use_record9`

| Column | Type |
|--------|------|
| `record_uuid` | varchar(36) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `router_addr` | varchar(50) NULL |
| `map_type` | varchar(20) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |

#### ⚠ Changed: `t_message_record`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `company` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `content` | ⚠ changed | varchar(2048) NOT NULL | varchar(2048) NOT NULL |
| `cre_time` | ⚠ changed | bigint NOT NULL | bigint NOT NULL |
| `remark` | ⚠ changed | varchar(1024) NULL | varchar(1024) NULL |
| `role` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `status` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |
| `type` | ⚠ changed | tinyint(1) NOT NULL | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_operator_change`

| Column | Type |
|--------|------|
| `operator_change_id` | bigint NOT NULL auto_increment |
| `oid_metms` | varchar(64) NULL |
| `oid_mtms` | varchar(64) NULL |
| `oid_tms` | varchar(64) NULL |
| `oid` | varchar(128) NOT NULL |
| `branch_id` | varchar(255) NOT NULL |
| `operator_id` | bigint NOT NULL |

#### ⚠ Changed: `t_operator_manufacturer`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `manufacturer_name` | ⚠ changed | varchar(128) NULL | varchar(256) NULL |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_CHARACTER_CODE_KEY_ID_MANUFACRURER_CODE` | ✖ only in check |  | UNIQUE (character_code, key_id, manufacturer_code) [BTREE] |

#### ⚠ Changed: `t_ota_release`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `version_name` | ⚠ changed | varchar(64) NOT NULL | varchar(128) NOT NULL |

#### ✚ Only in base: `t_param_mapping2`

| Column | Type |
|--------|------|
| `param_mapping_id` | bigint NOT NULL auto_increment |
| `oid` | varchar(128) NOT NULL |
| `operator_id` | bigint NOT NULL |
| `pkg_name` | varchar(128) NOT NULL |
| `dev_sn` | varchar(128) NOT NULL |
| `md5` | varchar(128) NOT NULL |
| `sha1` | varchar(128) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NOT NULL |
| `status` | tinyint NULL |

#### ✚ Only in base: `t_pub_xtms_branch_param_set`

| Column | Type |
|--------|------|
| `xtms_branch_param_set_id` | varchar(32) NOT NULL |
| `old_operator_id` | varchar(32) NOT NULL |
| `operator_id` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `old_group_id` | varchar(32) NULL |
| `group_id` | bigint NULL |
| `is_default` | varchar(4) NULL |
| `param_def_val` | varchar(255) NULL |
| `param_desc` | varchar(255) NULL |
| `param_desc_braz` | varchar(255) NULL |
| `param_desc_en` | varchar(255) NULL |
| `param_name` | varchar(255) NOT NULL |
| `param_option_type` | varchar(4) NOT NULL |
| `type` | varchar(4) NOT NULL |
| `upd_time` | datetime NULL |
| `xtms_param_option_value_id` | varchar(32) NULL |

#### ✚ Only in base: `t_receipt_app`

| Column | Type |
|--------|------|
| `receipt_app_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `type` | varchar(2) NOT NULL |
| `app_code` | varchar(50) NULL |
| `app_name` | varchar(100) NULL |

#### ⚠ Changed: `t_statistic_operator`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `statistic_operator_id` | ⚠ changed | int NOT NULL auto_increment | bigint NOT NULL auto_increment |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_STATISTIC_TYPE` | ✚ only in base |  (statistic_type) [BTREE] |  |

#### ⚠ Changed: `t_statistic_operator_dimension`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_STATISTIC_TYPE` | ✚ only in base |  (statistic_type) [BTREE] |  |

#### ⚠ Changed: `t_strategy_target_group`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_STRAGE_ID` | ✚ only in base |  (strategy_id) [BTREE] |  |

#### ⚠ Changed: `t_strategy_warning`

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_STATUS` | ✚ only in base |  (status) [BTREE] |  |

#### ⚠ Changed: `t_strategy_warning_details`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `new_value` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `old_value` | ⚠ changed | varchar(128) NULL | varchar(128) NULL |
| `report_info` | ⚠ changed | varchar(1024) NULL | varchar(1024) NULL |
| `status` | ⚠ changed | tinyint(1) NULL DEFAULT 1 | tinyint(1) NULL DEFAULT 1 |

##### Indexes

| Index | Status | Base | Check |
|-------|--------|------|-------|
| `I_WARNING_ID_STATUS` | ✚ only in base |  (strategy_warning_id, status) [BTREE] |  |

#### ⚠ Changed: `t_terminal_payment_parameter`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `mid` | ⚠ changed | varchar(64) NULL | varchar(32) NULL |

#### ⚠ Changed: `t_terminal_payment_parameter_error`

##### Columns

| Column | Status | Base | Check |
|--------|--------|------|-------|
| `acquirer_code` | ⚠ changed | varchar(255) NOT NULL DEFAULT  | varchar(255) NULL |
| `error_message` | ⚠ changed | varchar(64) NULL | varchar(64) NULL |
| `mid` | ⚠ changed | varchar(64) NULL | varchar(32) NULL |
| `type` | ⚠ changed | tinyint NULL | tinyint NULL |

### Views

#### ⚠ Changed: `view_strategy`

**Base SQL Definition:**

```sql
select `ts`.`strategy_id` AS `strategy_id`,`ts`.`strategy_name` AS `strategy_name`,`ts`.`plat_code` AS `plat_code`,`ts`.`character_code` AS `character_code`,`ts`.`key_id` AS `key_id`,`ts`.`org_id` AS `org_id`,`ts`.`notify_temp_id` AS `notify_temp_id`,`ts`.`warning_model_id` AS `warning_model_id`,`ts`.`strategy_threshold` AS `strategy_threshold`,`ts`.`email_notify_flag` AS `email_notify_flag`,`ts`.`inner_notify_flag` AS `inner_notify_flag`,`ts`.`timezone` AS `timezone`,`ts`.`target_bind_method` AS `target_bind_method`,date_format(from_unixtime(`ts`.`cre_time`),'%Y%m%d') AS `statistic_day`,date_format(from_unixtime(`ts`.`cre_time`),'%Y%m') AS `statistic_month`,date_format(from_unixtime(`ts`.`cre_time`),'%Y') AS `statistic_year`,`ts`.`cre_time` AS `cre_time`,`ts`.`upd_time` AS `upd_time` from `toms-mars`.`t_strategy` `ts`
```

**Check SQL Definition:**

```sql
select `ts`.`strategy_id` AS `strategy_id`,`ts`.`strategy_name` AS `strategy_name`,`ts`.`plat_code` AS `plat_code`,`ts`.`character_code` AS `character_code`,`ts`.`key_id` AS `key_id`,`ts`.`org_id` AS `org_id`,`ts`.`notify_temp_id` AS `notify_temp_id`,`ts`.`warning_model_id` AS `warning_model_id`,`ts`.`ignore_warning_keys` AS `ignore_warning_keys`,`ts`.`strategy_threshold` AS `strategy_threshold`,`ts`.`email_notify_flag` AS `email_notify_flag`,`ts`.`inner_notify_flag` AS `inner_notify_flag`,`ts`.`timezone` AS `timezone`,`ts`.`target_bind_method` AS `target_bind_method`,date_format(from_unixtime(`ts`.`cre_time`),'%Y%m%d') AS `statistic_day`,date_format(from_unixtime(`ts`.`cre_time`),'%Y%m') AS `statistic_month`,date_format(from_unixtime(`ts`.`cre_time`),'%Y') AS `statistic_year`,`ts`.`cre_time` AS `cre_time`,`ts`.`upd_time` AS `upd_time` from `toms-mars`.`t_strategy` `ts`
```

### Procedures

#### ✖ Only in check: `gendevice`

```sql
begin declare preSn varchar(12); declare counter int default 0; declare sn varchar(32); declare snPubkey varchar(2048); declare modelCode varchar(16); declare nowUTC int; declare deviceFlag varchar(16); set preSn='PTEST0'; set modelCode='N910'; set deviceFlag='A7'; set snPubkey='key'; select @nowUTC:=unix_timestamp(); while counter < 50000 do set sn = concat(preSn,lpad(started+counter,6,'0')); insert into t_device(dev_sn, manufacturer_code, model_code, model_name, bind_operator_id, bind_oid, bind_time, active_time, device_flag, dev_status, cre_time) values (sn, 'C1C3', modelCode, modelCode, operatorId, oid, @nowUTC, @nowUTC, deviceFlag, 6, @nowUTC); insert into t_device_bind(operator_id, operator_bind_time, org_id, dev_sn,model_code,model_name,device_flag,active_time, cre_type,cre_time,status) values (operatorId, @nowUTC, orgId, sn, modelCode, modelCode,deviceFlag, @nowUTC, 1, @nowUTC, 1); insert into t_device_security(dev_sn, pub_key,cre_type,key_change_times,cre_time,status) values (sn, snPubkey, 1, 0, @nowUTC, 1); IF mod(counter+1, 1000) = 0 THEN commit; END IF; set counter=counter+1; end while; commit; END
```

