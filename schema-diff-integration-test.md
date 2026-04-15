# Schema Diff Report

**Generated:** 2026-04-15 17:49:58

**Base:**  `192.168.35.15:48007`

**Check:** `192.168.35.140:65035`

---

## Summary

- **Databases compared:** 38
- **Databases only in base:** 21, only in check: 1, changed: 12

## ✚ Database only in base: `app-scan-mars`

### Tables

#### ✚ Only in base: `t_scan_file`

| Column | Type |
|--------|------|
| `scan_file_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `key_name` | varchar(128) NOT NULL |
| `mobsf_server` | varchar(128) NULL |
| `storage_scheme` | varchar(16) NOT NULL |
| `bucket_id` | varchar(32) NOT NULL |
| `file_path` | varchar(512) NOT NULL |
| `file_name` | varchar(512) NOT NULL |
| `file_md5` | varchar(32) NOT NULL |
| `file_length` | bigint NOT NULL |
| `file_report_path` | varchar(512) NULL |
| `scan_start_time` | bigint NULL |
| `warning_count` | tinyint NULL |
| `high_risk_count` | tinyint NULL |
| `security_score` | tinyint NULL |
| `scan_complete_time` | bigint NULL |
| `status` | tinyint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `upd_user_id` | bigint NULL |
| `remark` | varchar(256) NULL |

#### ✚ Only in base: `t_mobsf_instance`

| Column | Type |
|--------|------|
| `mobsf_instance_id` | bigint NOT NULL auto_increment |
| `instance_name` | varchar(64) NOT NULL |
| `base_url` | varchar(256) NOT NULL |
| `api_key` | varchar(128) NOT NULL |
| `max_file_size` | bigint NOT NULL |
| `max_concurrent` | tinyint NULL DEFAULT 1 |
| `status` | tinyint NULL DEFAULT 1 |

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

## ✚ Database only in base: `currency-converter`

### Tables

#### ✚ Only in base: `t_currencies`

| Column | Type |
|--------|------|
| `currency_id` | bigint NOT NULL auto_increment |
| `currency_code` | varchar(3) NOT NULL |
| `currency_name` | varchar(50) NOT NULL |
| `currency_numeric_code` | int NOT NULL |

#### ✚ Only in base: `t_currency_exchange_rate`

| Column | Type |
|--------|------|
| `exchange_rate_uuid` | char(32) NOT NULL |
| `from_currency` | varchar(3) NOT NULL |
| `to_currency` | varchar(3) NOT NULL |
| `rate` | decimal(10,4) NOT NULL |
| `operation_time` | bigint NOT NULL |
| `rate_date` | date NULL |
| `provider` | varchar(16) NULL |

#### ✚ Only in base: `t_history_exchange_rate`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `from_currency` | varchar(3) NOT NULL |
| `to_currency` | varchar(3) NOT NULL |
| `rate` | decimal(10,4) NOT NULL |
| `rate_date` | date NOT NULL |
| `exchange_rate_uuid` | char(32) NOT NULL |
| `operation_time` | bigint NOT NULL |
| `provider` | varchar(16) NULL |

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

## ✚ Database only in base: `document-mars`

### Tables

#### ✚ Only in base: `t_space_document_language_resource`

| Column | Type |
|--------|------|
| `space_document_language_resource_id` | bigint NOT NULL auto_increment |
| `space_document_id` | bigint NULL |
| `space_document_language_id` | bigint NOT NULL |
| `file_path` | varchar(512) NOT NULL |
| `web_url` | varchar(512) NOT NULL |
| `file_name` | varchar(256) NOT NULL |
| `file_size` | bigint NOT NULL |
| `file_type` | tinyint(1) NULL |

#### ✚ Only in base: `t_group`

| Column | Type |
|--------|------|
| `group_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NOT NULL |
| `name` | varchar(128) NOT NULL |
| `code` | varchar(16) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_platform`

| Column | Type |
|--------|------|
| `plat_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `plat_name` | varchar(128) NOT NULL |
| `plat_code` | varchar(16) NOT NULL |
| `cre_user_id` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_space`

| Column | Type |
|--------|------|
| `space_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `space_name` | varchar(128) NOT NULL |
| `space_code` | varchar(16) NOT NULL |
| `permission` | varchar(16) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_platform_directory_space_document`

| Column | Type |
|--------|------|
| `plat_directory_space_document_id` | bigint NOT NULL auto_increment |
| `plat_directory_id` | bigint NOT NULL |
| `document_id` | bigint NOT NULL |
| `sort` | int NOT NULL |

#### ✚ Only in base: `t_space_document`

| Column | Type |
|--------|------|
| `space_document_id` | bigint NOT NULL auto_increment |
| `space_id` | bigint NOT NULL |
| `directory_id` | bigint NOT NULL |
| `resource_version_name` | varchar(16) NOT NULL |
| `document_title` | varchar(256) NOT NULL |
| `permission` | varchar(16) NOT NULL |
| `sort` | int NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_group_document`

| Column | Type |
|--------|------|
| `group_document_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NOT NULL |
| `group_code` | varchar(16) NOT NULL |
| `document_id` | bigint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `directory_id` | bigint NULL |
| `permission` | varchar(100) NULL |

#### ✚ Only in base: `t_space_document_language`

| Column | Type |
|--------|------|
| `space_document_language_id` | bigint NOT NULL auto_increment |
| `space_document_id` | bigint NOT NULL |
| `document_title` | varchar(128) NOT NULL |
| `file_path` | varchar(512) NOT NULL |
| `web_url` | varchar(512) NOT NULL |
| `file_name` | varchar(256) NOT NULL |
| `file_size` | bigint NOT NULL |
| `language` | varchar(16) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `document_language_uuid` | varchar(32) NULL |
| `document_path` | varchar(512) NULL |

#### ✚ Only in base: `t_space_platform_whitelist`

| Column | Type |
|--------|------|
| `space_platform_whitelist_id` | bigint NOT NULL auto_increment |
| `space_code` | varchar(16) NOT NULL |
| `plat_code` | varchar(16) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_resource_version_remark`

| Column | Type |
|--------|------|
| `resource_version_remark_id` | bigint NOT NULL auto_increment |
| `resource_version_id` | bigint NOT NULL |
| `update_remark` | varchar(1024) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_directory`

| Column | Type |
|--------|------|
| `directory_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `relationship_id` | bigint NOT NULL |
| `resource_version_name` | varchar(16) NOT NULL |
| `title_zh` | varchar(128) NOT NULL |
| `title_en` | varchar(128) NULL |
| `title_pt` | varchar(128) NULL |
| `title_jp` | varchar(128) NULL |
| `parent_directory_id` | bigint NOT NULL |
| `sort` | int NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `file_directory_name` | varchar(128) NULL |

#### ✚ Only in base: `t_resource_version`

| Column | Type |
|--------|------|
| `resource_version_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `relationship_id` | bigint NOT NULL |
| `resource_version_name` | varchar(16) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | int NULL |

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

## ✚ Database only in base: `flycare`

### Tables

#### ✚ Only in base: `t_report_platform`

| Column | Type |
|--------|------|
| `report_platform_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(32) NOT NULL |
| `statistic_year` | varchar(4) NOT NULL |
| `statistic_month` | varchar(6) NOT NULL |
| `statistic_day` | varchar(8) NULL DEFAULT -- |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_chart_user`

| Column | Type |
|--------|------|
| `chart_user_id` | bigint NOT NULL auto_increment |
| `user_id` | bigint NOT NULL |
| `operator_id` | bigint NOT NULL |
| `chart_mode_id` | bigint NOT NULL |
| `chart_order` | int NULL |

#### ✚ Only in base: `t_report_event`

| Column | Type |
|--------|------|
| `report_event_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `month` | varchar(6) NOT NULL |
| `event_type` | varchar(32) NOT NULL |
| `customer_name` | varchar(256) NULL |
| `email` | varchar(1024) NULL |
| `exception_desc` | text NULL |
| `deal_remark` | varchar(256) NULL |
| `upd_user_id` | bigint NULL |
| `upd_username` | varchar(128) NULL |
| `upd_time` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_datasource_mode`

| Column | Type |
|--------|------|
| `datasource_mode_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `datasource_code` | varchar(64) NULL |
| `datasource_name` | varchar(64) NULL |
| `name` | varchar(64) NULL |
| `code` | varchar(64) NULL |
| `data_type` | tinyint NULL |
| `table_name` | varchar(64) NULL |
| `y_axis` | varchar(64) NULL |
| `column_value` | varchar(64) NULL |

#### ✚ Only in base: `t_chart_mode_condition`

| Column | Type |
|--------|------|
| `chart_mode_id` | bigint NOT NULL |
| `character_code` | varchar(100) NULL |
| `code` | varchar(64) NULL |
| `condition_type` | int NULL |
| `condition_value` | int NULL |
| `chart_mode_condition` | bigint NOT NULL auto_increment |

#### ✚ Only in base: `t_module`

| Column | Type |
|--------|------|
| `module_code` | varchar(32) NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_name` | varchar(45) NOT NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_contract_module`

| Column | Type |
|--------|------|
| `contract_module_id` | bigint NOT NULL auto_increment |
| `contract_uuid` | bigint NOT NULL |
| `sign_entity_id` | varchar(32) NULL |
| `billing_service_code` | varchar(64) NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_code` | varchar(32) NULL |
| `statistic_method_code` | varchar(32) NULL |
| `service_start_time` | bigint NULL |
| `service_end_time` | bigint NULL |
| `price` | decimal(10,4) NULL |
| `tier_price` | varchar(256) NULL |
| `currency` | varchar(16) NOT NULL |
| `free_num` | decimal(10,2) NOT NULL |
| `used_free_num` | decimal(10,2) NULL |
| `free_expire_time` | bigint NULL |
| `service_quantity` | decimal(10,2) NULL |
| `service_amount` | decimal(10,2) NULL |
| `write_off_quantity` | decimal(10,2) NULL DEFAULT 0.00 |
| `write_off_amount` | decimal(10,2) NULL DEFAULT 0.00 |
| `auto_renewal` | tinyint(1) NULL |
| `status` | tinyint(1) NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_user_id` | bigint NULL |
| `invalid_time` | bigint NULL |
| `invalid_user_id` | bigint NULL |
| `remark` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_report_month_detail`

| Column | Type |
|--------|------|
| `report_month_detail_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(32) NOT NULL |
| `key_id` | bigint NOT NULL |
| `report_id` | bigint NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_code` | varchar(32) NOT NULL |
| `org_id` | bigint NULL |
| `statistic_month` | varchar(6) NOT NULL |
| `statistic_count` | decimal(10,2) NOT NULL |
| `expire_date` | bigint NULL |
| `balance` | decimal(10,2) NULL |
| `free_count` | decimal(10,2) NOT NULL |
| `statistic_method_code` | varchar(32) NOT NULL |
| `statement_detail_id` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `detail_file_path` | varchar(128) NULL |

#### ✚ Only in base: `t_chart_mode_detail`

| Column | Type |
|--------|------|
| `chart_mode_detail_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `chart_mode_id` | bigint NOT NULL |
| `name` | varchar(64) NULL |
| `code` | varchar(64) NULL |
| `data_type` | int NULL |
| `y_axis` | varchar(64) NULL |
| `column_value` | varchar(64) NULL |

#### ✚ Only in base: `t_billing_service`

| Column | Type |
|--------|------|
| `billing_service_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NOT NULL |
| `billing_service_code` | varchar(64) NOT NULL |
| `billing_service_name` | varchar(64) NOT NULL |
| `statistic_method_code` | varchar(32) NULL |
| `payment_mode` | varchar(16) NULL |
| `billing_type` | varchar(16) NOT NULL |
| `service_amount_unit` | varchar(512) NOT NULL |
| `service_quantity_unit` | varchar(16) NULL |
| `allow_duplicate` | tinyint(1) NOT NULL |
| `allow_free` | tinyint(1) NOT NULL |
| `free_expire_flag` | tinyint(1) NULL |
| `start_time_require` | int NULL |
| `continuous_free` | int NULL |
| `write_off_type` | varchar(16) NOT NULL |
| `can_renewal` | tinyint(1) NULL |
| `renewal_month` | int NULL |
| `prewarning_flag` | tinyint(1) NOT NULL |
| `prewarning_num_threshold` | decimal(10,2) NULL |
| `prewarning_time_threshold` | bigint NULL |
| `expire_flag` | tinyint(1) NOT NULL |
| `billing_service_tag` | varchar(32) NOT NULL |
| `billing_service_desc` | varchar(255) NULL |
| `write_off_strategy` | int NULL |
| `billing_status` | tinyint(1) NOT NULL DEFAULT 1 |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_statistic_method`

| Column | Type |
|--------|------|
| `statistic_method_code` | varchar(32) NOT NULL |
| `statistic_method_name` | varchar(64) NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_code` | varchar(32) NOT NULL |
| `priority` | tinyint(1) NULL DEFAULT 10 |
| `remark` | varchar(512) NULL |
| `unit` | varchar(32) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `allow_free_num` | tinyint(1) NOT NULL DEFAULT 1 |
| `billing_cycle` | varchar(128) NOT NULL DEFAULT  |
| `bill_target` | varchar(16) NOT NULL DEFAULT MODULE |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_customer_platform`

| Column | Type |
|--------|------|
| `customer_platform_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `support_flag` | tinyint(1) NOT NULL |
| `expire_date` | bigint NULL |
| `month` | varchar(6) NULL |
| `status` | tinyint(1) NULL |

#### ✚ Only in base: `t_report_detail`

| Column | Type |
|--------|------|
| `report_detail_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(32) NOT NULL |
| `key_id` | bigint NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_code` | varchar(32) NOT NULL |
| `org_id` | bigint NULL |
| `model_code` | varchar(32) NULL |
| `statistic_method_code` | varchar(32) NOT NULL |
| `customer_cert_cn` | varchar(64) NULL |
| `statistic_year` | varchar(4) NOT NULL |
| `statistic_month` | varchar(6) NOT NULL |
| `statistic_day` | varchar(8) NULL DEFAULT -- |
| `statistic_count` | decimal(10,2) NOT NULL |
| `expire_date` | bigint NULL |
| `balance` | decimal(10,2) NULL |
| `free_count` | decimal(10,2) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `detail_file_path` | varchar(128) NULL |
| `condition_label_1` | varchar(64) NULL |
| `condition_label_2` | varchar(64) NULL |
| `remark` | varchar(512) NULL |

#### ✚ Only in base: `t_report_inform`

| Column | Type |
|--------|------|
| `report_inform_id` | int NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `email` | varchar(32) NOT NULL |
| `send_type` | tinyint NOT NULL |
| `inform_type` | tinyint NOT NULL |
| `user_id` | bigint NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_contract_default_email`

| Column | Type |
|--------|------|
| `email_id` | bigint NOT NULL auto_increment |
| `email_address` | varchar(256) NULL |
| `status` | tinyint NOT NULL DEFAULT 0 |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_report`

| Column | Type |
|--------|------|
| `report_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `month` | varchar(6) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `confirm_user_id` | bigint NULL |
| `confirm_username` | varchar(128) NULL |
| `confirm_state` | tinyint(1) NOT NULL |
| `confirm_time` | bigint NULL |
| `reject_reason` | varchar(512) NULL |
| `token` | varchar(64) NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_sign_entity`

| Column | Type |
|--------|------|
| `sign_entity_id` | varchar(32) NOT NULL DEFAULT  |
| `sign_entity_name` | varchar(64) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_chart_mode_order`

| Column | Type |
|--------|------|
| `chart_mode_order_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `chart_mode_id` | bigint NOT NULL |
| `name` | varchar(64) NULL |
| `code` | varchar(64) NULL |
| `order_type` | varchar(8) NULL |

#### ✚ Only in base: `t_statement`

| Column | Type |
|--------|------|
| `statement_id` | bigint NOT NULL auto_increment |
| `statement_month` | varchar(6) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `oa_customer_id` | varchar(45) NULL |
| `contract_uuid` | varchar(32) NOT NULL |
| `report_id` | bigint NULL |
| `sign_entity_id` | varchar(32) NOT NULL |
| `total_amount` | decimal(12,4) NULL DEFAULT 0.0000 |
| `bill_id` | bigint NULL |
| `notify_oa_flag` | tinyint(1) NULL |
| `notify_oa_time` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_bill`

| Column | Type |
|--------|------|
| `bill_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `oa_customer_id` | varchar(45) NOT NULL |
| `oa_bill_id` | varchar(32) NOT NULL |
| `oa_bill_num` | varchar(64) NULL |
| `bill_amount` | decimal(12,4) NOT NULL |
| `currency` | varchar(3) NOT NULL |
| `verify_person` | varchar(40) NOT NULL |
| `verify_time` | bigint NOT NULL |
| `payment_time` | bigint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `remark` | varchar(512) NULL |
| `status` | tinyint(1) NULL |

#### ✚ Only in base: `t_support_currency`

| Column | Type |
|--------|------|
| `currency_code` | varchar(16) NOT NULL DEFAULT  |
| `status` | tinyint NOT NULL DEFAULT 0 |
| `symbol` | varchar(4) NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_chart_mode`

| Column | Type |
|--------|------|
| `chart_mode_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `dashboard_show` | tinyint NULL |
| `mode_name` | varchar(64) NULL |
| `mode_code` | varchar(64) NULL |
| `menu_code` | varchar(64) NULL |
| `chart_order` | tinyint NULL |
| `datasource_code` | varchar(64) NULL |
| `table_name` | varchar(64) NULL |
| `chart_type` | varchar(32) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_contract`

| Column | Type |
|--------|------|
| `contract_uuid` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `contract_num` | varchar(128) NOT NULL |
| `effective_date` | varchar(8) NOT NULL |
| `expiring_date` | varchar(8) NULL |
| `email` | varchar(1024) NULL |
| `invalid_time` | bigint NULL |
| `invalid_user_id` | bigint NULL |
| `sign_entity_id` | varchar(32) NOT NULL |
| `upd_time` | bigint NULL |
| `upd_user_id` | bigint NULL |
| `cre_user_id` | bigint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `remark` | varchar(512) NULL |
| `status` | tinyint(1) NULL |

#### ✚ Only in base: `t_event`

| Column | Type |
|--------|------|
| `event_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `plat_code` | varchar(16) NULL |
| `org_id` | bigint NOT NULL |
| `user_id` | bigint NOT NULL |
| `event_code` | varchar(32) NOT NULL |
| `event_desc` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(1024) NULL |

#### ✚ Only in base: `t_statement_detail`

| Column | Type |
|--------|------|
| `statement_detail_id` | bigint NOT NULL auto_increment |
| `statement_id` | bigint NOT NULL DEFAULT 0 |
| `report_id` | bigint NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `month` | varchar(6) NOT NULL |
| `plat_code` | varchar(32) NOT NULL |
| `module_code` | varchar(32) NULL |
| `statistic_method_code` | varchar(32) NULL |
| `billing_service_code` | varchar(64) NULL |
| `price` | decimal(12,4) NULL |
| `actual_charge` | decimal(12,4) NULL |
| `module_count` | decimal(10,2) NULL |
| `module_free_count` | decimal(10,2) NOT NULL |
| `contract_free_count` | decimal(10,2) NOT NULL |
| `oa_customer_id` | varchar(45) NULL |
| `currency` | varchar(3) NOT NULL |
| `module_amount` | decimal(12,4) NOT NULL |
| `contract_uuid` | bigint NOT NULL |
| `contract_module_id` | bigint NULL |
| `status` | tinyint(1) NOT NULL |
| `bill_id` | bigint NULL |
| `notify_oa_time` | bigint NULL |
| `sign_entity_id` | varchar(32) NOT NULL |
| `notify_oa_flag` | tinyint(1) NOT NULL DEFAULT 0 |
| `settlement_remark` | varchar(256) NULL |
| `remark` | varchar(255) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `usd_exchange_amount` | decimal(12,4) NULL |
| `usd_exchange_rate` | float NULL |
| `usd_exchange_rate_id` | varchar(32) NULL |

### Views

#### ✚ Only in base: `view_report_month_detail`

```sql
select `a`.`report_month_detail_id` AS `report_month_detail_id`,`a`.`character_code` AS `character_code`,`a`.`key_id` AS `key_id`,`a`.`report_id` AS `report_id`,`a`.`plat_code` AS `plat_code`,`a`.`module_code` AS `module_code`,`a`.`org_id` AS `org_id`,`a`.`statistic_month` AS `statistic_month`,`a`.`statistic_count` AS `statistic_count`,`a`.`free_count` AS `free_count`,`a`.`statistic_method_code` AS `statistic_method_code`,`a`.`statement_detail_id` AS `statement_detail_id`,`a`.`cre_time` AS `cre_time`,`a`.`detail_file_path` AS `detail_file_path` from (`flycare`.`t_report_month_detail` `a` join `flycare`.`t_report` `b`) where ((`a`.`report_id` = `b`.`report_id`) and (`b`.`confirm_state` <> 4))
```

#### ✚ Only in base: `view_statement_detail`

```sql
select `a`.`month` AS `statistic_month`,substr(`a`.`month`,1,4) AS `statistic_year`,`a`.`character_code` AS `character_code`,`a`.`key_id` AS `key_id`,`a`.`plat_code` AS `plat_code`,`a`.`billing_service_code` AS `billing_service_code`,`a`.`module_amount` AS `module_amount`,`a`.`usd_exchange_amount` AS `usd_exchange_amount`,`a`.`bill_id` AS `bill_id`,`a`.`sign_entity_id` AS `sign_entity_id`,(period_diff(date_format(now(),'%Y%m'),`a`.`month`) - 1) AS `overdueMonth`,`a`.`contract_uuid` AS `contract_uuid`,`a`.`cre_time` AS `cre_time` from `flycare`.`t_statement_detail` `a`
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

## ✚ Database only in base: `flyiot-demo`

### Tables

#### ✚ Only in base: `t_callback_data`

| Column | Type |
|--------|------|
| `callback_data_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `cmd_code` | varchar(64) NULL |
| `message_id` | varchar(64) NULL |
| `headers` | text NULL |
| `content` | text NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_openapi`

| Column | Type |
|--------|------|
| `openapi_id` | bigint NOT NULL auto_increment |
| `title` | varchar(128) NOT NULL |
| `app_id` | varchar(64) NOT NULL |
| `app_key` | varchar(64) NOT NULL |
| `status` | tinyint(1) NULL |
| `cre_time` | bigint NULL |
| `remark` | varchar(128) NULL |

#### ✚ Only in base: `t_openapi_device`

| Column | Type |
|--------|------|
| `openapi_dev_id` | bigint NOT NULL auto_increment |
| `openapi_id` | bigint NOT NULL |
| `product_key` | varchar(32) NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

## ✚ Database only in base: `gcluster-inner`

### Tables

#### ✚ Only in base: `t_node_server`

| Column | Type |
|--------|------|
| `node_id` | varchar(32) NOT NULL |
| `node_name` | varchar(64) NULL |
| `node_outer_ip` | varchar(50) NULL |
| `node_inner_ip` | varchar(16) NULL |
| `ws_port` | int NULL |
| `node_port` | int NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `domain` | varchar(128) NULL |

#### ✚ Only in base: `t_operator_transition`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `node_id` | varchar(32) NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `terminal_host` | varchar(128) NOT NULL |
| `transition_ip` | varchar(128) NOT NULL |
| `transition_port` | int NOT NULL |
| `cre_time` | datetime NULL |

#### ✚ Only in base: `t_operator_domain`

| Column | Type |
|--------|------|
| `oid` | varchar(64) NOT NULL |
| `cre_time` | datetime NULL |

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

## ✚ Database only in base: `gcluster-out`

### Tables

#### ✚ Only in base: `t_node_server`

| Column | Type |
|--------|------|
| `node_id` | varchar(32) NOT NULL |
| `node_name` | varchar(64) NULL |
| `node_outer_ip` | varchar(50) NULL |
| `node_inner_ip` | varchar(16) NULL |
| `ws_port` | int NULL |
| `node_port` | int NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

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

## ✚ Database only in base: `message`

### Tables

#### ✚ Only in base: `t_verify_code`

| Column | Type |
|--------|------|
| `verify_code_uuid` | varchar(32) NOT NULL |
| `plat_code` | varchar(8) NULL |
| `key_id` | bigint NULL |
| `target_type` | int NULL |
| `target` | varchar(128) NULL |
| `verify_code` | varchar(16) NOT NULL |
| `cre_time` | bigint NULL |
| `verify_time` | bigint NULL |
| `verify_times` | int NULL |
| `status` | tinyint(1) NULL |

#### ✚ Only in base: `t_channel`

| Column | Type |
|--------|------|
| `chnl_id` | bigint NOT NULL auto_increment |
| `chnl_code` | varchar(32) NOT NULL |
| `chnl_title` | varchar(64) NULL |
| `sender` | varchar(32) NULL |
| `app_key` | varchar(64) NULL |
| `app_secret` | varchar(64) NULL |
| `request_url` | varchar(128) NULL |
| `sign_scheme` | varchar(32) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_message_detail`

| Column | Type |
|--------|------|
| `message_detail_uuid` | varchar(32) NOT NULL |
| `chnl_id` | bigint NOT NULL |
| `chnl_code` | varchar(32) NOT NULL |
| `request_url` | varchar(128) NULL |
| `template_id` | varchar(128) NOT NULL |
| `template_text` | varchar(128) NOT NULL |
| `message_type` | int NOT NULL |
| `signature` | varchar(64) NULL |
| `params` | varchar(256) NULL |
| `from_mobile` | varchar(32) NULL |
| `to_mobile` | varchar(32) NULL |
| `cre_time` | bigint NOT NULL |
| `send_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |
| `status_time` | datetime NULL |
| `status_message` | varchar(128) NULL |
| `call_back_url` | varchar(256) NULL |
| `call_back_text` | varchar(4000) NULL |

#### ✚ Only in base: `t_chnl_template`

| Column | Type |
|--------|------|
| `chnl_temp_id` | bigint NOT NULL auto_increment |
| `chnl_code` | varchar(32) NOT NULL |
| `chnl_id` | bigint NOT NULL |
| `template_id` | varchar(64) NOT NULL |
| `template_text` | varchar(1024) NOT NULL |
| `message_type` | tinyint(1) NULL |
| `signature` | varchar(64) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

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

## ✚ Database only in base: `mss`

### Tables

#### ✚ Only in base: `t_role_resource`

| Column | Type |
|--------|------|
| `role_resource_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `resource_id` | varchar(64) NOT NULL |
| `role_id` | varchar(32) NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_electronic_tag`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `is_default` | varchar(2) NOT NULL |
| `organ_id` | varchar(32) NOT NULL |
| `tag_code` | varchar(32) NOT NULL |
| `tag_name` | varchar(255) NULL |
| `tag_name_braz` | varchar(255) NULL |
| `tag_name_en` | varchar(255) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_email_push_detail`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `amount_bar_path` | varchar(255) NULL |
| `amount_line_path` | varchar(255) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `email` | varchar(32) NOT NULL |
| `organ_id` | varchar(32) NULL |
| `shop_id` | varchar(32) NULL |
| `status` | varchar(2) NOT NULL |
| `sum_date` | varchar(10) NULL |
| `sum_month` | varchar(10) NULL |
| `type` | varchar(2) NULL |
| `upd_time` | datetime NULL |
| `volume_bar_path` | varchar(255) NULL |
| `volume_line_path` | varchar(255) NULL |

#### ✚ Only in base: `t_elec_template_detail`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `font_size` | int NULL |
| `left_offset` | int NULL |
| `offset_x` | varchar(255) NULL |
| `offset_y` | varchar(255) NULL |
| `organ_id` | varchar(32) NOT NULL |
| `partial_picture` | longtext NULL |
| `print_position` | varchar(2) NULL |
| `row_width` | int NULL |
| `sort` | int NULL |
| `tag_code` | varchar(255) NULL |
| `template_id` | varchar(32) NOT NULL |
| `template_type` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_product`

| Column | Type |
|--------|------|
| `prod_code` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `firm_code` | varchar(32) NOT NULL |
| `img_path` | varchar(255) NULL |
| `mode_hand` | varchar(1) NULL |
| `mode_hd` | varchar(1) NULL |
| `prod_name` | varchar(60) NOT NULL |
| `prod_status` | varchar(2) NULL DEFAULT 1 |
| `remark` | longtext NULL |
| `remark_braz` | longtext NULL |
| `remark_en` | longtext NULL |
| `type` | varchar(4) NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_web_action`

| Column | Type |
|--------|------|
| `web_action_id` | varchar(32) NOT NULL |
| `action_desc` | varchar(20) NOT NULL |
| `action_path` | varchar(100) NOT NULL |
| `branch_id` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `organ_id` | varchar(32) NULL |
| `user_name` | varchar(100) NOT NULL |

#### ✚ Only in base: `t_device`

| Column | Type |
|--------|------|
| `dev_sn` | varchar(32) NOT NULL |
| `activation_status` | varchar(2) NULL DEFAULT 0 |
| `activation_time` | datetime NULL |
| `batch_no` | varchar(20) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `dev_check_value` | varchar(32) NULL |
| `dev_status` | varchar(2) NULL |
| `firm_code` | varchar(32) NOT NULL |
| `organ_id` | varchar(32) NULL |
| `prod_code` | varchar(32) NOT NULL |
| `shop_id` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_branch`

| Column | Type |
|--------|------|
| `branch_id` | varchar(32) NOT NULL |
| `address` | varchar(120) NULL |
| `auth_date` | varchar(255) NULL |
| `auth_sign` | varchar(32) NULL |
| `auth_terminal_amount` | int NULL |
| `branch_name` | varchar(120) NOT NULL |
| `branch_status` | varchar(2) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `currency` | varchar(255) NULL |
| `oid_metms` | varchar(32) NULL |
| `oid_mtms` | varchar(40) NULL |
| `oid_tms` | varchar(32) NULL |
| `remark` | varchar(210) NULL |
| `upd_time` | datetime NULL |
| `time_zone` | varchar(255) NULL |

#### ✚ Only in base: `t_shop_user`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `contacts` | varchar(64) NULL |
| `cre_time` | datetime NULL |
| `email` | varchar(64) NULL |
| `err_pwd_num` | int NULL DEFAULT 0 |
| `err_pwd_time` | varchar(255) NULL |
| `last_login_ip` | varchar(40) NULL |
| `last_login_time` | datetime NULL |
| `login_num` | int NULL DEFAULT 0 |
| `mobile` | varchar(32) NULL |
| `old_password_1` | varchar(255) NULL |
| `old_password_2` | varchar(255) NULL |
| `old_password_3` | varchar(255) NULL |
| `old_password_4` | varchar(255) NULL |
| `old_password_5` | varchar(255) NULL |
| `password` | varchar(255) NULL |
| `pic` | varchar(100) NULL |
| `role_id` | varchar(32) NULL |
| `set_pwd_date` | datetime NULL |
| `shop_id` | varchar(32) NOT NULL |
| `upd_time` | datetime NULL |
| `user_name` | varchar(64) NULL |
| `captcha` | varchar(255) NULL |
| `captcha_cre_time` | datetime NULL |
| `captcha_error_num` | int NULL |

#### ✚ Only in base: `t_sys_auth`

| Column | Type |
|--------|------|
| `sys_auth_id` | varchar(32) NOT NULL |
| `auth_code` | longtext NULL |
| `auth_pub_key` | longtext NOT NULL |
| `machine_list` | longtext NOT NULL |

#### ✚ Only in base: `t_sn_app_key`

| Column | Type |
|--------|------|
| `key_id` | varchar(32) NOT NULL |
| `sn` | varchar(64) NOT NULL |
| `app_code` | varchar(256) NOT NULL |
| `cert_post` | varchar(2048) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `cert_pos` | varchar(2048) NULL |

#### ✚ Only in base: `t_organ`

| Column | Type |
|--------|------|
| `organ_id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NOT NULL |
| `captcha` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `email` | varchar(32) NULL |
| `email_code` | varchar(32) NULL |
| `email_host` | varchar(32) NULL |
| `email_status` | varchar(2) NULL DEFAULT 0 |
| `email_type` | varchar(32) NULL |
| `invite_code` | varchar(32) NOT NULL |
| `organ_code` | varchar(45) NOT NULL |
| `organ_name` | varchar(45) NOT NULL |
| `organ_remark` | longtext NULL |
| `parent_code` | varchar(45) NOT NULL |
| `parent_id` | varchar(32) NOT NULL |
| `upd_time` | datetime NULL |
| `toms_organ_id` | varchar(32) NULL |

#### ✚ Only in base: `t_detail_his`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `acquirer` | varchar(32) NULL |
| `amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `app_code` | varchar(128) NULL |
| `app_name` | varchar(128) NULL |
| `auth_code` | varchar(32) NULL |
| `batch_no` | varchar(32) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `car_holder_sign_img` | blob NULL |
| `card_number` | varchar(32) NULL |
| `card_prt_format` | varchar(2) NULL |
| `certificate_a` | mediumblob NULL |
| `certificate_c` | mediumblob NULL |
| `certificate_m` | mediumblob NULL |
| `certificate_number` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `currency` | varchar(32) NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `email` | varchar(32) NULL |
| `email_send_res` | varchar(32) NULL |
| `issuer` | varchar(32) NULL |
| `merchant_id` | varchar(32) NULL |
| `merchant_name` | varchar(255) NULL |
| `merchant_no` | varchar(32) NULL |
| `message_body` | varchar(4096) NULL |
| `operator_no` | varchar(32) NULL |
| `org_auth_no` | varchar(32) NULL |
| `organ_id` | varchar(32) NOT NULL |
| `reference_number` | varchar(32) NULL |
| `remark` | varchar(255) NULL |
| `shop_id` | varchar(32) NULL |
| `status` | varchar(10) NULL |
| `status_msg` | varchar(255) NULL |
| `terminal_no` | varchar(32) NULL |
| `trans_code` | varchar(32) NULL |
| `trans_name` | varchar(32) NULL |
| `uniq_identifier` | varchar(32) NULL |
| `upd_time` | datetime NULL |
| `validity_period` | varchar(10) NULL |
| `voucher_type` | varchar(2) NULL |
| `car_holder_sign_data` | varchar(2048) NULL |

#### ✚ Only in base: `t_sys_rsa_key_group`

| Column | Type |
|--------|------|
| `group_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `index_key` | varchar(2) NOT NULL |
| `pri_key` | longtext NOT NULL |
| `pub_key` | longtext NOT NULL |
| `upd_time` | datetime NOT NULL |

#### ✚ Only in base: `t_shop`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NOT NULL |
| `captcha` | varchar(32) NULL |
| `cre_time` | datetime NOT NULL |
| `email` | varchar(32) NULL |
| `email_status` | varchar(2) NULL DEFAULT 0 |
| `emailType` | varchar(32) NULL |
| `isSubscribe` | varchar(2) NULL DEFAULT 0 |
| `lang` | varchar(32) NULL DEFAULT en_US |
| `organ_id` | varchar(32) NOT NULL |
| `parent_code` | varchar(45) NULL |
| `parent_id` | varchar(32) NULL |
| `remark` | varchar(255) NULL |
| `shop_code` | varchar(45) NOT NULL |
| `shop_name` | varchar(256) NOT NULL |
| `shop_status` | varchar(2) NULL |
| `subscribe_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `open_flag` | varchar(2) NULL |

#### ✚ Only in base: `t_email_type`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `host` | varchar(32) NOT NULL |
| `name` | varchar(32) NOT NULL |
| `suffix` | varchar(32) NOT NULL |
| `upd_time` | datetime NOT NULL |

#### ✚ Only in base: `t_user`

| Column | Type |
|--------|------|
| `user_name` | varchar(40) NOT NULL |
| `branch_id` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `email` | varchar(32) NULL |
| `err_pwd_num` | int NULL DEFAULT 0 |
| `err_pwd_time` | varchar(8) NULL |
| `last_login_ip` | varchar(40) NULL |
| `last_login_time` | datetime NULL |
| `login_num` | int NULL DEFAULT 0 |
| `mobile` | varchar(32) NULL |
| `old_password_1` | varchar(255) NULL |
| `old_password_2` | varchar(255) NULL |
| `old_password_3` | varchar(255) NULL |
| `old_password_4` | varchar(255) NULL |
| `old_password_5` | varchar(255) NULL |
| `organ_id` | varchar(32) NULL |
| `password` | varchar(255) NOT NULL |
| `pic` | varchar(100) NULL |
| `real_name` | varchar(64) NULL |
| `remark` | varchar(210) NULL |
| `role_id` | varchar(32) NULL |
| `role_name` | varchar(45) NULL |
| `set_pwd_date` | datetime NULL |
| `theme` | varchar(1) NULL DEFAULT 1 |
| `upd_time` | datetime NULL |
| `user_status` | varchar(2) NOT NULL |
| `user_type` | varchar(2) NOT NULL |

#### ✚ Only in base: `t_device_app`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NULL |
| `app_ver` | varchar(128) NULL |
| `cre_time` | datetime NULL |
| `dev_sn` | varchar(32) NULL |
| `merchant_id` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_question`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `author` | varchar(100) NOT NULL |
| `content_ch` | varchar(255) NOT NULL |
| `content_en` | varchar(255) NOT NULL |
| `cre_time` | datetime NULL |
| `title_ch` | varchar(100) NOT NULL |
| `title_en` | varchar(100) NOT NULL |
| `upd_time` | datetime NULL |
| `view_count` | int NOT NULL |

#### ✚ Only in base: `t_merchant`

| Column | Type |
|--------|------|
| `id` | varchar(255) NOT NULL |
| `app_id` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `merchant_code` | varchar(32) NOT NULL |
| `merchant_name` | varchar(255) NOT NULL |
| `merchant_status` | varchar(255) NOT NULL |
| `remark` | varchar(210) NULL |
| `shop_id` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_resource`

| Column | Type |
|--------|------|
| `resource_id` | varchar(64) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `parent_id` | varchar(64) NULL |
| `permission_code` | varchar(128) NULL |
| `priority` | varchar(64) NULL |
| `resource_icon` | varchar(64) NULL |
| `resource_name` | varchar(50) NOT NULL |
| `resource_status` | varchar(2) NOT NULL |
| `resource_tag` | varchar(64) NOT NULL |
| `resource_type` | varchar(4) NOT NULL |
| `resource_url` | varchar(255) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_electronic_basic_unit`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NOT NULL |
| `organ_id` | varchar(32) NOT NULL |
| `unit_type` | varchar(16) NOT NULL |
| `slogan_name` | varchar(64) NULL |
| `img_path` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `updTime` | datetime NULL |

#### ✚ Only in base: `t_electronic_template`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `organ_id` | varchar(32) NOT NULL |
| `stub_type` | varchar(32) NULL |
| `template_height` | varchar(5) NULL |
| `trans_code` | varchar(32) NULL |
| `upd_time` | datetime NULL |
| `voucher_type` | varchar(2) NULL |

#### ✚ Only in base: `t_settlement`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NULL |
| `app_name` | varchar(128) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `certificate_a` | mediumblob NULL |
| `certificate_c` | mediumblob NULL |
| `certificate_m` | mediumblob NULL |
| `cre_time` | datetime NULL |
| `credit_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `credit_total` | varchar(10) NULL DEFAULT 0 |
| `currency` | varchar(32) NULL |
| `debit_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `debit_total` | varchar(10) NULL DEFAULT 0 |
| `dev_sn` | varchar(32) NOT NULL |
| `email` | varchar(32) NULL |
| `email_send_res` | varchar(32) NULL |
| `merchant_id` | varchar(32) NULL |
| `merchant_name` | varchar(255) NULL |
| `merchant_no` | varchar(32) NULL |
| `message_body` | varchar(4096) NULL |
| `operator_no` | varchar(32) NULL |
| `organ_id` | varchar(32) NOT NULL |
| `other_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `other_total` | varchar(10) NULL DEFAULT 0 |
| `remark` | varchar(255) NULL |
| `shop_id` | varchar(32) NULL |
| `terminal_no` | varchar(32) NULL |
| `trans_code` | varchar(32) NULL |
| `trans_name` | varchar(32) NULL |
| `uniq_identifier` | varchar(32) NULL |
| `voucher_type` | varchar(2) NULL |
| `email_send_status` | varchar(2) NULL DEFAULT 0 |

#### ✚ Only in base: `t_product_sum`

| Column | Type |
|--------|------|
| `organ_code` | varchar(45) NOT NULL |
| `prod_code` | varchar(32) NOT NULL |
| `sum_day` | varchar(8) NOT NULL |
| `type_code` | varchar(32) NOT NULL |
| `active_sum` | bigint NULL |
| `active_today` | bigint NULL |
| `branch_id` | varchar(32) NOT NULL |
| `online_sum` | bigint NULL |
| `upd_time` | datetime NULL |
| `wait_active_sum` | bigint NULL |
| `wait_active_today` | bigint NULL |

#### ✚ Only in base: `t_channel`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NULL |
| `channel_name` | varchar(255) NULL |
| `channel_name_braz` | varchar(255) NULL |
| `channel_name_en` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_detail`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `acquirer` | varchar(32) NULL |
| `amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `app_code` | varchar(128) NULL |
| `app_name` | varchar(128) NULL |
| `auth_code` | varchar(32) NULL |
| `batch_no` | varchar(32) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `car_holder_sign_img` | blob NULL |
| `card_number` | varchar(32) NULL |
| `card_prt_format` | varchar(2) NULL |
| `certificate_a` | mediumblob NULL |
| `certificate_c` | mediumblob NULL |
| `certificate_m` | mediumblob NULL |
| `certificate_number` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `currency` | varchar(32) NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `email` | varchar(32) NULL |
| `email_send_res` | varchar(32) NULL |
| `issuer` | varchar(32) NULL |
| `merchant_id` | varchar(32) NULL |
| `merchant_name` | varchar(255) NULL |
| `merchant_no` | varchar(32) NULL |
| `message_body` | varchar(4096) NULL |
| `operator_no` | varchar(32) NULL |
| `org_auth_no` | varchar(32) NULL |
| `organ_id` | varchar(32) NOT NULL |
| `reference_number` | varchar(32) NULL |
| `remark` | varchar(255) NULL |
| `shop_id` | varchar(32) NULL |
| `status` | varchar(10) NULL |
| `status_msg` | varchar(255) NULL |
| `terminal_no` | varchar(32) NULL |
| `trans_code` | varchar(32) NULL |
| `trans_name` | varchar(32) NULL |
| `uniq_identifier` | varchar(32) NULL |
| `upd_time` | datetime NULL |
| `validity_period` | varchar(10) NULL |
| `voucher_type` | varchar(2) NULL |
| `email_send_status` | varchar(2) NULL DEFAULT 0 |
| `car_holder_sign_data` | varchar(2048) NULL |

#### ✚ Only in base: `t_role`

| Column | Type |
|--------|------|
| `role_id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NULL |
| `cre_time` | datetime NOT NULL |
| `organ_id` | varchar(255) NULL |
| `role_name` | varchar(45) NOT NULL |
| `role_remark` | longtext NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_channel_tran`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `channel_id` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `http_method` | varchar(5) NULL |
| `http_url` | varchar(80) NULL |
| `tcp_ip` | varchar(32) NULL |
| `tcp_port` | varchar(5) NULL |
| `tcp_tpdu` | varchar(10) NULL |
| `trans_code` | varchar(32) NULL |
| `trans_name` | varchar(255) NULL |
| `trans_name_braz` | varchar(255) NULL |
| `trans_name_en` | varchar(255) NULL |
| `trans_protocol` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_firm`

| Column | Type |
|--------|------|
| `firm_code` | varchar(32) NOT NULL |
| `address` | varchar(120) NULL |
| `cre_time` | datetime NULL |
| `firm_name` | varchar(120) NOT NULL |
| `firm_status` | varchar(2) NULL DEFAULT 1 |
| `img_path` | varchar(255) NULL |
| `linkman` | varchar(60) NULL |
| `mobile` | varchar(15) NULL |
| `remark` | varchar(210) NULL |
| `shortName` | varchar(32) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_login_info`

| Column | Type |
|--------|------|
| `login_info_id` | varchar(32) NOT NULL |
| `branch_id` | varchar(32) NULL |
| `cre_time` | datetime NOT NULL |
| `login_addr` | varchar(255) NULL |
| `login_ip` | varchar(100) NULL |
| `organ_id` | varchar(32) NULL |
| `user_name` | varchar(100) NOT NULL |

#### ✚ Only in base: `t_trans_type`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(100) NULL |
| `branch_id` | varchar(32) NOT NULL |
| `channel_tran_id` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `credit_debit` | varchar(2) NOT NULL |
| `trans_code` | varchar(32) NOT NULL |
| `trans_name` | varchar(255) NULL |
| `trans_name_braz` | varchar(255) NULL |
| `trans_name_en` | varchar(255) NULL |
| `trans_way` | varchar(2) NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_app`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `app_code` | varchar(128) NOT NULL |
| `app_name` | varchar(128) NULL |
| `branch_id` | varchar(32) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NOT NULL |

#### ✚ Only in base: `t_trans_sum`

| Column | Type |
|--------|------|
| `id` | varchar(32) NOT NULL |
| `cre_time` | datetime NULL |
| `credit_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `credit_total` | varchar(10) NULL DEFAULT 0 |
| `debit_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `debit_total` | varchar(10) NULL DEFAULT 0 |
| `other_amount` | decimal(21,4) NULL DEFAULT 0.0000 |
| `other_total` | varchar(10) NULL DEFAULT 0 |
| `shop_id` | varchar(32) NOT NULL |
| `sum_date` | varchar(10) NULL |
| `sum_month` | varchar(10) NULL |

## ✚ Database only in base: `mss-mars`

### Tables

#### ✚ Only in base: `t_store_receipt_template`

| Column | Type |
|--------|------|
| `store_receipt_template_id` | int NOT NULL auto_increment |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `receipt_template_id` | bigint NOT NULL |
| `logo_path` | varchar(256) NULL |
| `cre_time` | int NOT NULL |
| `upd_time` | int NULL |

#### ✚ Only in base: `t_receipt_token`

| Column | Type |
|--------|------|
| `receipt_token_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NULL |
| `token` | varchar(128) NULL |
| `timeout_timestamp` | bigint NULL |
| `read_time` | int NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction`

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
| `payment_method` | varchar(16) NOT NULL |
| `payment_method_sub` | varchar(16) NULL |
| `read_card_type` | varchar(16) NOT NULL |
| `card_scheme` | varchar(16) NOT NULL |
| `pan_mask` | varchar(16) NULL |
| `pan_token` | varchar(64) NULL |
| `trans_type` | varchar(64) NOT NULL |
| `batch_no` | varchar(16) NULL |
| `trace_no` | varchar(16) NULL |
| `amount` | bigint NOT NULL |
| `currency_code` | varchar(3) NOT NULL |
| `terminal_utc` | varchar(12) NOT NULL |
| `channel_code` | varchar(16) NULL |
| `channel_mid` | varchar(32) NULL |
| `channel_tid` | varchar(16) NULL |
| `channel_trans_time` | varchar(14) NULL |
| `reference_no` | varchar(160) NULL |
| `approval_code` | varchar(32) NULL |
| `trv` | varchar(16) NULL |
| `tsi` | varchar(16) NULL |
| `channel_trans_status` | int NULL |
| `trans_date` | varchar(8) NULL |
| `trans_time` | varchar(6) NULL |
| `host_resp_code` | varchar(16) NULL |
| `host_error_description` | varchar(1024) NULL |
| `receipt_flag` | tinyint(1) NOT NULL |
| `backend_remark` | varchar(1024) NULL |
| `ori_trans_uuid` | varchar(16) NULL |
| `status` | tinyint NOT NULL |
| `cashier_name` | varchar(255) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_receipt_image`

| Column | Type |
|--------|------|
| `receipt_image_id` | bigint NOT NULL auto_increment |
| `trans_uuid` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `cre_time` | int NOT NULL |

#### ✚ Only in base: `t_receipt_notice`

| Column | Type |
|--------|------|
| `receipt_notice_id` | varchar(32) NOT NULL |
| `trans_uuid` | varchar(32) NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `merch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `method` | varchar(64) NULL |
| `email` | varchar(128) NULL |
| `receipt_token` | varchar(32) NULL |
| `status` | int NOT NULL |
| `message` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_transaction_statistic`

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
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_app_key`

| Column | Type |
|--------|------|
| `dev_app_key_id` | bigint NOT NULL auto_increment |
| `app_package_name` | varchar(256) NOT NULL |
| `dev_sn` | varchar(45) NOT NULL |
| `operator_id` | bigint NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `pub_key` | varchar(2048) NOT NULL |
| `dev_trans_key` | varchar(64) NOT NULL |
| `aes_timeout_timestamp` | bigint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_order`

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
| `cashier` | varchar(128) NOT NULL |
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
| `order_status` | tinyint(1) NOT NULL |
| `card_scheme` | varchar(16) NULL |
| `payment_method` | varchar(16) NULL |
| `trans_time_local` | bigint NULL |
| `card_mask` | varchar(4) NULL |
| `receipt_flag` | tinyint(1) NOT NULL |
| `merchant_remark` | varchar(512) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_function_config`

| Column | Type |
|--------|------|
| `fun_conf_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NOT NULL |
| `param_key` | varchar(100) NOT NULL |
| `param_value` | varchar(100) NOT NULL |

#### ✚ Only in base: `t_order_goods`

| Column | Type |
|--------|------|
| `order_goods_uuid` | varchar(32) NOT NULL |
| `order_uuid` | varchar(32) NOT NULL |
| `order_no` | varchar(64) NOT NULL |
| `source_id` | varchar(64) NOT NULL |
| `goods_label` | varchar(128) NOT NULL |
| `price_unit` | bigint NOT NULL |
| `num` | int NOT NULL |
| `amount_discount_percent` | int NOT NULL |
| `amount_discount` | bigint NOT NULL |
| `goods_amount` | bigint NOT NULL |
| `description` | varchar(4096) NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_receipt_template`

| Column | Type |
|--------|------|
| `receipt_template_id` | bigint NOT NULL auto_increment |
| `content_file_path` | varchar(255) NULL |
| `label` | varchar(256) NULL |

## ✖ Database only in check: `nacos`

### Tables

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

#### ✖ Only in check: `users`

| Column | Type |
|--------|------|
| `username` | varchar(50) NOT NULL |
| `password` | varchar(500) NOT NULL |
| `enabled` | tinyint(1) NOT NULL |

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

#### ✖ Only in check: `permissions`

| Column | Type |
|--------|------|
| `role` | varchar(50) NOT NULL |
| `resource` | varchar(255) NOT NULL |
| `action` | varchar(8) NOT NULL |

#### ✖ Only in check: `roles`

| Column | Type |
|--------|------|
| `username` | varchar(50) NOT NULL |
| `role` | varchar(50) NOT NULL |

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

## ✚ Database only in base: `notification-mars`

### Tables

#### ✚ Only in base: `t_maintenance_notice`

| Column | Type |
|--------|------|
| `maintencance_id` | bigint NOT NULL auto_increment |
| `notice_channel` | int NOT NULL |
| `email_notification_config` | varchar(64) NULL |
| `status` | int NOT NULL DEFAULT 0 |
| `title` | varchar(128) NOT NULL |
| `begin_time` | bigint NOT NULL |
| `end_time` | varchar(128) NOT NULL |
| `content` | varchar(2048) NOT NULL |
| `create_time` | bigint NOT NULL |
| `update_time` | bigint NULL |

## ✚ Database only in base: `ntms`

### Tables

#### ✚ Only in base: `t_partner_ram_account`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `access_key` | varchar(16) NOT NULL |
| `security_key` | varchar(64) NOT NULL |
| `link_source` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_attach`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `path_id` | bigint NOT NULL |
| `file_name` | varchar(128) NOT NULL |
| `file_size` | int NULL |
| `file_type` | varchar(32) NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `cre_time` | datetime NOT NULL |

#### ✚ Only in base: `t_app_file`

| Column | Type |
|--------|------|
| `app_file_id` | bigint unsigned NOT NULL auto_increment |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_default_param_management`

| Column | Type |
|--------|------|
| `id_default_param_management` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `default_value` | bigint unsigned NOT NULL |
| `operation` | tinyint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_paused_trigger_grps`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |

#### ✚ Only in base: `t_param_job`

| Column | Type |
|--------|------|
| `job_id` | bigint NOT NULL auto_increment |
| `job_uuid` | varchar(32) NULL |
| `job_name` | varchar(255) NULL |
| `job_mode` | int NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NULL |
| `job_desc` | varchar(255) NULL |
| `job_status` | int NOT NULL |
| `is_cancel` | int NULL DEFAULT 0 |
| `is_bind` | int NOT NULL DEFAULT 0 |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_param_value_audit`

| Column | Type |
|--------|------|
| `id_param_value` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `ref_table_entity` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_default_param_value`

| Column | Type |
|--------|------|
| `id_default_param_value` | bigint unsigned NOT NULL auto_increment |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_entity_value` | bigint unsigned NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_layout`

| Column | Type |
|--------|------|
| `id_param_layout` | bigint unsigned NOT NULL auto_increment |
| `param_id` | bigint NULL |
| `parent` | bigint NOT NULL |
| `x` | int NOT NULL |
| `y` | int NOT NULL |
| `w` | int NOT NULL |
| `h` | int NOT NULL |
| `type` | tinyint NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_file_flow`

| Column | Type |
|--------|------|
| `flow_id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `content_sha1` | varchar(160) NULL |
| `sha1` | varchar(160) NULL |
| `version` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `release_time` | datetime NULL |
| `terminal_update_time` | datetime NULL |
| `app_update_time` | datetime NULL |
| `status` | int NULL |

#### ✚ Only in base: `t_default_param_management_audit`

| Column | Type |
|--------|------|
| `id_default_param_management` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `default_value` | bigint unsigned NOT NULL |
| `operation` | tinyint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_org`

| Column | Type |
|--------|------|
| `org_id` | bigint NOT NULL auto_increment |
| `org_name` | varchar(45) NOT NULL |
| `org_desc` | varchar(200) NULL |
| `parent_id` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_param_entity_audit_history`

| Column | Type |
|--------|------|
| `audit_history_id` | bigint unsigned NOT NULL auto_increment |
| `audit_id` | bigint unsigned NOT NULL |
| `from` | varchar(64) NOT NULL |
| `to` | varchar(64) NOT NULL |
| `status` | tinyint unsigned NOT NULL |
| `remark` | varchar(510) NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_locks`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `LOCK_NAME` | varchar(40) NOT NULL |

#### ✚ Only in base: `t_path`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `parent_id` | bigint NOT NULL |
| `folder_name` | varchar(64) NOT NULL |
| `parent_path` | varchar(1024) NULL |
| `oid` | varchar(64) NOT NULL |
| `cre_time` | datetime NOT NULL |

#### ✚ Only in base: `t_param_value`

| Column | Type |
|--------|------|
| `id_param_value` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `ref_table_entity` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_notification`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `content` | text NULL |
| `level` | int NULL |
| `status` | int NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `cre_user` | varchar(45) NULL |
| `oid` | varchar(64) NOT NULL |

#### ✚ Only in base: `t_dev_app_group`

| Column | Type |
|--------|------|
| `dev_app_group_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `app_group_id` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_user_role`

| Column | Type |
|--------|------|
| `user_role_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `role_id` | bigint NOT NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `qrtz_fired_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `ENTRY_ID` | varchar(95) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `INSTANCE_NAME` | varchar(200) NOT NULL |
| `FIRED_TIME` | bigint NOT NULL |
| `SCHED_TIME` | bigint NOT NULL |
| `PRIORITY` | int NOT NULL |
| `STATE` | varchar(16) NOT NULL |
| `JOB_NAME` | varchar(200) NULL |
| `JOB_GROUP` | varchar(200) NULL |
| `IS_NONCONCURRENT` | varchar(1) NULL |
| `REQUESTS_RECOVERY` | varchar(1) NULL |

#### ✚ Only in base: `t_table_schema`

| Column | Type |
|--------|------|
| `id_table_schema` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `description` | varchar(200) NULL |
| `parent` | bigint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_simprop_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `STR_PROP_1` | varchar(512) NULL |
| `STR_PROP_2` | varchar(512) NULL |
| `STR_PROP_3` | varchar(512) NULL |
| `INT_PROP_1` | int NULL |
| `INT_PROP_2` | int NULL |
| `LONG_PROP_1` | bigint NULL |
| `LONG_PROP_2` | bigint NULL |
| `DEC_PROP_1` | decimal(13,4) NULL |
| `DEC_PROP_2` | decimal(13,4) NULL |
| `BOOL_PROP_1` | varchar(1) NULL |
| `BOOL_PROP_2` | varchar(1) NULL |

#### ✚ Only in base: `t_app_group_detail`

| Column | Type |
|--------|------|
| `app_group_detail_id` | bigint NOT NULL auto_increment |
| `app_group_id` | bigint NOT NULL |
| `app_group_name` | varchar(60) NOT NULL |
| `app_name` | varchar(255) NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `app_file_id` | bigint NOT NULL |

#### ✚ Only in base: `t_app_template_ref`

| Column | Type |
|--------|------|
| `app_template_id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `template_modify_mark` | tinyint NULL DEFAULT 0 |

#### ✚ Only in base: `t_app_group`

| Column | Type |
|--------|------|
| `app_group_id` | bigint NOT NULL auto_increment |
| `app_group_name` | varchar(60) NOT NULL |
| `app_group_desc` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `org_id` | bigint NOT NULL |

#### ✚ Only in base: `t_device`

| Column | Type |
|--------|------|
| `dev_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |

#### ✚ Only in base: `qrtz_cron_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `CRON_EXPRESSION` | varchar(120) NOT NULL |
| `TIME_ZONE_ID` | varchar(80) NULL |

#### ✚ Only in base: `qrtz_scheduler_state`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `INSTANCE_NAME` | varchar(200) NOT NULL |
| `LAST_CHECKIN_TIME` | bigint NOT NULL |
| `CHECKIN_INTERVAL` | bigint NOT NULL |

#### ✚ Only in base: `t_param_schema`

| Column | Type |
|--------|------|
| `id_param_schema` | bigint unsigned NOT NULL auto_increment |
| `NAME` | varchar(64) NULL |
| `description` | varchar(200) NULL |
| `parent` | bigint unsigned NOT NULL |
| `key` | varchar(64) NULL |
| `length` | int NULL |
| `nullable` | tinyint unsigned NOT NULL |
| `value_type` | tinyint unsigned NOT NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_table_entity_audit`

| Column | Type |
|--------|------|
| `id_table_entity` | bigint unsigned NOT NULL auto_increment |
| `table_schema` | bigint unsigned NOT NULL |
| `name` | varchar(45) NOT NULL |
| `audit_id` | bigint unsigned NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `JOB_NAME` | varchar(200) NOT NULL |
| `JOB_GROUP` | varchar(200) NOT NULL |
| `DESCRIPTION` | varchar(250) NULL |
| `NEXT_FIRE_TIME` | bigint NULL |
| `PREV_FIRE_TIME` | bigint NULL |
| `PRIORITY` | int NULL |
| `TRIGGER_STATE` | varchar(16) NOT NULL |
| `TRIGGER_TYPE` | varchar(8) NOT NULL |
| `START_TIME` | bigint NOT NULL |
| `END_TIME` | bigint NULL |
| `CALENDAR_NAME` | varchar(200) NULL |
| `MISFIRE_INSTR` | smallint NULL |
| `JOB_DATA` | blob NULL |

#### ✚ Only in base: `t_params_template`

| Column | Type |
|--------|------|
| `id_params_template` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `description` | varchar(200) NULL |
| `entry` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |
| `oid` | varchar(64) NOT NULL |
| `file_type` | tinyint NULL DEFAULT 1 |

#### ✚ Only in base: `qrtz_job_details`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `JOB_NAME` | varchar(200) NOT NULL |
| `JOB_GROUP` | varchar(200) NOT NULL |
| `DESCRIPTION` | varchar(250) NULL |
| `JOB_CLASS_NAME` | varchar(250) NOT NULL |
| `IS_DURABLE` | varchar(1) NOT NULL |
| `IS_NONCONCURRENT` | varchar(1) NOT NULL |
| `IS_UPDATE_DATA` | varchar(1) NOT NULL |
| `REQUESTS_RECOVERY` | varchar(1) NOT NULL |
| `JOB_DATA` | blob NULL |

#### ✚ Only in base: `qrtz_blob_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `BLOB_DATA` | blob NULL |

#### ✚ Only in base: `t_param_job_detail`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `job_id` | bigint NOT NULL |
| `flow_id` | bigint NULL |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `status` | int NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_params_template_entity`

| Column | Type |
|--------|------|
| `id_params_template_entity` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `description` | varchar(200) NULL |
| `params_template` | bigint NULL |
| `table_entity_entry` | bigint NULL |
| `version` | int NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_simple_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `REPEAT_COUNT` | bigint NOT NULL |
| `REPEAT_INTERVAL` | bigint NOT NULL |
| `TIMES_TRIGGERED` | bigint NOT NULL |

#### ✚ Only in base: `t_table_entity`

| Column | Type |
|--------|------|
| `id_table_entity` | bigint unsigned NOT NULL auto_increment |
| `table_schema` | bigint unsigned NOT NULL |
| `name` | varchar(45) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_role`

| Column | Type |
|--------|------|
| `role_id` | bigint NOT NULL auto_increment |
| `role_name` | varchar(45) NOT NULL |
| `role_remark` | varchar(45) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_partner_info`

| Column | Type |
|--------|------|
| `partner_id` | bigint NOT NULL auto_increment |
| `partner_code` | varchar(32) NOT NULL |
| `partner_rsa_pub` | varchar(3072) NOT NULL |
| `partner_desc` | varchar(512) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_user`

| Column | Type |
|--------|------|
| `user_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `user_mobile` | varchar(64) NOT NULL |
| `password` | varchar(64) NULL |
| `user_status` | tinyint NULL |
| `user_type` | tinyint NOT NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `remark` | varchar(255) NULL |
| `full_name` | varchar(100) NULL |

#### ✚ Only in base: `t_user_org`

| Column | Type |
|--------|------|
| `user_org_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_power_role_resource`

| Column | Type |
|--------|------|
| `power_role_resource_id` | bigint NOT NULL auto_increment |
| `role_id` | bigint NOT NULL |
| `permission_code` | varchar(100) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_task`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `status` | int NULL |
| `total_num` | int NULL |
| `complete_num` | int NULL |
| `percent` | int NULL |
| `message` | text NULL |
| `download_url` | varchar(512) NULL |
| `start_time` | datetime NOT NULL |
| `finish_time` | datetime NULL |
| `cre_time` | datetime NOT NULL |
| `cre_user` | varchar(45) NULL |
| `oid` | varchar(64) NOT NULL |

#### ✚ Only in base: `qrtz_calendars`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `CALENDAR_NAME` | varchar(200) NOT NULL |
| `CALENDAR` | blob NOT NULL |

#### ✚ Only in base: `t_power_resource`

| Column | Type |
|--------|------|
| `power_resource_id` | bigint NOT NULL auto_increment |
| `resource_name` | varchar(50) NOT NULL |
| `resource_type` | tinyint NOT NULL |
| `priority` | int NULL |
| `resource_icon` | varchar(64) NULL |
| `resource_url` | varchar(255) NULL |
| `parent_id` | bigint NULL |
| `permission_code` | varchar(128) NULL |
| `resource_status` | tinyint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_param_entity_audit`

| Column | Type |
|--------|------|
| `audit_id` | bigint unsigned NOT NULL auto_increment |
| `app_name` | varchar(255) NOT NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `table_schema_name` | varchar(255) NOT NULL |
| `id_table_entity` | bigint unsigned NOT NULL |
| `entity_name` | varchar(255) NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `applicant` | varchar(64) NOT NULL |
| `auditor` | varchar(64) NULL |
| `operation_type` | tinyint unsigned NOT NULL |
| `action` | tinyint unsigned NOT NULL |
| `status` | tinyint unsigned NOT NULL |
| `remark` | varchar(510) NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_file_release`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `sha1` | varchar(160) NULL |
| `version` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `release_time` | datetime NULL |
| `terminal_update_time` | datetime NULL |
| `app_update_time` | datetime NULL |
| `status` | int NULL |

### Procedures

#### ✚ Only in base: `finishJob`

```sql
BEGIN select jobId; END
```

#### ✚ Only in base: `pressure_pro`

```sql
BEGIN declare templateNum int DEFAULT 1; declare terminalTableNum int DEFAULT 1; declare linkTableNum int DEFAULT 2; declare terminalParamNum int DEFAULT 60; declare linkParamNum int DEFAULT 2; declare terminalEntityNum int DEFAULT 1; declare linkEntityNum int DEFAULT 2; declare appGroup_id int default 0; declare appFile_id int default 0; declare tpl_id int default 0; declare terminalTable_id int default 0; declare linkTable_id int default 0; declare terminalEntity_id int default 0; declare linkEntity_id int default 0; declare paramSchema_id int default 0; declare terminalEntity_idRecord int default 0; declare i int DEFAULT 1; declare j int DEFAULT 1; declare k int DEFAULT 1; declare m int DEFAULT 1; declare n int DEFAULT 1; declare o int DEFAULT 1; declare p int DEFAULT 1; declare q int DEFAULT 1; set terminalEntityNum = terminalNum; insert into t_app_group(app_group_name, cre_time, org_id)values( 'test1', SYSDATE(), 1 ); set appGroup_id = @@IDENTITY; WHILE (i <= templateNum) DO insert into t_app_file(app_package_name, app_name, cre_time)VALUES( CONCAT('com.test',i), CONCAT('压测',i), SYSDATE() ); set appFile_id = @@IDENTITY; insert into t_app_group_detail(app_group_id, app_group_name, app_name, app_package_name, cre_time, app_file_id)values( appGroup_id, 'test1', CONCAT('压测',i), CONCAT('com.test',i), SYSDATE(), appFile_id ); insert into t_params_template(`name`, entry, create_time)VALUES( CONCAT('template',i), null, SYSDATE() ); set tpl_id = @@IDENTITY; insert into t_app_template_ref(template_id, app_package_name, app_name, cre_time)VALUES( tpl_id, CONCAT('com.test',i), CONCAT('压测',i), SYSDATE() ); insert into t_table_schema(`name`, parent, create_time)VALUES( 'Profile', tpl_id, SYSDATE() ); set terminalTable_id = @@IDENTITY; update t_params_template set entry = terminalTable_id where id_params_template = tpl_id; set n =1; WHILE (n <= linkTableNum) DO insert into t_table_schema(`name`, parent, create_time)VALUES( CONCAT(tpl_id,'-Link-',n), tpl_id, SYSDATE() ); set linkTable_id = @@IDENTITY; set o = 1; WHILE (o <= linkEntityNum) DO insert into t_table_entity(table_schema, `name`, create_time)VALUES( linkTable_id, CONCAT('L-',o), SYSDATE() ); set linkEntity_id = @@IDENTITY; if o <=1 then set p =1; WHILE (p <= linkParamNum) DO insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(linkTable_id,'-L-',p),linkTable_id,CONCAT(linkTable_id,'-L-',p), 10, 1, 1, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; insert into t_param_layout(param_id, parent, x, y, w, h, type, create_time)VALUES( paramSchema_id, linkTable_id, 0,0,1,1,1,SYSDATE() ); set q = 1; WHILE (q <= linkEntityNum) DO insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( linkEntity_id, paramSchema_id, 1, q, null, null, null,null, SYSDATE() ); SET q = q+1; END WHILE; SET p = p+1; END WHILE; end if; SET o = o+1; END WHILE; set n = n+1; END WHILE; set k = 1; WHILE (k <= terminalEntityNum) DO insert into t_table_entity(table_schema, `name`, create_time)VALUES( terminalTable_id, CONCAT('SN',k), SYSDATE() ); set terminalEntity_id = @@IDENTITY; insert into t_device(dev_sn, cre_time, org_id)VALUES( CONCAT('SN',k), SYSDATE(), 1 ); insert into t_dev_app_group(dev_sn, app_group_id, cre_time)VALUES( CONCAT('SN',k), appGroup_id, SYSDATE() ); if k <=1 then set j =1; WHILE (j <= terminalParamNum * terminalTableNum) DO if j <= 20 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 5, linkTable_id, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m = 1; WHILE (m <= terminalNum) DO set terminalEntity_idRecord = terminalEntity_id+m; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( terminalEntity_idRecord-1, paramSchema_id, 5, null, null, null, linkTable_id,linkEntity_id, SYSDATE() ); set m = m +1; END WHILE; ELSEIF j <=terminalParamNum * terminalTableNum/5*2 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 1, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m = 1; WHILE (m <= terminalNum) DO set terminalEntity_idRecord = terminalEntity_id+m; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( terminalEntity_idRecord-1, paramSchema_id, 1, m, null, null, null,null, SYSDATE() ); set m = m +1; END WHILE; ELSEIF j <=terminalParamNum * terminalTableNum/5*3 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 2, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m = 1; WHILE (m <= terminalNum) DO set terminalEntity_idRecord = terminalEntity_id+m; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( terminalEntity_idRecord-1, paramSchema_id, 2, null, 1, null, null,null, SYSDATE() ); set m = m +1; END WHILE; ELSEIF j <=terminalParamNum * terminalTableNum/5*4 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 3, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m = 1; WHILE (m <= terminalNum) DO set terminalEntity_idRecord = terminalEntity_id+m; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( terminalEntity_idRecord-1, paramSchema_id, 3, '10', null, null, null,null,SYSDATE() ); set m = m +1; END WHILE; ELSE insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 4, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m=1; WHILE (m <= terminalNum) DO set terminalEntity_idRecord = terminalEntity_id+m; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( terminalEntity_idRecord-1, paramSchema_id, 4, null, null, UNHEX(HEX(16)), null,null, SYSDATE() ); set m = m +1; END WHILE; end if; insert into t_param_layout(param_id, parent, x, y, w, h, type, create_time)VALUES( paramSchema_id, terminalTable_id, 0,0,1,1,1,SYSDATE() ); set j = j+1; END WHILE; END if; set k = k+1; END WHILE; set i = i+1; END WHILE; END
```

#### ✚ Only in base: `pressure_pro2`

```sql
BEGIN declare templateNum int DEFAULT 1; declare terminalTableNum int DEFAULT 1; declare linkTableNum int DEFAULT 49; declare terminalParamNum int DEFAULT 500; declare linkParamNum int DEFAULT 50; declare terminalEntityNum int DEFAULT 1; declare linkEntityNum int DEFAULT 50; declare tplName_prefix VARCHAR(255) DEFAULT 'tpl4'; declare pkgName_prefix VARCHAR(255) DEFAULT 'com.test4'; declare appName_prefix VARCHAR(255) DEFAULT '压测4'; declare orgId int DEFAULT 1; declare orgName VARCHAR(255) DEFAULT 'test'; declare tpl_id int default 0; declare terminalTable_id int default 0; declare linkTable_id int default 0; declare terminalEntity_id int default 0; declare linkEntity_id int default 0; declare paramSchema_id int default 0; declare linkEntity_idRecord int default 0; declare terminalEntity_idRecord int default 0; declare i int DEFAULT 1; declare j int DEFAULT 1; declare k int DEFAULT 1; declare m int DEFAULT 1; declare n int DEFAULT 1; declare o int DEFAULT 1; declare p int DEFAULT 1; declare q int DEFAULT 1; set terminalEntityNum = terminalNum; WHILE (i <= templateNum) DO insert into t_params_template(`name`, app_package_name, app_name, oid, entry, create_time)VALUES( CONCAT(tplName_prefix,i), CONCAT(pkgName_prefix,i), CONCAT(appName_prefix,i), '1BD291A39C39517B1E970C2CD0A98FBE1B61D899876881B1A1BEA5B3F6D0F33B', null, SYSDATE() ); set tpl_id = @@IDENTITY; insert into t_table_schema(`name`, parent, create_time)VALUES( 'Profile', tpl_id, SYSDATE() ); set terminalTable_id = @@IDENTITY; update t_params_template set entry = terminalTable_id where id_params_template = tpl_id; set n =1; WHILE (n <= linkTableNum) DO insert into t_table_schema(`name`, parent, create_time)VALUES( CONCAT(tpl_id,'-Link-',n), tpl_id, SYSDATE() ); set linkTable_id = @@IDENTITY; set o = 1; WHILE (o <= linkEntityNum) DO insert into t_table_entity(table_schema, `name`, org_id, org_name, create_time)VALUES( linkTable_id, CONCAT('L-',o), orgId, orgName, SYSDATE() ); set linkEntity_id = @@IDENTITY; if o <=1 then set p =1; WHILE (p <= linkParamNum) DO insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(linkTable_id,'-L-',p),linkTable_id,CONCAT(linkTable_id,'-L-',p), 10, 1, 1, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; insert into t_param_layout(param_id, parent, x, y, w, h, type, create_time)VALUES( paramSchema_id, linkTable_id, 0,0,1,1,1,SYSDATE() ); set q = 1; WHILE (q <= linkEntityNum) DO set linkEntity_idRecord = linkEntity_id + q; insert into t_param_value(parent, param_schema, param_type, string_value, boolean_value, blob_value, ref_table_schema, ref_table_entity, create_time)VALUES( linkEntity_idRecord-1, paramSchema_id, 1, q, null, null, null,null, SYSDATE() ); SET q = q+1; END WHILE; SET p = p+1; END WHILE; end if; SET o = o+1; END WHILE; set n = n+1; END WHILE; set k = 1; WHILE (k <= terminalEntityNum) DO insert into t_table_entity(table_schema, `name`, org_id, org_name, create_time)VALUES( terminalTable_id, CONCAT('SN',k), orgId, orgName, SYSDATE() ); set terminalEntity_id = @@IDENTITY; if k <=1 then set j =1; WHILE (j <= terminalParamNum * terminalTableNum) DO if j <= 20 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 5, linkTable_id, SYSDATE() ); set paramSchema_id = @@IDENTITY; ELSEIF j <=terminalParamNum * terminalTableNum/5*2 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 1, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; ELSEIF j <=terminalParamNum * terminalTableNum/5*3 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 2, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; set m = 1; ELSEIF j <=terminalParamNum * terminalTableNum/5*4 then insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 3, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; ELSE insert into t_param_schema(`name`, parent, `key`, length, nullable, value_type, ref_table_schema, create_time)VALUES( CONCAT(terminalTable_id,'-T-',j),terminalTable_id,CONCAT(terminalTable_id,'-T-',j), 10, 1, 4, null, SYSDATE() ); set paramSchema_id = @@IDENTITY; end if; insert into t_param_layout(param_id, parent, x, y, w, h, type, create_time)VALUES( paramSchema_id, terminalTable_id, 0,0,1,1,1,SYSDATE() ); set j = j+1; END WHILE; END if; set k = k+1; END WHILE; set i = i+1; END WHILE; END
```

#### ✚ Only in base: `license_pro`

```sql
BEGIN declare appNum int DEFAULT 1; END
```

## ✚ Database only in base: `ntms-mars`

### Tables

#### ✚ Only in base: `t_app_file`

| Column | Type |
|--------|------|
| `app_file_id` | bigint unsigned NOT NULL auto_increment |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_power_role_resource`

| Column | Type |
|--------|------|
| `power_role_resource_id` | bigint NOT NULL auto_increment |
| `role_id` | bigint NOT NULL |
| `permission_code` | varchar(100) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_param_job_detail`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `job_id` | bigint NOT NULL |
| `flow_id` | bigint NULL |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `status` | int NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_param_file_flow`

| Column | Type |
|--------|------|
| `flow_id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `content_sha1` | varchar(160) NULL |
| `sha1` | varchar(160) NULL |
| `version` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `release_time` | datetime NULL |
| `terminal_update_time` | datetime NULL |
| `app_update_time` | datetime NULL |
| `status` | int NULL |

#### ✚ Only in base: `t_param_schema`

| Column | Type |
|--------|------|
| `id_param_schema` | bigint unsigned NOT NULL auto_increment |
| `NAME` | varchar(64) NULL |
| `description` | varchar(200) NULL |
| `parent` | bigint unsigned NOT NULL |
| `key` | varchar(64) NULL |
| `length` | int NULL |
| `nullable` | tinyint unsigned NOT NULL |
| `value_type` | tinyint unsigned NOT NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_role`

| Column | Type |
|--------|------|
| `role_id` | bigint NOT NULL auto_increment |
| `role_name` | varchar(45) NOT NULL |
| `role_remark` | varchar(45) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `qrtz_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `JOB_NAME` | varchar(200) NOT NULL |
| `JOB_GROUP` | varchar(200) NOT NULL |
| `DESCRIPTION` | varchar(250) NULL |
| `NEXT_FIRE_TIME` | bigint NULL |
| `PREV_FIRE_TIME` | bigint NULL |
| `PRIORITY` | int NULL |
| `TRIGGER_STATE` | varchar(16) NOT NULL |
| `TRIGGER_TYPE` | varchar(8) NOT NULL |
| `START_TIME` | bigint NOT NULL |
| `END_TIME` | bigint NULL |
| `CALENDAR_NAME` | varchar(200) NULL |
| `MISFIRE_INSTR` | smallint NULL |
| `JOB_DATA` | blob NULL |

#### ✚ Only in base: `t_params_template`

| Column | Type |
|--------|------|
| `id_params_template` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `app_name` | varchar(255) NOT NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `description` | varchar(200) NULL |
| `entry` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |
| `oid` | varchar(64) NOT NULL |
| `file_type` | tinyint NULL DEFAULT 1 |

#### ✚ Only in base: `t_table_entity`

| Column | Type |
|--------|------|
| `id_table_entity` | bigint unsigned NOT NULL auto_increment |
| `table_schema` | bigint unsigned NOT NULL |
| `name` | varchar(45) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_partner_info`

| Column | Type |
|--------|------|
| `partner_id` | bigint NOT NULL auto_increment |
| `partner_code` | varchar(32) NOT NULL |
| `partner_rsa_pub` | varchar(3072) NOT NULL |
| `partner_desc` | varchar(512) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_task`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `status` | int NULL |
| `total_num` | int NULL |
| `complete_num` | int NULL |
| `percent` | int NULL |
| `message` | text NULL |
| `download_url` | varchar(512) NULL |
| `start_time` | datetime NOT NULL |
| `finish_time` | datetime NULL |
| `cre_time` | datetime NOT NULL |
| `cre_user` | varchar(45) NULL |
| `oid` | varchar(64) NOT NULL |

#### ✚ Only in base: `qrtz_scheduler_state`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `INSTANCE_NAME` | varchar(200) NOT NULL |
| `LAST_CHECKIN_TIME` | bigint NOT NULL |
| `CHECKIN_INTERVAL` | bigint NOT NULL |

#### ✚ Only in base: `t_device`

| Column | Type |
|--------|------|
| `dev_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |

#### ✚ Only in base: `t_user`

| Column | Type |
|--------|------|
| `user_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `user_mobile` | varchar(64) NOT NULL |
| `password` | varchar(64) NULL |
| `user_status` | tinyint NULL |
| `user_type` | tinyint NOT NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |
| `remark` | varchar(255) NULL |
| `full_name` | varchar(100) NULL |

#### ✚ Only in base: `qrtz_cron_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `CRON_EXPRESSION` | varchar(120) NOT NULL |
| `TIME_ZONE_ID` | varchar(80) NULL |

#### ✚ Only in base: `t_app_group`

| Column | Type |
|--------|------|
| `app_group_id` | bigint NOT NULL auto_increment |
| `app_group_name` | varchar(60) NOT NULL |
| `app_group_desc` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `org_id` | bigint NOT NULL |

#### ✚ Only in base: `t_user_org`

| Column | Type |
|--------|------|
| `user_org_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_operator_config`

| Column | Type |
|--------|------|
| `operator_config_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `param_key` | varchar(100) NOT NULL |
| `param_value` | varchar(100) NOT NULL |

#### ✚ Only in base: `qrtz_blob_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `BLOB_DATA` | blob NULL |

#### ✚ Only in base: `qrtz_job_details`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `JOB_NAME` | varchar(200) NOT NULL |
| `JOB_GROUP` | varchar(200) NOT NULL |
| `DESCRIPTION` | varchar(250) NULL |
| `JOB_CLASS_NAME` | varchar(250) NOT NULL |
| `IS_DURABLE` | varchar(1) NOT NULL |
| `IS_NONCONCURRENT` | varchar(1) NOT NULL |
| `IS_UPDATE_DATA` | varchar(1) NOT NULL |
| `REQUESTS_RECOVERY` | varchar(1) NOT NULL |
| `JOB_DATA` | blob NULL |

#### ✚ Only in base: `t_table_entity_audit`

| Column | Type |
|--------|------|
| `id_table_entity` | bigint unsigned NOT NULL auto_increment |
| `table_schema` | bigint unsigned NOT NULL |
| `name` | varchar(45) NOT NULL |
| `audit_id` | bigint unsigned NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_entity_audit`

| Column | Type |
|--------|------|
| `audit_id` | bigint unsigned NOT NULL auto_increment |
| `app_name` | varchar(255) NOT NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `table_schema_name` | varchar(255) NOT NULL |
| `id_table_entity` | bigint unsigned NOT NULL |
| `entity_name` | varchar(255) NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `applicant` | varchar(64) NOT NULL |
| `auditor` | varchar(64) NULL |
| `operation_type` | tinyint unsigned NOT NULL |
| `action` | tinyint unsigned NOT NULL |
| `status` | tinyint unsigned NOT NULL |
| `remark` | varchar(510) NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_org`

| Column | Type |
|--------|------|
| `org_id` | bigint NOT NULL auto_increment |
| `org_name` | varchar(45) NOT NULL |
| `org_desc` | varchar(200) NULL |
| `parent_id` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `qrtz_calendars`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `CALENDAR_NAME` | varchar(200) NOT NULL |
| `CALENDAR` | blob NOT NULL |

#### ✚ Only in base: `t_notification`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `content` | text NULL |
| `level` | int NULL |
| `status` | int NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `cre_user` | varchar(45) NULL |
| `oid` | varchar(64) NOT NULL |

#### ✚ Only in base: `t_default_param_management_audit`

| Column | Type |
|--------|------|
| `id_default_param_management` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `default_value` | bigint unsigned NOT NULL |
| `operation` | tinyint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_app_template_ref`

| Column | Type |
|--------|------|
| `app_template_id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `template_modify_mark` | tinyint NULL DEFAULT 0 |

#### ✚ Only in base: `qrtz_simprop_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `STR_PROP_1` | varchar(512) NULL |
| `STR_PROP_2` | varchar(512) NULL |
| `STR_PROP_3` | varchar(512) NULL |
| `INT_PROP_1` | int NULL |
| `INT_PROP_2` | int NULL |
| `LONG_PROP_1` | bigint NULL |
| `LONG_PROP_2` | bigint NULL |
| `DEC_PROP_1` | decimal(13,4) NULL |
| `DEC_PROP_2` | decimal(13,4) NULL |
| `BOOL_PROP_1` | varchar(1) NULL |
| `BOOL_PROP_2` | varchar(1) NULL |

#### ✚ Only in base: `t_default_param_management`

| Column | Type |
|--------|------|
| `id_default_param_management` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `default_value` | bigint unsigned NOT NULL |
| `operation` | tinyint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_path`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `parent_id` | bigint NOT NULL |
| `folder_name` | varchar(64) NOT NULL |
| `parent_path` | varchar(1024) NULL |
| `oid` | varchar(64) NOT NULL |
| `cre_time` | datetime NOT NULL |

#### ✚ Only in base: `t_param_file_release`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `template_id` | bigint NOT NULL |
| `app_package_name` | varchar(255) NULL |
| `app_name` | varchar(255) NULL |
| `dev_sn` | varchar(32) NULL |
| `sha1` | varchar(160) NULL |
| `version` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `release_time` | datetime NULL |
| `terminal_update_time` | datetime NULL |
| `app_update_time` | datetime NULL |
| `status` | int NULL |

#### ✚ Only in base: `t_attach`

| Column | Type |
|--------|------|
| `id` | bigint unsigned NOT NULL auto_increment |
| `path_id` | bigint NOT NULL |
| `file_name` | varchar(128) NOT NULL |
| `file_size` | int NULL |
| `file_type` | varchar(32) NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `cre_time` | datetime NOT NULL |

#### ✚ Only in base: `t_param_job`

| Column | Type |
|--------|------|
| `job_id` | bigint NOT NULL auto_increment |
| `job_uuid` | varchar(32) NULL |
| `job_name` | varchar(255) NULL |
| `job_mode` | int NOT NULL |
| `oid` | varchar(64) NOT NULL |
| `org_id` | bigint NOT NULL |
| `org_name` | varchar(64) NULL |
| `job_desc` | varchar(255) NULL |
| `job_status` | int NOT NULL |
| `is_cancel` | int NULL DEFAULT 0 |
| `is_bind` | int NOT NULL DEFAULT 0 |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_partner_ram_account`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `access_key` | varchar(16) NOT NULL |
| `security_key` | varchar(64) NOT NULL |
| `link_source` | varchar(32) NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_power_resource`

| Column | Type |
|--------|------|
| `power_resource_id` | bigint NOT NULL auto_increment |
| `resource_name` | varchar(50) NOT NULL |
| `resource_type` | tinyint NOT NULL |
| `priority` | int NULL |
| `resource_icon` | varchar(64) NULL |
| `resource_url` | varchar(255) NULL |
| `parent_id` | bigint NULL |
| `permission_code` | varchar(128) NULL |
| `resource_status` | tinyint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `qrtz_fired_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `ENTRY_ID` | varchar(95) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `INSTANCE_NAME` | varchar(200) NOT NULL |
| `FIRED_TIME` | bigint NOT NULL |
| `SCHED_TIME` | bigint NOT NULL |
| `PRIORITY` | int NOT NULL |
| `STATE` | varchar(16) NOT NULL |
| `JOB_NAME` | varchar(200) NULL |
| `JOB_GROUP` | varchar(200) NULL |
| `IS_NONCONCURRENT` | varchar(1) NULL |
| `REQUESTS_RECOVERY` | varchar(1) NULL |

#### ✚ Only in base: `t_param_layout`

| Column | Type |
|--------|------|
| `id_param_layout` | bigint unsigned NOT NULL auto_increment |
| `param_id` | bigint NULL |
| `parent` | bigint NOT NULL |
| `x` | int NOT NULL |
| `y` | int NOT NULL |
| `w` | int NOT NULL |
| `h` | int NOT NULL |
| `type` | tinyint NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_value`

| Column | Type |
|--------|------|
| `id_param_value` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `ref_table_entity` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_app_group_detail`

| Column | Type |
|--------|------|
| `app_group_detail_id` | bigint NOT NULL auto_increment |
| `app_group_id` | bigint NOT NULL |
| `app_group_name` | varchar(60) NOT NULL |
| `app_name` | varchar(255) NULL |
| `app_package_name` | varchar(255) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |
| `app_file_id` | bigint NOT NULL |

#### ✚ Only in base: `qrtz_paused_trigger_grps`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |

#### ✚ Only in base: `t_table_schema`

| Column | Type |
|--------|------|
| `id_table_schema` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `description` | varchar(200) NULL |
| `parent` | bigint unsigned NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_param_entity_audit_history`

| Column | Type |
|--------|------|
| `audit_history_id` | bigint unsigned NOT NULL auto_increment |
| `audit_id` | bigint unsigned NOT NULL |
| `from` | varchar(64) NOT NULL |
| `to` | varchar(64) NOT NULL |
| `status` | tinyint unsigned NOT NULL |
| `remark` | varchar(510) NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_simple_triggers`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `TRIGGER_NAME` | varchar(200) NOT NULL |
| `TRIGGER_GROUP` | varchar(200) NOT NULL |
| `REPEAT_COUNT` | bigint NOT NULL |
| `REPEAT_INTERVAL` | bigint NOT NULL |
| `TIMES_TRIGGERED` | bigint NOT NULL |

#### ✚ Only in base: `t_params_template_entity`

| Column | Type |
|--------|------|
| `id_params_template_entity` | bigint unsigned NOT NULL auto_increment |
| `name` | varchar(45) NOT NULL |
| `description` | varchar(200) NULL |
| `params_template` | bigint NULL |
| `table_entity_entry` | bigint NULL |
| `version` | int NOT NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_default_param_value`

| Column | Type |
|--------|------|
| `id_default_param_value` | bigint unsigned NOT NULL auto_increment |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_entity_value` | bigint unsigned NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `qrtz_locks`

| Column | Type |
|--------|------|
| `SCHED_NAME` | varchar(120) NOT NULL |
| `LOCK_NAME` | varchar(40) NOT NULL |

#### ✚ Only in base: `t_param_value_audit`

| Column | Type |
|--------|------|
| `id_param_value` | bigint unsigned NOT NULL auto_increment |
| `parent` | bigint unsigned NOT NULL |
| `param_schema` | bigint unsigned NOT NULL |
| `param_type` | tinyint unsigned NOT NULL |
| `string_value` | varchar(255) NULL |
| `boolean_value` | tinyint unsigned NULL |
| `blob_value` | tinyblob NULL |
| `ref_table_schema` | bigint unsigned NULL |
| `ref_table_entity` | bigint unsigned NULL |
| `create_time` | datetime NOT NULL |
| `update_time` | datetime NULL |

#### ✚ Only in base: `t_dev_app_group`

| Column | Type |
|--------|------|
| `dev_app_group_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `app_group_id` | bigint NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NULL |

#### ✚ Only in base: `t_user_role`

| Column | Type |
|--------|------|
| `user_role_id` | bigint NOT NULL auto_increment |
| `user_name` | varchar(100) NOT NULL |
| `role_id` | bigint NOT NULL |
| `cre_time` | datetime NULL |
| `upd_time` | datetime NULL |

## ✚ Database only in base: `opennl`

### Tables

#### ✚ Only in base: `t_store`

| Column | Type |
|--------|------|
| `store_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `store_name` | varchar(32) NOT NULL |
| `app_source` | tinyint(1) NOT NULL |
| `filter_type` | tinyint(1) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_device_tag`

| Column | Type |
|--------|------|
| `dev_tag_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `operator_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `character_coce` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `key_org_id` | bigint NULL |
| `tag` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_transaction`

| Column | Type |
|--------|------|
| `trans_id` | bigint unsigned NOT NULL auto_increment |
| `order_no` | varchar(32) NULL |
| `trans_code` | varchar(8) NOT NULL |
| `channel_code` | varchar(16) NOT NULL |
| `channel_mrch_no` | varchar(16) NOT NULL |
| `channel_term_no` | varchar(16) NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint unsigned NOT NULL |
| `character_name` | varchar(32) NULL |
| `account_id` | bigint unsigned NULL |
| `device_sn` | varchar(32) NULL |
| `reference_no` | varchar(32) NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(10,2) NOT NULL |
| `card_no_mask` | varchar(20) NULL |
| `cre_time` | bigint unsigned NOT NULL |
| `upd_time` | bigint unsigned NULL |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(256) NULL |

#### ✚ Only in base: `t_device`

| Column | Type |
|--------|------|
| `dev_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NULL |
| `manufacturer_code` | varchar(8) NULL |
| `manufacturer_name` | varchar(45) NULL |
| `model_code` | varchar(16) NULL |
| `bind_operator_id` | bigint NULL |
| `bind_oid` | varchar(64) NULL |
| `bind_time` | bigint NULL |
| `run_mode` | tinyint(1) NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `activate_time` | bigint NULL |
| `cre_type` | tinyint(1) NULL |

#### ✚ Only in base: `t_application_tags`

| Column | Type |
|--------|------|
| `app_tags_id` | bigint NOT NULL auto_increment |
| `app_id` | binary(10) NOT NULL |
| `package_name` | varchar(128) NOT NULL |
| `tags` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_settlement_account`

| Column | Type |
|--------|------|
| `settlement_account_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `country` | varchar(32) NOT NULL |
| `prov_name` | varchar(32) NOT NULL |
| `city_name` | varchar(32) NOT NULL |
| `address` | varchar(256) NOT NULL |
| `bank_code` | varchar(32) NOT NULL |
| `bank_name` | varchar(128) NOT NULL |
| `account_no` | varchar(32) NOT NULL |
| `account_name` | varchar(64) NOT NULL |
| `main_flag` | tinyint(1) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_device_application`

| Column | Type |
|--------|------|
| `dev_app_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `package_name` | varchar(64) NOT NULL |
| `app_id` | bigint NULL |
| `app_version` | varchar(16) NOT NULL |
| `app_version_code` | bigint NOT NULL |
| `app_version_id` | bigint NULL |
| `app_install_time` | bigint NOT NULL |
| `upd_time` | bigint NOT NULL |

#### ✚ Only in base: `t_application_strategy`

| Column | Type |
|--------|------|
| `app_strategy_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `charge_strategy` | tinyint NOT NULL DEFAULT 1 |
| `subscribe_type` | tinyint(1) NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(7,2) unsigned NOT NULL DEFAULT 0.00 |
| `cre_time` | bigint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_application_download`

| Column | Type |
|--------|------|
| `app_down_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `app_version_id` | bigint NOT NULL |
| `app_version` | varchar(16) NOT NULL |
| `app_version_code` | bigint NOT NULL |
| `download_time` | bigint NOT NULL |
| `install_time` | bigint NULL |
| `reason` | tinyint(1) NOT NULL |
| `old_app_version` | varchar(16) NULL |
| `old_app_version_code` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_device_ca`

| Column | Type |
|--------|------|
| `dev_ca_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `ca_cn` | varchar(64) NOT NULL |
| `certificate_content` | varchar(2048) NOT NULL |
| `cre_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_application_version`

| Column | Type |
|--------|------|
| `app_version_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `package_name` | varchar(64) NOT NULL |
| `app_version` | varchar(32) NOT NULL |
| `app_version_code` | int unsigned NOT NULL |
| `app_size` | decimal(6,2) unsigned NOT NULL |
| `file_name` | varchar(128) NOT NULL |
| `charge_strategy` | tinyint NOT NULL DEFAULT 1 |
| `subscribe_type` | tinyint(1) NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(10,2) NOT NULL |
| `app_icon_url` | varchar(128) NOT NULL |
| `auto_fit` | tinyint(1) NOT NULL |
| `portrait` | tinyint(1) NOT NULL |
| `landscape` | tinyint(1) NOT NULL |
| `upload_user_id` | bigint NOT NULL |
| `download_count` | bigint unsigned NOT NULL |
| `release_type` | tinyint(1) NOT NULL |
| `release_phase` | tinyint(1) NULL |
| `main_language` | varchar(8) NOT NULL |
| `support_language` | varchar(64) NULL |
| `submit_time` | bigint NULL |
| `examine_time` | bigint NULL |
| `release_time` | bigint NULL |
| `examine_user_id` | bigint NULL |
| `app_introduction` | varchar(512) NULL |
| `revision` | varchar(1024) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_character_service`

| Column | Type |
|--------|------|
| `character_service_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `service_no` | varchar(16) NOT NULL |
| `service_name` | varchar(32) NOT NULL |
| `balance` | decimal(10,2) NOT NULL |
| `expire_time` | bigint NOT NULL |

#### ✚ Only in base: `t_order_voucher`

| Column | Type |
|--------|------|
| `order_voucher_id` | bigint NOT NULL auto_increment |
| `order_no` | varchar(32) NOT NULL |
| `voucher_no` | varchar(32) NOT NULL |
| `voucher_img_url` | varchar(128) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_store_application_blacklist`

| Column | Type |
|--------|------|
| `store_app_backlist_id` | bigint unsigned NOT NULL auto_increment |
| `store_id` | bigint unsigned NOT NULL |
| `app_id` | bigint unsigned NOT NULL |
| `cre_time` | bigint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | bigint unsigned NULL DEFAULT 0 |

#### ✚ Only in base: `t_order`

| Column | Type |
|--------|------|
| `order_no` | varchar(32) NOT NULL |
| `order_type` | tinyint NOT NULL |
| `ori_order_no` | varchar(32) NULL |
| `order_title` | varchar(32) NOT NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(10,2) NOT NULL |
| `dev_sn` | varchar(32) NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `user_id` | bigint NOT NULL |
| `user_name` | varchar(32) NOT NULL |
| `describe` | varchar(128) NULL |
| `operator_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `agent_id` | bigint NULL |
| `sett_type` | tinyint(1) NOT NULL |
| `sett_account_id` | bigint NULL |
| `sett_time` | bigint NULL |
| `bank_card_no_mask` | varchar(20) NULL |
| `voucher_code` | varchar(32) NULL |
| `trans_id` | bigint NULL |
| `status` | tinyint(1) NOT NULL |
| `examine_time` | bigint NULL |
| `examine_character_code` | varchar(16) NULL |
| `examine_key_id` | bigint NULL |
| `examine_user_id` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_category`

| Column | Type |
|--------|------|
| `category_id` | bigint unsigned NOT NULL auto_increment |
| `category_name` | varchar(32) NOT NULL |
| `category_icon_url` | varchar(128) NOT NULL |

#### ✚ Only in base: `t_purchase`

| Column | Type |
|--------|------|
| `purchase_no` | varchar(18) NOT NULL |
| `order_no` | varchar(32) NOT NULL |
| `mrch_id` | bigint unsigned NOT NULL |
| `app_id` | bigint NOT NULL |
| `package_name` | varchar(64) NOT NULL |
| `dev_sn` | varchar(32) NULL |
| `charge_strategy` | tinyint unsigned NOT NULL DEFAULT 1 |
| `subscribe_type` | tinyint(1) NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(7,2) NOT NULL DEFAULT 0.00 |
| `next_deduct_time` | bigint unsigned NOT NULL |
| `status` | tinyint unsigned NOT NULL |
| `description` | varchar(128) NOT NULL |
| `cre_time` | bigint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | bigint unsigned NOT NULL DEFAULT 0 |

#### ✚ Only in base: `t_application_examine`

| Column | Type |
|--------|------|
| `app_examine_id` | bigint NOT NULL auto_increment |
| `app_version_id` | bigint NOT NULL |
| `examine_step` | tinyint NOT NULL |
| `examine_info` | varchar(255) NULL |
| `attach_uuid_1` | bigint NULL |
| `attach_uuid_2` | bigint NULL |
| `attach_uuid_3` | bigint NULL |
| `examine_user_id` | bigint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NOT NULL |

#### ✚ Only in base: `t_application`

| Column | Type |
|--------|------|
| `app_id` | bigint NOT NULL auto_increment |
| `lastest_version_id` | bigint unsigned NOT NULL DEFAULT 0 |
| `lastest_version_name` | varchar(16) NULL |
| `app_name` | varchar(32) NOT NULL |
| `package_name` | varchar(64) NOT NULL |
| `app_introduction` | varchar(512) NOT NULL |
| `app_category_id` | bigint NOT NULL |
| `category_name` | varchar(32) NOT NULL |
| `release_target` | tinyint(1) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint unsigned NOT NULL |
| `key_name` | varchar(128) NOT NULL |
| `app_icon_url` | varchar(128) NULL |
| `download_count` | bigint unsigned NOT NULL DEFAULT 0 |
| `auto_fit` | tinyint(1) NOT NULL |
| `portrait` | tinyint(1) NOT NULL |
| `landscape` | tinyint(1) NOT NULL |
| `bankcard_app` | tinyint(1) NULL |
| `high_quality` | tinyint(1) NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `main_language` | varchar(8) NOT NULL |
| `support_language` | varchar(64) NULL |
| `score_average` | decimal(4,2) NULL |
| `appraise_count` | int NULL |
| `trial_days` | tinyint unsigned NOT NULL DEFAULT 0 |

#### ✚ Only in base: `t_application_appraise`

| Column | Type |
|--------|------|
| `app_appraise_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `app_version_id` | bigint NULL |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `user_id` | bigint NULL |
| `content` | varchar(255) NULL |
| `score` | int NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_extend`

| Column | Type |
|--------|------|
| `dev_extend_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `longitude` | decimal(12,8) NULL |
| `latitude` | decimal(12,8) NULL |
| `source` | varchar(16) NULL |
| `lbs` | varchar(256) NULL |
| `android_version` | varchar(16) NULL |
| `ota_version` | varchar(16) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `address` | varchar(128) NULL |
| `login_token` | varchar(64) NULL |
| `last_login_time` | bigint NULL |

#### ✚ Only in base: `t_cert`

| Column | Type |
|--------|------|
| `cert_id` | bigint NOT NULL auto_increment |
| `type` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `sn` | varchar(32) NULL |
| `cn` | varchar(128) NOT NULL |
| `ou` | varchar(128) NULL |
| `o` | varchar(128) NULL |
| `sha1` | varchar(64) NOT NULL |
| `pub_key` | varchar(2048) NULL |
| `issuer_cn` | varchar(128) NULL |
| `issuer_ou` | varchar(128) NULL |
| `issuer_o` | varchar(128) NULL |
| `issuer_sn` | varchar(32) NULL |
| `file_path` | varchar(128) NULL |
| `oid` | varchar(64) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `oss_custom_id` | varchar(45) NULL |
| `oss_cert_id` | varchar(45) NULL |

#### ✚ Only in base: `t_tag`

| Column | Type |
|--------|------|
| `tag_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `org_id` | bigint NULL |
| `tag` | varchar(32) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_withdrawal`

| Column | Type |
|--------|------|
| `withdrawal_id` | bigint unsigned NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint unsigned NOT NULL |
| `user_id` | bigint unsigned NOT NULL |
| `account_id` | bigint unsigned NOT NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(10,0) NOT NULL |
| `balance_before` | decimal(10,2) NULL |
| `balance_after` | decimal(10,2) NULL |
| `examine_time` | bigint NULL |
| `examine_user_id` | bigint NULL |
| `remark` | varchar(256) NOT NULL |
| `refuse_remark` | varchar(256) NULL |
| `freeze_status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_application_screenshot`

| Column | Type |
|--------|------|
| `app_images_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `app_version_id` | bigint NULL |
| `sort_index` | int NOT NULL |
| `screen_type` | tinyint(1) NOT NULL |
| `url` | varchar(128) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_attach`

| Column | Type |
|--------|------|
| `attach_uuid` | varchar(32) NOT NULL |
| `scene` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `extension` | varchar(8) NULL |
| `path` | varchar(128) NOT NULL |
| `url` | varchar(128) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint NOT NULL |
| `character_code` | varchar(32) NULL |
| `table_name` | varchar(32) NULL |
| `column_name` | varchar(32) NULL |
| `key_value` | varchar(32) NULL |

#### ✚ Only in base: `t_application_model`

| Column | Type |
|--------|------|
| `app_model_id` | bigint NOT NULL auto_increment |
| `app_id` | bigint NOT NULL |
| `app_version_id` | bigint NOT NULL DEFAULT 0 |
| `package_name` | varchar(64) NOT NULL |
| `model_code` | varchar(16) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint NOT NULL |

#### ✚ Only in base: `t_device_bind`

| Column | Type |
|--------|------|
| `dev_bind_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `operator_bind_time` | bigint NULL |
| `org_id` | bigint NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `agent_id` | bigint NULL |
| `agent_bind_time` | bigint NULL |
| `developer_id` | bigint NULL |
| `developer_bind_time` | bigint NULL |
| `merchant_id` | bigint NULL |
| `merchant_bind_time` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `cre_type` | tinyint(1) NULL |

#### ✚ Only in base: `t_application_agreement`

| Column | Type |
|--------|------|
| `app_agreement_id` | bigint unsigned NOT NULL auto_increment |
| `app_id` | bigint unsigned NOT NULL |
| `content` | text NOT NULL |
| `cre_time` | bigint unsigned NOT NULL |
| `user_id` | bigint unsigned NOT NULL |

#### ✚ Only in base: `t_device_security`

| Column | Type |
|--------|------|
| `dev_security_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `pub_key` | varchar(2048) NOT NULL |
| `cre_type` | tinyint(1) NOT NULL |
| `ca_cn` | varchar(64) NULL |
| `dev_ca_id` | bigint NULL |
| `key_change_times` | smallint unsigned NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_merchant_application`

| Column | Type |
|--------|------|
| `mrch_app_id` | bigint unsigned NOT NULL auto_increment |
| `mrch_id` | bigint unsigned NOT NULL |
| `app_id` | bigint NOT NULL |
| `purchase_no` | varchar(32) NULL |
| `start_time` | bigint NOT NULL |
| `expire_time` | bigint unsigned NOT NULL DEFAULT 0 |
| `charge_strategy` | tinyint NULL |
| `subscribe_type` | tinyint(1) NULL |
| `cre_time` | bigint unsigned NOT NULL DEFAULT 0 |
| `upd_time` | bigint unsigned NOT NULL DEFAULT 0 |

#### ✚ Only in base: `t_merchant_application_trial`

| Column | Type |
|--------|------|
| `trial_id` | int unsigned NOT NULL auto_increment |
| `mrch_id` | int unsigned NOT NULL |
| `app_id` | int unsigned NOT NULL |
| `app_version_id` | int unsigned NOT NULL |
| `purchase_no` | varchar(32) NULL |
| `start_time` | int NOT NULL |
| `end_time` | int NOT NULL |
| `cre_time` | int NOT NULL |
| `upd_time` | int NOT NULL |

#### ✚ Only in base: `t_order_goods`

| Column | Type |
|--------|------|
| `order_goods_id` | bigint unsigned NOT NULL |
| `order_no` | varchar(32) NOT NULL |
| `goods_id` | varchar(45) NOT NULL |
| `goods_type` | tinyint NOT NULL |
| `goods_desc` | varchar(128) NULL |
| `cre_time` | bigint unsigned NULL |
| `upd_time` | bigint unsigned NULL |

#### ✚ Only in base: `t_service`

| Column | Type |
|--------|------|
| `service_no` | varchar(16) NOT NULL |
| `service_name` | varchar(32) NOT NULL |
| `charge_strategy` | tinyint(1) NOT NULL |
| `subscribe_type` | tinyint(1) NULL |
| `currency` | varchar(8) NOT NULL |
| `amount` | decimal(10,2) NOT NULL |
| `times_limit` | bigint NULL |
| `describe` | varchar(2048) NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_dict`

| Column | Type |
|--------|------|
| `dict_id` | bigint NOT NULL auto_increment |
| `type` | varchar(32) NOT NULL |
| `key_code` | varchar(32) NOT NULL |
| `key_value` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_device_model`

| Column | Type |
|--------|------|
| `dev_model_id` | bigint NOT NULL auto_increment |
| `manufacturer_code` | varchar(16) NOT NULL |
| `manufacturer_name` | varchar(32) NOT NULL |
| `model_code` | varchar(16) NOT NULL |
| `model_type` | tinyint(1) NOT NULL |
| `screen_size` | varchar(16) NOT NULL |
| `resolution` | varchar(16) NOT NULL |
| `pixel_density` | varchar(16) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_account`

| Column | Type |
|--------|------|
| `account_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `currency` | varchar(8) NOT NULL |
| `balance` | decimal(10,2) NOT NULL DEFAULT 0.00 |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

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

## ✚ Database only in base: `oss-front`

### Tables

#### ✚ Only in base: `t_custom`

| Column | Type |
|--------|------|
| `custom_id` | int unsigned NOT NULL auto_increment |
| `custom_uid` | varchar(32) NOT NULL |
| `ca_container` | varchar(64) NOT NULL |
| `parent_cert_cn` | varchar(64) NOT NULL |
| `custom_status` | tinyint unsigned NOT NULL DEFAULT 0 |
| `common_name` | varchar(64) NOT NULL |
| `country_name` | varchar(64) NOT NULL |
| `province_name` | varchar(32) NOT NULL |
| `locality_name` | varchar(64) NOT NULL |
| `org_name` | varchar(64) NOT NULL |
| `org_unit_name` | varchar(64) NOT NULL |
| `email_address` | varchar(128) NOT NULL |
| `cre_time` | bigint unsigned NOT NULL |
| `upd_time` | bigint unsigned NOT NULL |
| `cus_desc` | varchar(128) NOT NULL |
| `biz_type` | varchar(12) NOT NULL |
| `app_id` | varchar(32) NOT NULL |

#### ✚ Only in base: `t_cert_content`

| Column | Type |
|--------|------|
| `cert_id` | int unsigned NOT NULL auto_increment |
| `custom_id` | int unsigned NOT NULL |
| `cert_uid` | varchar(32) NOT NULL |
| `cert_content` | varchar(2048) NOT NULL |
| `cert_key` | varchar(3072) NOT NULL |
| `valid_not_before` | varchar(8) NOT NULL |
| `valid_not_after` | varchar(8) NOT NULL |
| `alias_name` | varchar(80) NOT NULL |
| `cre_time` | bigint unsigned NOT NULL |
| `upd_time` | bigint unsigned NOT NULL |

#### ✚ Only in base: `t_apps`

| Column | Type |
|--------|------|
| `id` | int unsigned NOT NULL auto_increment |
| `app_id` | varchar(32) NOT NULL |
| `app_key` | varchar(1024) NOT NULL |
| `biz_type` | varchar(12) NOT NULL |
| `app_desc` | varchar(45) NULL |
| `status` | tinyint unsigned NOT NULL DEFAULT 1 |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_appid_function`

| Column | Type |
|--------|------|
| `appid_func_id` | int unsigned NOT NULL auto_increment |
| `app_id` | varchar(32) NOT NULL |
| `func_code` | varchar(45) NULL |
| `cre_time` | bigint unsigned NOT NULL |

#### ✚ Only in base: `t_function`

| Column | Type |
|--------|------|
| `func_id` | int unsigned NOT NULL auto_increment |
| `func_code` | varchar(45) NOT NULL |
| `func_name` | varchar(45) NOT NULL |
| `func_url` | varchar(128) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `biz_type` | varchar(12) NOT NULL |

#### ✚ Only in base: `t_signature_flow`

| Column | Type |
|--------|------|
| `signature_flow_id` | int unsigned NOT NULL auto_increment |
| `data_to_sign` | varchar(64) NOT NULL |
| `signature` | varchar(512) NOT NULL |
| `cert_alias_name` | varchar(80) NOT NULL |
| `biz_type` | varchar(12) NOT NULL |
| `app_id` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `apk_package_name` | varchar(128) NULL |
| `apk_version` | varchar(32) NULL |

## ✚ Database only in base: `ota-authorization`

### Tables

#### ✚ Only in base: `t_character_menu_blacklist`

| Column | Type |
|--------|------|
| `menu_blacklist_id` | bigint NOT NULL |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `menu_id` | bigint NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_operator_config`

| Column | Type |
|--------|------|
| `operator_config_id` | bigint NOT NULL auto_increment |
| `operator_id` | bigint NOT NULL |
| `config_key` | varchar(64) NOT NULL |
| `config_value` | varchar(256) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_organization`

| Column | Type |
|--------|------|
| `org_id` | bigint NOT NULL auto_increment |
| `parent_org_id` | bigint NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_code` | varchar(128) NOT NULL |
| `org_name` | varchar(32) NOT NULL |
| `org_type` | tinyint(1) NULL |
| `logo_url` | varchar(128) NULL |
| `master_name` | varchar(32) NULL |
| `master_tel` | varchar(16) NULL |
| `contact_name` | varchar(32) NULL |
| `contact_tel` | varchar(16) NULL |
| `country_code` | char(3) NULL |
| `area_code` | varchar(6) NULL |
| `prov_name` | varchar(32) NULL |
| `city_name` | varchar(32) NULL |
| `area_name` | varchar(32) NULL |
| `address` | varchar(256) NULL |
| `remark` | varchar(1024) NULL |
| `status` | tinyint(1) NULL |
| `cre_time` | bigint NULL |
| `cre_user_id` | bigint NULL |
| `upd_time` | bigint NULL |
| `upd_user_id` | bigint NULL |

#### ✚ Only in base: `t_subject`

| Column | Type |
|--------|------|
| `subject_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `country_code` | char(3) NOT NULL |
| `area_code` | varchar(8) NOT NULL |
| `prov_name` | varchar(32) NULL |
| `city_name` | varchar(32) NULL |
| `area_name` | varchar(32) NULL |
| `address` | varchar(128) NULL |
| `name` | varchar(64) NOT NULL |
| `short_name` | varchar(64) NULL |
| `linkman` | varchar(64) NOT NULL |
| `mobile` | varchar(64) NULL |
| `email` | varchar(128) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_user_id` | bigint NULL |

#### ✚ Only in base: `t_user_role`

| Column | Type |
|--------|------|
| `user_role_uuid` | varchar(32) NOT NULL |
| `user_id` | bigint NOT NULL |
| `role_id` | bigint NOT NULL |
| `cre_time` | bigint NULL |
| `cre_user_id` | bigint NULL |

#### ✚ Only in base: `t_menu`

| Column | Type |
|--------|------|
| `menu_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `parent_menu_id` | bigint NOT NULL |
| `menu_title` | varchar(36) NOT NULL |
| `show_flag` | varchar(32) NULL |
| `href` | varchar(64) NULL |
| `sort` | int NOT NULL DEFAULT 10 |
| `action_id` | varchar(64) NULL |
| `icon` | varchar(32) NULL |
| `status` | tinyint NOT NULL DEFAULT 1 |

#### ✚ Only in base: `t_attach`

| Column | Type |
|--------|------|
| `attach_uuid` | varchar(32) NOT NULL |
| `scene` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `extension` | varchar(8) NOT NULL |
| `path` | varchar(128) NOT NULL |
| `url` | varchar(128) NOT NULL |
| `host` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_character_bind`

| Column | Type |
|--------|------|
| `character_bind_id` | bigint NOT NULL auto_increment |
| `openid` | varchar(64) NULL |
| `bind_type` | varchar(16) NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `bind_character_code` | varchar(16) NOT NULL |
| `bind_key_id` | bigint NOT NULL |
| `extend_1` | varchar(128) NULL |
| `extend_2` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_role_function`

| Column | Type |
|--------|------|
| `role_func_id` | bigint NOT NULL auto_increment |
| `role_id` | bigint NOT NULL |
| `menu_func_code` | varchar(32) NOT NULL |
| `cre_time` | bigint NULL |
| `cre_user_id` | bigint NULL |

#### ✚ Only in base: `t_operator`

| Column | Type |
|--------|------|
| `operator_id` | bigint NOT NULL auto_increment |
| `operator_no` | varchar(16) NULL |
| `operator_type` | varchar(16) NOT NULL |
| `character_code` | varchar(16) NULL |
| `name` | varchar(128) NOT NULL |
| `short_name` | varchar(64) NULL |
| `oid` | varchar(64) NULL |
| `country_code` | char(3) NOT NULL |
| `area_code` | varchar(8) NULL |
| `prov_name` | varchar(32) NULL |
| `city_name` | varchar(64) NULL |
| `area_name` | varchar(64) NULL |
| `address` | varchar(128) NULL |
| `linkman` | varchar(64) NULL |
| `mobile` | varchar(64) NULL |
| `email` | varchar(128) NOT NULL |
| `license` | varchar(128) NULL |
| `status` | tinyint(1) NOT NULL |
| `level_code` | varchar(32) NULL |
| `expire_time` | bigint NULL |
| `remark` | varchar(256) NULL |
| `map_type` | varchar(16) NULL |
| `map_key` | varchar(128) NULL |
| `map_token` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_user_id` | bigint NULL |

#### ✚ Only in base: `t_developer`

| Column | Type |
|--------|------|
| `developer_id` | bigint NOT NULL auto_increment |
| `developer_type` | tinyint(1) NOT NULL |
| `first_name` | varchar(64) NOT NULL |
| `last_name` | varchar(64) NULL |
| `pid_type` | tinyint(1) NULL |
| `pid_no` | varchar(32) NULL |
| `legal_person` | varchar(32) NULL |
| `company_reg_date` | varchar(10) NULL |
| `contact_name` | varchar(64) NOT NULL |
| `mobile` | varchar(18) NULL |
| `email` | varchar(64) NOT NULL |
| `country_code` | char(3) NOT NULL |
| `prov_name` | varchar(32) NULL |
| `city_name` | varchar(32) NULL |
| `area_name` | varchar(32) NULL |
| `area_code` | varchar(6) NULL |
| `address` | varchar(128) NULL |
| `post_code` | varchar(8) NULL |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(256) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_store`

| Column | Type |
|--------|------|
| `store_uuid` | varchar(32) NOT NULL |
| `mrch_id` | bigint NOT NULL |
| `name` | varchar(64) NULL |
| `short_name` | varchar(128) NULL |
| `head_office` | tinyint(1) NULL |
| `area_code` | varchar(6) NULL |
| `prov_name` | varchar(64) NULL |
| `city_name` | varchar(64) NULL |
| `area_name` | varchar(64) NULL |
| `address` | varchar(128) NULL |
| `contact_name` | varchar(64) NULL |
| `mobile` | varchar(16) NULL |
| `email` | varchar(128) NULL |
| `post_code` | varchar(6) NULL |
| `status` | bigint NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `cre_user_id` | bigint NULL |
| `upd_user_id` | bigint NULL |
| `remark` | varchar(2048) NULL |

#### ✚ Only in base: `t_character_bind_outer`

| Column | Type |
|--------|------|
| `character_bind_outer_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NULL |
| `outer_plat_code` | varchar(32) NOT NULL |
| `outer_character_code` | varchar(16) NOT NULL |
| `outer_key_id` | varchar(128) NOT NULL |
| `outer_org_id` | varchar(32) NULL |
| `outer_username` | varchar(128) NULL |
| `outer_password` | varchar(128) NULL |
| `extend_1` | varchar(32) NULL |
| `extend_2` | varchar(64) NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_role`

| Column | Type |
|--------|------|
| `role_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `role_name` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_timezone`

| Column | Type |
|--------|------|
| `timezone_id` | bigint NOT NULL |
| `zone_name` | varchar(8) NULL |
| `utc_time` | char(9) NULL |

#### ✚ Only in base: `t_message`

| Column | Type |
|--------|------|
| `message_uuid` | varchar(32) NOT NULL |
| `sender_character_code` | varchar(16) NOT NULL |
| `sender_key_id` | bigint NOT NULL |
| `sender_user_id` | bigint NOT NULL |
| `message_title` | varchar(128) NOT NULL |
| `message_type` | varchar(32) NOT NULL |
| `message_outlines` | varchar(512) NULL |
| `message_content` | varchar(4096) NOT NULL |
| `topic` | varchar(32) NOT NULL |
| `receiver_character_code` | varchar(16) NOT NULL |
| `receiver_key_id` | bigint NOT NULL |
| `receiver_user_id` | bigint NOT NULL |
| `readed` | tinyint(1) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_area`

| Column | Type |
|--------|------|
| `area_id` | bigint NOT NULL auto_increment |
| `country_code` | char(3) NOT NULL |
| `area_code` | varchar(8) NOT NULL |
| `parent_code` | varchar(8) NULL |
| `area_name` | varchar(64) NOT NULL |
| `area_name_en` | varchar(64) NULL |

#### ✚ Only in base: `t_user_ram`

| Column | Type |
|--------|------|
| `user_ram_id` | bigint NOT NULL auto_increment |
| `user_id` | bigint NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `access_key_id` | varchar(32) NOT NULL |
| `access_key_secret` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `verify_error_times` | int NOT NULL |

#### ✚ Only in base: `t_event`

| Column | Type |
|--------|------|
| `event_uuid` | varchar(32) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `plat_code` | varchar(16) NULL |
| `org_id` | bigint NOT NULL |
| `user_id` | bigint NOT NULL |
| `event_code` | varchar(32) NOT NULL |
| `event_desc` | varchar(256) NULL |
| `cre_time` | binary(12) NOT NULL |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(1024) NULL |

#### ✚ Only in base: `t_password_reset`

| Column | Type |
|--------|------|
| `password_reset_id` | bigint NOT NULL auto_increment |
| `user_id` | bigint NOT NULL |
| `email` | varchar(128) NOT NULL |
| `token` | varchar(64) NOT NULL |
| `expire_time` | bigint NOT NULL |
| `error_times` | int NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_menu_function_url`

| Column | Type |
|--------|------|
| `menu_func_url_id` | bigint NOT NULL auto_increment |
| `menu_id` | bigint NOT NULL |
| `menu_func_code` | varchar(16) NOT NULL |
| `uri` | varchar(64) NOT NULL |

#### ✚ Only in base: `t_character_level_menu`

| Column | Type |
|--------|------|
| `character_level_menu_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `level_id` | bigint NULL |
| `menu_id` | bigint NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_country`

| Column | Type |
|--------|------|
| `country_id` | bigint NOT NULL |
| `country_code2` | char(2) NOT NULL |
| `country_code3` | char(3) NOT NULL |
| `country_name_en` | varchar(64) NOT NULL |
| `country_name_zh` | varchar(32) NOT NULL |
| `default_utc_time` | char(9) NULL |
| `default_lang` | varchar(8) NULL |
| `status` | tinyint(1) NULL |

#### ✚ Only in base: `t_character_manager_function`

| Column | Type |
|--------|------|
| `character_manager_func_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `menu_func_code` | varchar(64) NULL |

#### ✚ Only in base: `t_character_level`

| Column | Type |
|--------|------|
| `character_level_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `level_code` | varchar(32) NOT NULL |
| `level_name` | varchar(45) NULL |
| `level_desc` | varchar(45) NULL |
| `is_default` | tinyint(1) NULL |
| `sort` | int NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_menu_function`

| Column | Type |
|--------|------|
| `menu_func_code` | varchar(64) NOT NULL |
| `menu_id` | bigint NOT NULL |
| `menu_name` | varchar(64) NOT NULL |
| `fun_name` | varchar(64) NOT NULL |
| `index_sort` | int NOT NULL DEFAULT 10 |

#### ✚ Only in base: `t_manufacturers`

| Column | Type |
|--------|------|
| `manufacturer_id` | bigint NOT NULL auto_increment |
| `manufacturer_code` | char(4) NOT NULL |
| `name` | varchar(64) NOT NULL |
| `short_name` | varchar(32) NOT NULL |
| `address` | varchar(256) NULL |
| `linkman` | varchar(64) NULL |
| `mobile` | varchar(16) NULL |
| `email` | varchar(128) NOT NULL |
| `img_path` | varchar(255) NULL |
| `remark` | varchar(210) NULL |
| `status` | tinyint unsigned NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_merchant`

| Column | Type |
|--------|------|
| `mrch_id` | bigint NOT NULL auto_increment |
| `mid` | varchar(64) NULL |
| `name` | varchar(256) NOT NULL |
| `short_name` | varchar(256) NOT NULL |
| `mrch_no` | varchar(32) NULL |
| `oid_` | varchar(64) NULL |
| `business_license` | varchar(64) NULL |
| `contact_name` | varchar(32) NULL |
| `mobile` | varchar(32) NULL |
| `email` | varchar(128) NULL |
| `country_code` | char(3) NOT NULL |
| `area_code` | varchar(6) NULL |
| `prov_name` | varchar(32) NULL |
| `city_name` | varchar(32) NULL |
| `area_name` | varchar(32) NULL |
| `address` | varchar(128) NULL |
| `post_code` | varchar(8) NULL |
| `status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `cre_user_id` | bigint NOT NULL |
| `upd_user_id` | bigint NULL |
| `remark` | varchar(2048) NULL |

#### ✚ Only in base: `t_password_history`

| Column | Type |
|--------|------|
| `pwd_his_id` | bigint NOT NULL auto_increment |
| `user_id` | bigint NOT NULL |
| `password` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `remark` | varchar(128) NULL |

#### ✚ Only in base: `t_user`

| Column | Type |
|--------|------|
| `user_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_code` | varchar(128) NOT NULL |
| `org_id` | bigint NOT NULL |
| `store_uuid` | varchar(32) NULL |
| `user_type` | tinyint NOT NULL DEFAULT 1 |
| `username` | varchar(64) NOT NULL |
| `password` | varchar(64) NOT NULL |
| `password_encrypt_type` | varchar(16) NOT NULL |
| `name` | varchar(32) NULL |
| `email` | varchar(128) NOT NULL |
| `mobile` | varchar(16) NULL |
| `gender` | tinyint(1) NULL |
| `logo_url` | varchar(128) NULL |
| `country_code` | char(3) NULL |
| `address` | varchar(128) NULL |
| `utc_time` | varchar(9) NULL |
| `lang` | varchar(8) NULL |
| `last_login_time` | bigint NULL |
| `cre_user_id` | bigint NOT NULL |
| `manager_flag` | tinyint(1) NOT NULL |
| `password_times` | tinyint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `password_modify_time` | bigint NULL |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(1024) NULL |
| `date_format` | varchar(16) NULL |

#### ✚ Only in base: `t_character_platform`

| Column | Type |
|--------|------|
| `character_plat_id` | bigint NOT NULL auto_increment |
| `character_code` | varchar(16) NULL |
| `key_id` | bigint NULL |
| `plat_code` | varchar(16) NULL |
| `cre_time` | datetime NULL |
| `cre_user_id` | bigint NULL |
| `upd_time` | datetime NULL |
| `upd_user_id` | varchar(45) NULL |

#### ✚ Only in base: `t_user_plat`

| Column | Type |
|--------|------|
| `user_plat_uuid` | varchar(32) NOT NULL |
| `user_id` | bigint NULL |
| `plat_code` | varchar(16) NULL |
| `cre_time` | datetime NULL |
| `cre_user_id` | bigint NULL |

### Functions

#### ✚ Only in base: `getUserMenuList`

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd = ''; SELECT GROUP_CONCAT(a.menu_id) INTO sTempChd FROM t_menu a INNER JOIN t_menu_function b ON a.menu_id=b.menu_id INNER JOIN t_role_function c ON b.menu_func_code = c.menu_func_code INNER JOIN t_user_role d ON c.role_id=d.role_id WHERE d.user_id=userId; WHILE sTempChd IS NOT NULL DO SET sTemp = CONCAT(sTemp,',',sTempChd); SELECT GROUP_CONCAT(parent_menu_id) INTO sTempChd FROM ( SELECT DISTINCT parent_menu_id FROM t_menu WHERE FIND_IN_SET(menu_id, sTempChd)>0 AND parent_menu_id>0 AND status = 1 ) AS x; END WHILE; RETURN sTemp; END
```

#### ✚ Only in base: `getOrgIdList`

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd = ''; SELECT GROUP_CONCAT(a.org_id) INTO sTempChd FROM t_organization a WHERE character_code = characterCode AND a.key_id = keyId AND (IFNULL(parent_org_id, 0) = parentOrgId OR org_id = parentOrgId); WHILE sTempChd IS NOT NULL DO SET sTemp = CONCAT(sTemp,',',sTempChd); SELECT GROUP_CONCAT(x.org_id) INTO sTempChd FROM ( SELECT DISTINCT org_id FROM t_organization WHERE FIND_IN_SET(parent_org_id, sTempChd)>0 ) AS x; END WHILE; RETURN sTemp; END
```

#### ✚ Only in base: `getUserManagerMenuList`

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); SET sTemp = '$'; SET sTempChd = ''; SELECT GROUP_CONCAT(a.menu_id) INTO sTempChd FROM t_menu a INNER JOIN t_menu_function b ON a.menu_id=b.menu_id INNER JOIN t_character_manager_function c ON b.menu_func_code = c.menu_func_code WHERE c.character_code = characterCode; WHILE sTempChd IS NOT NULL DO SET sTemp = CONCAT(sTemp,',',sTempChd); SELECT GROUP_CONCAT(parent_menu_id) INTO sTempChd FROM ( SELECT DISTINCT parent_menu_id FROM t_menu WHERE FIND_IN_SET(menu_id, sTempChd)>0 AND parent_menu_id>0 AND status = 1 ) AS x; END WHILE; RETURN sTemp; END
```

#### ✚ Only in base: `getOrgIdListWithBind`

```sql
BEGIN DECLARE sTemp VARCHAR(2000); DECLARE sTempChd VARCHAR(2000); DECLARE sBindChd VARCHAR(1000); SET sTemp = '$'; SET sTempChd = ''; SET sBindChd = ''; SELECT GROUP_CONCAT(a.org_id) INTO sTempChd FROM t_organization a WHERE character_code = characterCode AND a.key_id = keyId AND (IFNULL(parent_org_id, 0) = parentOrgId OR org_id = parentOrgId); WHILE sTempChd IS NOT NULL DO SET sTemp = CONCAT(sTemp,',',sTempChd); SET sBindChd = sTempChd; SELECT GROUP_CONCAT(x.org_id) INTO sTempChd FROM ( SELECT DISTINCT org_id FROM t_organization WHERE FIND_IN_SET(parent_org_id, sTempChd)>0 ) AS x; SELECT GROUP_CONCAT(b.org_id) INTO sBindChd FROM ( select org.org_id from t_character_bind cb left join t_organization org on org.character_code = cb.bind_character_code and org.key_id = cb.bind_key_id where FIND_IN_SET(cb.org_id, sBindChd)>0 and cb.bind_character_code <> exceptCharacterCode ) AS b; SET sTempChd = CONCAT(IFNULL(sTempChd,''),',',IFNULL(sBindChd,'')); SET sTempChd = IF(sTempChd = ',', NULL, sTempChd); END WHILE; RETURN sTemp; END
```

## ✚ Database only in base: `ota-updater`

### Tables

#### ✚ Only in base: `t_file_progress`

| Column | Type |
|--------|------|
| `progress_id` | bigint NOT NULL auto_increment |
| `user_id` | varchar(45) NOT NULL |
| `name` | varchar(45) NULL |
| `username` | varchar(45) NOT NULL |
| `file_name` | varchar(255) NOT NULL |
| `total_part` | int NOT NULL |
| `current_part` | int NOT NULL DEFAULT 0 |
| `status` | tinyint(1) NOT NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_strategy`

| Column | Type |
|--------|------|
| `strategy_id` | bigint NOT NULL auto_increment |
| `strategy_name` | varchar(255) NULL |
| `user_id` | bigint NOT NULL |
| `username` | varchar(255) NULL |
| `is_dynamic` | tinyint(1) NOT NULL |
| `task_id` | bigint NULL |
| `target_strategy_id` | bigint NULL |
| `comm_freq` | varchar(2) NULL |
| `interval_hour` | int NULL DEFAULT 48 |
| `collection_level` | varchar(255) NULL |
| `download_limit` | bigint NULL |
| `force_limit` | varchar(16) NULL |
| `random_range` | int NULL |
| `first_failed_retry` | int NULL |
| `second_failed_retry` | int NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_device_strategy`

| Column | Type |
|--------|------|
| `device_strategy_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `strategy_id` | bigint NOT NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_user_model`

| Column | Type |
|--------|------|
| `user_model_id` | bigint NOT NULL auto_increment |
| `username` | varchar(255) NOT NULL |
| `models` | text NOT NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `checked_os` | varchar(45) NULL |

#### ✚ Only in base: `t_task_schedule`

| Column | Type |
|--------|------|
| `schedule_id` | bigint NOT NULL auto_increment |
| `task_id` | bigint NOT NULL |
| `percent` | float NOT NULL |
| `start_time` | bigint NULL |
| `end_time` | bigint NOT NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_app_file`

| Column | Type |
|--------|------|
| `app_file_id` | bigint NOT NULL auto_increment |
| `app_file_name` | varchar(255) NOT NULL |
| `file_size` | bigint NULL |
| `file_type` | varchar(2) NULL |
| `user_id` | bigint NULL |
| `username` | varchar(64) NULL |
| `os` | varchar(20) NULL |
| `model_code` | varchar(16) NULL |
| `hardware_id` | varchar(16) NULL |
| `md5` | varchar(255) NULL |
| `test_flag` | tinyint(1) NOT NULL DEFAULT 0 |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_test_user`

| Column | Type |
|--------|------|
| `test_user_id` | bigint NOT NULL auto_increment |
| `username` | varchar(255) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device`

| Column | Type |
|--------|------|
| `dev_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `dev_pn` | varchar(32) NULL |
| `imei` | varchar(15) NULL DEFAULT  |
| `meid` | varchar(15) NULL |
| `prod_date` | varchar(32) NULL |
| `device_model` | varchar(16) NULL DEFAULT  |
| `hardware_id` | varchar(64) NULL |
| `hardware_config` | varchar(255) NULL DEFAULT  |
| `firmware_id` | varchar(12) NULL DEFAULT  |
| `baseband_version` | varchar(64) NULL DEFAULT  |
| `emmc` | varchar(16) NULL |
| `secure_model` | varchar(16) NULL |
| `contactless_model` | varchar(16) NULL |
| `state_secret_model` | varchar(16) NULL |
| `print_model` | varchar(16) NULL |
| `camera_back_model` | varchar(12) NULL DEFAULT  |
| `camera_front_model` | varchar(12) NULL DEFAULT  |
| `scanner_model` | varchar(16) NULL |
| `lcd_manufacturer` | varchar(255) NULL |
| `touch_screen_version` | varchar(128) NULL |
| `firmware_branch_tag` | varchar(255) NULL |
| `payment_crt_subject` | varchar(255) NULL |
| `payment_crt_cn` | varchar(255) NULL |
| `firmware_version` | varchar(64) NULL |
| `boot_version` | varchar(64) NULL |
| `devcfg_version` | varchar(64) NULL |
| `master_version` | varchar(64) NULL |
| `mapp_version` | varchar(64) NULL |
| `power_operation_count` | int NULL DEFAULT 0 |
| `power_on_time` | int NULL |
| `battery_charge_count` | int NULL DEFAULT 0 |
| `battery_charge_time` | int NULL |
| `print_meters` | float NULL DEFAULT 0 |
| `print_cut_count` | int NULL |
| `print_count` | int NULL |
| `last_print_status` | int NULL |
| `swipe_card_count` | int NULL |
| `last_swipe_card_status` | int NULL |
| `plugin_card_count` | int NULL |
| `last_plugin_card_status` | int NULL |
| `contactless_count` | int NULL |
| `last_contactless_status` | int NULL |
| `front_camera_opened_counts` | int NULL |
| `back_camera_opened_counts` | int NULL DEFAULT 0 |
| `scanner_open_count` | int NULL |
| `audio_plugin_count` | int NULL |
| `usb_operation_count` | int NULL DEFAULT 0 |
| `key_pressed_count` | varchar(255) NULL |
| `screen_touched_count` | int NULL DEFAULT 0 |
| `screen_light_time` | int NULL |
| `base_station` | text NULL |
| `hardware_attack_count` | int NULL DEFAULT 0 |
| `hardware_attack_time` | bigint NULL |
| `software_attack_count` | int NULL DEFAULT 0 |
| `software_attack_time` | bigint NULL |
| `storage_available` | bigint NULL DEFAULT 0 |
| `lang` | varchar(32) NULL |
| `time_zone` | varchar(16) NULL DEFAULT  |
| `screen_brightness` | int NULL |
| `screen_sleep_duration` | int NULL |
| `media_volume` | int NULL |
| `bell_volume` | int NULL |
| `wifi` | varchar(32) NULL DEFAULT  |
| `ssid` | varchar(64) NULL DEFAULT  |
| `cellular_network` | varchar(32) NULL |
| `ethernet_network` | varchar(32) NULL |
| `bluetooth` | varchar(32) NULL DEFAULT  |
| `gps` | varchar(32) NULL |
| `cellular_network_traffic` | varchar(255) NULL |
| `upd_epochsecond` | bigint NULL |
| `ip` | varchar(32) NULL |
| `client_version` | varchar(64) NULL |
| `first_connect_time` | bigint NOT NULL |
| `two_way_auth` | tinyint(1) NOT NULL DEFAULT 0 |

#### ✚ Only in base: `t_test_device`

| Column | Type |
|--------|------|
| `test_device_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_tmp`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `batch_no` | varchar(64) NOT NULL |
| `dev_sn` | varchar(45) NULL |
| `task_id` | bigint NULL |
| `device_task_id` | bigint NULL |
| `task_name` | varchar(255) NULL |
| `task_status` | tinyint(1) NULL |
| `strategy_id` | bigint NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `gray_released` | tinyint(1) NULL |

#### ✚ Only in base: `t_task`

| Column | Type |
|--------|------|
| `task_id` | bigint NOT NULL auto_increment |
| `user_id` | bigint NOT NULL |
| `username` | varchar(64) NOT NULL |
| `task_name` | varchar(255) NOT NULL |
| `model_code` | varchar(16) NOT NULL |
| `hardware_id` | varchar(16) NULL |
| `gray_release` | tinyint(1) NOT NULL |
| `terminate_if_failed` | tinyint(1) NOT NULL |
| `silent_update` | tinyint(1) NOT NULL |
| `allow_cancel` | int NOT NULL DEFAULT 2 |
| `device_count` | int NULL DEFAULT 0 |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_model`

| Column | Type |
|--------|------|
| `model_id` | bigint NOT NULL auto_increment |
| `model_code` | varchar(16) NULL |
| `os` | varchar(16) NOT NULL |
| `hardware_id` | varchar(16) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_update_event`

| Column | Type |
|--------|------|
| `event_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `task_id` | bigint NULL |
| `device_task_id` | bigint NOT NULL |
| `app_file_name` | varchar(255) NOT NULL |
| `app_file_md5` | varchar(32) NOT NULL |
| `event_status` | tinyint(1) NOT NULL |
| `start_time` | bigint NULL |
| `end_time` | bigint NULL |
| `cre_time` | bigint NOT NULL |
| `remark` | varchar(255) NULL |

#### ✚ Only in base: `t_device_task`

| Column | Type |
|--------|------|
| `dev_task_id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `task_id` | bigint NOT NULL |
| `task_name` | varchar(255) NOT NULL |
| `start_time` | bigint NULL |
| `end_time` | bigint NULL |
| `task_status` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `gray_released` | tinyint(1) NOT NULL |

#### ✚ Only in base: `t_task_app`

| Column | Type |
|--------|------|
| `task_app_id` | bigint NOT NULL auto_increment |
| `task_id` | bigint NOT NULL |
| `app_file_id` | int NOT NULL |
| `app_file_name` | varchar(255) NOT NULL |
| `app_file_size` | bigint NOT NULL |
| `app_file_md5` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

## ✚ Database only in base: `pre-warning`

### Tables

#### ✚ Only in base: `t_device_exception_data`

| Column | Type |
|--------|------|
| `exception_data_id` | bigint NOT NULL auto_increment |
| `data_key` | varchar(64) NOT NULL |
| `val_exception` | varchar(64) NOT NULL |
| `val_before_exception` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_strategy_notify_user`

| Column | Type |
|--------|------|
| `strategy_notify_account_id` | bigint NOT NULL auto_increment |
| `strategy_id` | bigint NOT NULL |
| `user_id` | bigint NOT NULL |
| `user_name` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_warning_notification`

| Column | Type |
|--------|------|
| `warning_notification_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `strategy_id` | bigint NOT NULL |
| `strategy_name` | varchar(64) NULL |
| `target` | varchar(64) NOT NULL |
| `key_name` | varchar(45) NOT NULL |
| `key_value` | decimal(12,4) NOT NULL |
| `user_id` | bigint NOT NULL |
| `state` | tinyint(1) NOT NULL |
| `content` | varchar(1024) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_strategy_target`

| Column | Type |
|--------|------|
| `strategy_target_id` | bigint NOT NULL auto_increment |
| `strategy_id` | bigint NOT NULL |
| `target` | varchar(32) NOT NULL |
| `tag1` | varchar(64) NULL |
| `tag2` | varchar(64) NULL |
| `tag3` | varchar(64) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_strategy_warning_details`

| Column | Type |
|--------|------|
| `strategy_warning_details_id` | bigint NOT NULL auto_increment |
| `strategy_warning_id` | bigint NOT NULL |
| `warning_model_id` | bigint NOT NULL |
| `warning_model_key_id` | bigint NOT NULL |
| `actual_value` | decimal(12,4) NOT NULL |
| `max_value` | decimal(12,4) NULL |
| `threshold_value` | decimal(12,4) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |
| `notify_time` | bigint NULL |

#### ✚ Only in base: `t_warning_model`

| Column | Type |
|--------|------|
| `warning_model_id` | bigint NOT NULL auto_increment |
| `plat_code` | varchar(16) NOT NULL |
| `model_name` | varchar(64) NOT NULL |
| `warning_frequency` | varchar(16) NOT NULL |
| `notify_frequency` | varchar(16) NULL |
| `calculate_type` | tinyint NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_warning_model_key`

| Column | Type |
|--------|------|
| `warning_model_key_id` | bigint NOT NULL auto_increment |
| `warning_model_id` | bigint NOT NULL |
| `key_name` | varchar(32) NOT NULL |
| `key_max_value` | bigint NULL |
| `key_unit` | varchar(32) NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_strategy_warning`

| Column | Type |
|--------|------|
| `strategy_warning_id` | bigint NOT NULL auto_increment |
| `target` | varchar(32) NOT NULL |
| `tag1` | varchar(64) NULL |
| `tag2` | varchar(64) NULL |
| `tag3` | varchar(64) NULL |
| `strategy_id` | bigint NOT NULL |
| `warning_date` | varchar(8) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_strategy`

| Column | Type |
|--------|------|
| `strategy_id` | bigint NOT NULL auto_increment |
| `strategy_name` | varchar(64) NOT NULL |
| `plat_code` | varchar(16) NOT NULL |
| `character_code` | varchar(16) NOT NULL |
| `key_id` | bigint NOT NULL |
| `org_id` | bigint NOT NULL |
| `warning_model_id` | bigint NOT NULL |
| `strategy_threshold` | bigint NOT NULL |
| `email_notify_flag` | tinyint(1) NOT NULL |
| `notify_email` | varchar(128) NULL |
| `inner_notify_flag` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NULL |

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

## ✚ Database only in base: `tdas-oa`

### Tables

#### ✚ Only in base: `t_business_statistic_4`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_5`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_7`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_7`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_device_ship_info_6`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_device_ship_info_1`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_10`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_business_statistic_3`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_business_statistic_9`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_8`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_device_ship_info_12`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_unknown_record`

| Column | Type |
|--------|------|
| `unknown_record_id` | int NOT NULL auto_increment |
| `unknown_code` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `dev_sn` | varchar(64) NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_9`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_device_batch_file`

| Column | Type |
|--------|------|
| `batch_file_uuid` | varchar(32) NOT NULL |
| `file_name` | varchar(64) NOT NULL |
| `file_md5` | varchar(32) NOT NULL |
| `file_path` | varchar(255) NOT NULL |
| `file_source` | tinyint NOT NULL |
| `sync_status` | tinyint NOT NULL |
| `sync_time` | bigint NULL |
| `remark` | varchar(128) NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_customer`

| Column | Type |
|--------|------|
| `customer_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NOT NULL |
| `customer_name` | varchar(128) NULL |
| `customer_short_name` | varchar(128) NULL |
| `dept_name` | varchar(128) NULL |
| `dept_code` | varchar(64) NULL |
| `email` | varchar(255) NULL |
| `address` | varchar(255) NULL |
| `contact_name` | varchar(255) NULL |
| `is_valid` | varchar(2) NULL |
| `is_oversea` | varchar(2) NULL |
| `is_ota` | tinyint(1) NOT NULL DEFAULT 0 |
| `is_toms` | tinyint(1) NOT NULL DEFAULT 0 |
| `region` | varchar(64) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_12`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_business_statistic_1`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_customer_cn`

| Column | Type |
|--------|------|
| `customer_cn_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_cn` | varchar(128) NULL DEFAULT unknown |
| `code_from_oa` | varchar(128) NULL |
| `code_from_manual` | varchar(128) NULL |
| `is_ota` | tinyint(1) NOT NULL DEFAULT 0 |
| `is_toms` | tinyint(1) NOT NULL DEFAULT 0 |
| `source` | tinyint(1) NOT NULL |
| `cre_time` | bigint NOT NULL |
| `upd_time` | bigint NOT NULL |

#### ✚ Only in base: `t_business_statistic_8`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_11`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_11`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_business_statistic_6`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_3`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_2`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_10`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_device_ship_info_2`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_business_statistic_5`

| Column | Type |
|--------|------|
| `stat_uuid` | varchar(32) NOT NULL |
| `plate_code` | varchar(16) NOT NULL |
| `business_code` | varchar(64) NOT NULL |
| `stat_label_1` | varchar(64) NULL |
| `stat_label_2` | varchar(64) NULL |
| `stat_label_3` | varchar(64) NULL |
| `stat_value` | decimal(10,4) NOT NULL DEFAULT 0.0000 |
| `date_type` | varchar(16) NOT NULL |
| `stat_date` | varchar(32) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_device_ship_info_4`

| Column | Type |
|--------|------|
| `ship_info_id` | bigint NOT NULL auto_increment |
| `customer_code` | varchar(32) NULL |
| `customer_short_name` | varchar(64) NULL |
| `customer_name` | varchar(128) NULL |
| `customer_cn` | varchar(128) NULL |
| `model_code` | varchar(16) NULL |
| `spname` | varchar(64) NULL |
| `sn` | varchar(32) NULL |
| `tu_sn` | varchar(32) NULL |
| `hw_code` | varchar(255) NULL |
| `sw_code` | varchar(255) NULL |
| `fw_code` | varchar(255) NULL |
| `is_ota` | tinyint(1) NULL |
| `is_ota_auto` | tinyint(1) NULL |
| `is_toms` | tinyint(1) NULL |
| `ship_date` | varchar(32) NULL |
| `ship_no` | varchar(128) NULL |
| `oa_add_time` | varchar(32) NULL |
| `rki` | varchar(8) NULL |
| `kms` | varchar(8) NULL |
| `cre_time` | bigint NULL |

#### ✚ Only in base: `t_ship_data_sync_record`

| Column | Type |
|--------|------|
| `sync_record_id` | bigint NOT NULL auto_increment |
| `oa_add_time` | varchar(16) NOT NULL |
| `sync_amount` | int NULL |
| `cre_time` | bigint NOT NULL |

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

## ✚ Database only in base: `toms-mars-geo`

### Tables

#### ✚ Only in base: `t_geo_outrange10`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location9`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange14`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange12`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location13`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_device_lock`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `dev_sn` | varchar(32) NOT NULL |
| `status` | tinyint NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |

#### ✚ Only in base: `t_geo_location1`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange13`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location10`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange4`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange5`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange6`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange1`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange15`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_notify_user`

| Column | Type |
|--------|------|
| `geo_notify_account_id` | bigint NOT NULL auto_increment |
| `geo_schema_cap_id` | bigint NOT NULL |
| `user_id` | bigint NOT NULL |
| `user_name` | varchar(64) NOT NULL |
| `cre_time` | bigint NOT NULL |

#### ✚ Only in base: `t_geo_location12`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location4`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location15`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location8`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location0`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location7`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location2`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location3`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location5`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange11`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange8`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_schema_cap`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `schema_desc` | varchar(64) NOT NULL |
| `max_range` | double(16,6) NOT NULL |
| `mail` | varchar(64) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `lock` | tinyint NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `org_id` | bigint NULL |
| `notify` | tinyint NOT NULL DEFAULT 0 |

#### ✚ Only in base: `t_geo_outrange2`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location14`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange3`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_schema_device`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `dev_sn` | varchar(32) NOT NULL |
| `schema_id` | bigint NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange9`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location6`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange7`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_outrange0`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `upd_time` | bigint NULL |
| `active_lat` | double(16,6) NOT NULL |
| `active_lng` | double(16,6) NOT NULL |
| `range` | double(16,6) NOT NULL |
| `schema_name` | varchar(64) NOT NULL |
| `org_id` | bigint NULL |

#### ✚ Only in base: `t_geo_location11`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `sn` | varchar(32) NOT NULL |
| `lat` | double(16,6) NULL |
| `lng` | double(16,6) NULL |
| `cre_time` | bigint NULL |
| `org_id` | bigint NULL |

## ✚ Database only in base: `toms-mars-pricing`

### Tables

#### ✚ Only in base: `t_biz_def`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `biz_code` | varchar(32) NOT NULL |
| `biz_name` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `up_time` | datetime NOT NULL |
| `biz_desc` | varchar(128) NULL |

#### ✚ Only in base: `t_branch_biz_ref`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `biz_code` | varchar(32) NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NOT NULL |
| `platform` | varchar(64) NULL |

#### ✚ Only in base: `t_branch_biz_res`

| Column | Type |
|--------|------|
| `id` | bigint NOT NULL auto_increment |
| `oid` | varchar(64) NOT NULL |
| `biz_code` | varchar(32) NOT NULL |
| `res_type` | varchar(16) NOT NULL |
| `res_code` | varchar(32) NOT NULL |
| `res_remain` | int NOT NULL |
| `res_effect_time` | datetime NOT NULL |
| `res_expire_time` | datetime NOT NULL |
| `cre_time` | datetime NOT NULL |
| `upd_time` | datetime NOT NULL |
| `order_id` | bigint NOT NULL |
| `version` | bigint NULL DEFAULT 1 |
| `platform` | varchar(64) NULL |

#### ✚ Only in base: `t_device_bind`

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

#### ✚ Only in base: `t_subscribe_terminal`

| Column | Type |
|--------|------|
| `subscribe_uuid` | varchar(32) NOT NULL |
| `bus_type` | varchar(16) NOT NULL |
| `father_object` | varchar(32) NOT NULL |
| `subscriber` | varchar(32) NOT NULL |
| `services` | varchar(2048) NOT NULL |
| `subscribe_time` | bigint NOT NULL |
| `upd_time` | binary(12) NULL |

