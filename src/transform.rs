use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Hash, Eq, Default, PartialEq, Debug, Clone, Serialize, Deserialize, Ord, PartialOrd)]
struct FactEntry {
    pub name: String,
    pub value: String,
}

// Accept alert json and return microsoft teams card
pub fn create_card(
    alert_json: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let mut card_body = json!({
        "@type": "MessageCard",
        "@context": "https://schema.org/extensions",
        "summary": "",
        "themeColor": "",
        "title": "",
        "sections": [
            {
                "activityTitle": "",
                "activitySubtitle": "",
                "activityImage": "https://company-30077.frontify.com/api/screen/download/eyJpZCI6MzQwMzI2NCwidmVyc2lvbiI6IjIwMTktMDgtMDIgMTk6Mjg6MDYifQ:frontify:DkJTntON9g0YByA8Q_M4vJX_XxO7je1rn7PJN6RJ_TI/?download&title_as_filename&track",
                "facts": {}
            }
        ]
    });

    let green: &str = "12924F";
    let yellow: &str = "12924F";
    let red: &str = "D7000C";
    let other: &str = "0078D7";

    // Set status of card
    if alert_json["status"].is_string() {
        match alert_json["status"].as_str() {
            Some("OPEN") => {
                card_body["title"] = serde_json::to_value("New Alert Triggered")?;
                card_body["themeColor"] = serde_json::to_value(red)?;
                if alert_json["created"].is_string() {
                    card_body["sections"][0]["activitySubtitle"] = alert_json["created"].clone()
                };
            }
            Some("CLOSED") => {
                card_body["title"] = serde_json::to_value("Alert Closed")?;
                card_body["themeColor"] = serde_json::to_value(green)?;
                if alert_json["updated"].is_string() {
                    card_body["sections"][0]["activitySubtitle"] = alert_json["resolved"].clone()
                };
            }
            Some("INFORMATIONAL") => {
                card_body["title"] = serde_json::to_value("Informational Alert")?;
                card_body["themeColor"] = serde_json::to_value(yellow)?;
                if alert_json["created"].is_string() {
                    card_body["sections"][0]["activitySubtitle"] = alert_json["created"].clone()
                };
            }
            _ => {
                card_body["title"] = serde_json::to_value(alert_json["status"].clone())?;
                card_body["themeColor"] = serde_json::to_value(other)?;
                if alert_json["created"].is_string() {
                    card_body["sections"][0]["activitySubtitle"] = alert_json["created"].clone()
                };
            }
        }
    };

    // Set title and summary based on eventTypeName
    if alert_json["eventTypeName"].is_string() {
        match get_message_string(
            alert_json["eventTypeName"]
                .as_str()
                .expect("Logically, we should not have hit this error"),
        ) {
            Some(string) => {
                card_body["sections"][0]["activityTitle"] = serde_json::to_value(string)?;
                let summary = format!("[{}]: {}", card_body["title"], string);
                card_body["summary"] = serde_json::to_value(summary)?;
            }
            None => {
                card_body["sections"][0]["activityTitle"] = alert_json["eventTypeName"].clone();
                let summary = format!("[{}]: {}", card_body["title"], "Unknown event type");
                card_body["summary"] = serde_json::to_value(summary)?;
            }
        }
    } else {
        // Every event should have an eventTypeName, but return a error in the response if an event does not
        card_body["sections"][0]["activityTitle"] =
            serde_json::to_value("Missing eventTypeName".to_string())?;
        card_body["summary"] = serde_json::to_value("Error, unknown eventTypeName".to_string())?;
    }

    let mut facts_vec: Vec<FactEntry> = Vec::new();

    // Create facts array and push to card
    if alert_json["replicaSetName"].is_string() {
        let fact = FactEntry {
            name: "Replicaset".to_string(),
            value: alert_json["replicaSetName"].to_string(),
        };
        facts_vec.push(fact);
    } else if alert_json["clusterName"].is_string() {
        let fact = FactEntry {
            name: "Cluster Name".to_string(),
            value: alert_json["clusterName"].to_string(),
        };
        facts_vec.push(fact);
    } else if alert_json["groupId"].is_string() {
        let fact = FactEntry {
            name: "Group".to_string(),
            value: alert_json["groupId"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["hostnameAndPort"].is_string() {
        let fact = FactEntry {
            name: "Server".to_string(),
            value: alert_json["hostnameAndPort"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["sourceTypeName"].is_string() {
        let fact = FactEntry {
            name: "Source Type".to_string(),
            value: alert_json["sourceTypeName"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["metricName"].is_string() {
        let fact = FactEntry {
            name: "Metric Name".to_string(),
            value: alert_json["metricName"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["currentValue"]["number"].is_string() {
        let fact = FactEntry {
            name: "Metric Value".to_string(),
            value: alert_json["currentValue"]["number"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["currentValue"]["units"].is_string() {
        let fact = FactEntry {
            name: "Metric Unit".to_string(),
            value: alert_json["currentValue"]["units"].to_string(),
        };
        facts_vec.push(fact);
    }
    if alert_json["typeName"].is_string() {
        let fact = FactEntry {
            name: "Type".to_string(),
            value: alert_json["typeName"].to_string(),
        };
        facts_vec.push(fact);
    }

    let facts = json!(facts_vec);
    card_body["sections"][0]["facts"] = facts;
    Ok(card_body)
}

pub fn get_message_string(alert_type: &str) -> Option<&str> {
    match alert_type {
        "AUTOMATION_AGENT_DOWN" => Some("Automation is down"),
        "AUTOMATION_AGENT_UP" => Some("Automation is up"),
        "BACKUP_AGENT_CONF_CALL_FAILURE" => Some("Backup has too many conf call failures"),
        "BACKUP_AGENT_DOWN" => Some("Backup is down"),
        "BACKUP_AGENT_UP" => Some("Backup is up"),
        "BACKUP_AGENT_VERSION_BEHIND" => Some("Backup does not have the latest version"),
        "BACKUP_AGENT_VERSION_CURRENT" => Some("Backup has the latest version"),
        "BLOCKSTORE_JOB_TOO_MANY_RETRIES" => Some("Blockstore jobs have reached a high number of retries"),
        "MONITORING_AGENT_DOWN" => Some("Monitoring is down"),
        "MONITORING_AGENT_UP" => Some("Monitoring is up"),
        "MONITORING_AGENT_VERSION_BEHIND" => Some("Monitoring does not have the latest version"),
        "MONITORING_AGENT_VERSION_CURRENT" => Some("Monitoring has the latest version"),
        "AUTOMATION_CONFIG_PUBLISHED_AUDIT" => Some("Deployment configuration published"),
        "BAD_CLUSTERSHOTS" => Some("Backup has possibly inconsistent cluster snapshots"),
        "CLUSTER_BLACKLIST_UPDATED_AUDIT" => Some("Excluded namespaces were modified for cluster"),
        "CLUSTER_CHECKKPOINT_UPDATED_AUDIT" => Some("Checkpoint interval updated for cluster"),
        "CLUSTER_CREDENTIAL_UPDATED_AUDIT" => Some("Backup authentication credentials updated for cluster"),
        "CLUSTER_SNAPSHOT_SCHEDULE_UPDATED_AUDIT" => Some("Snapshot schedule updated for cluster"),
        "CLUSTER_STATE_CHANGED_AUDIT" => Some("Cluster backup state is now"),
        "CLUSTER_STORAGE_ENGINE_UPDATED_AUDIT" => Some("Cluster storage engine has been updated"),
        "CLUSTERSHOT_DELETED_AUDIT" => Some("Cluster snapshot has been deleted"),
        "CLUSTERSHOT_EXPIRY_UPDATED_AUDIT" => Some("Clustershot expiry has been updated"),
        "CONSISTENT_BACKUP_CONFIGURATION" => Some("Backup configuration is consistent"),
        "GOOD_CLUSTERSHOT" => Some("Backup has a good clustershot"),
        "INCONSISTENT_BACKUP_CONFIGURATION" => Some("Inconsistent backup configuration has been detected"),
        "INITIAL_SYNC_FINISHED_AUDIT" => Some("Backup initial sync finished"),
        "INITIAL_SYNC_STARTED_AUDIT" => Some("Backup initial sync started"),
        "OPLOG_BEHIND" => Some("Backup oplog is behind"),
        "OPLOG_CURRENT" => Some("Backup oplog is current"),
        "RESTORE_REQUESTED_AUDIT" => Some("A restore has been requested"),
        "RESYNC_PERFORMED" => Some("Backup has been resynced"),
        "RESYNC_REQUIRED" => Some("Backup requires a resync"),
        "RS_BLACKLIST_UPDATED_AUDIT" => Some("Excluded namespaces were modified for replica set"),
        "RS_CREDENTIAL_UPDATED_AUDIT" => Some("Backup authentication credentials updated for replica set"),
        "RS_ROTATE_MASTER_KEY_AUDIT" => Some("A master key rotation has been requested for a replica set"),
        "RS_SNAPSHOT_SCHEDULE_UPDATED_AUDIT" => Some("Snapshot schedule updated for replica set"),
        "RS_STATE_CHANGED_AUDIT" => Some("Replica set backup state is now"),
        "RS_STORAGE_ENGINE_UPDATED_AUDIT" => Some("Replica set storage engine has been updated"),
        "SNAPSHOT_DELETED_AUDIT" => Some("Snapshot has been deleted"),
        "SNAPSHOT_EXPIRY_UPDATED_AUDIT" => Some("Snapshot expiry has been updated"),
        "SYNC_PENDING_AUDIT" => Some("Backup sync is pending"),
        "SYNC_REQUIRED_AUDIT" => Some("Backup sync has been initiated"),
        "BI_CONNECTOR_DOWN" => Some("BI Connector is down"),
        "BI_CONNECTOR_UP" => Some("BI Connector is up Project"),
        "CLUSTER_MONGOS_IS_MISSING" => Some("Cluster is missing an active mongos"),
        "CLUSTER_MONGOS_IS_PRESENT" => Some("Cluster has an active mongos"),
        "SHARD_ADDED" => Some("Shard added"),
        "SHARD_REMOVED" => Some("Shard removed"),
        "DATA_EXPLORER" => Some("User performed a Data Explorer read-only operation"),
        "DATA_EXPLORER_CRUD" => Some("User performed a Data Explorer CRUD operation"),
        "ADD_HOST_AUDIT" => Some("Host added"),
        "ADD_HOST_TO_REPLICA_SET_AUDIT" => Some("Host added to replica set"),
        "ATTEMPT_KILLOP_AUDIT" => Some("Attempted to kill operation"),
        "ATTEMPT_KILLSESSION_AUDIT" => Some("Attempted to kill session"),
        "DB_PROFILER_DISABLE_AUDIT" => Some("Database profiling disabled"),
        "DB_PROFILER_ENABLE_AUDIT" => Some("Database profiling enabled"),
        "DELETE_HOST_AUDIT" => Some("Host removed"),
        "DISABLE_HOST_AUDIT" => Some("Host disabled"),
        "HIDE_AND_DISABLE_HOST_AUDIT" => Some("Host disabled and hidden"),
        "HIDE_HOST_AUDIT" => Some("Host hidden"),
        "HOST_DOWN" => Some("Host is down"),
        "HOST_DOWNGRADED" => Some("Host has been downgraded"),
        "HOST_IP_CHANGED_AUDIT" => Some("Host IP address changed"),
        "HOST_NOW_PRIMARY" => Some("Host is now primary"),
        "HOST_NOW_SECONDARY" => Some("Host is now secondary"),
        "HOST_NOW_STANDALONE" => Some("Host is now a standalone"),
        "HOST_RECOVERED" => Some("Host has recovered"),
        "HOST_RECOVERING" => Some("Host is recovering"),
        "HOST_RESTARTED" => Some("Host has restarted"),
        "HOST_ROLLBACK" => Some("Host experienced a rollback"),
        "HOST_SSL_CERTIFICATE_CURRENT" => Some("Host’s SSL certificate is current"),
        "HOST_SSL_CERTIFICATE_STALE" => Some("Host’s SSL certificate will expire within 30 days"),
        "HOST_UP" => Some("Host is up"),
        "HOST_UPGRADED" => Some("Host has been upgraded"),
        "INSIDE_METRIC_THRESHOLD" => Some("Inside metric threshold"),
        "NEW_HOST" => Some("Host is new"),
        "OUTSIDE_METRIC_THRESHOLD" => Some("Outside metric threshold"),
        "PAUSE_HOST_AUDIT" => Some("Host paused"),
        "REMOVE_HOST_FROM_REPLICA_SET_AUDIT" => Some("Host removed from replica set"),
        "RESUME_HOST_AUDIT" => Some("Host resumed"),
        "UNDELETE_HOST_AUDIT" => Some("Host undeleted"),
        "VERSION_BEHIND" => Some("Host does not have the latest version"),
        "VERSION_CHANGED" => Some("Host version changed"),
        "VERSION_CURRENT" => Some("Host has the latest version Project"),
        "ALL_ORG_USERS_HAVE_MFA" => Some("Organization users have two-factor authentication enabled"),
        "ORG_API_KEY_ADDED" => Some("API key has been added"),
        "ORG_API_KEY_DELETED" => Some("API key has been deleted"),
        "ORG_EMPLOYEE_ACCESS_RESTRICTED" => Some("MongoDB Production Support Employees restricted from accessing Atlas backend infrastructure for any Atlas cluster in this organization (You may grant a 24 hour bypass to the access restriction at the Atlas cluster level),"),
        "ORG_EMPLOYEE_ACCESS_UNRESTRICTED" => Some("MongoDB Production Support Employees unrestricted from accessing Atlas backend infrastructure for any Atlas cluster in this organization"),
        "ORG_PUBLIC_API_WHITELIST_NOT_REQUIRED" => Some("IP Whitelist for Public API Not Required"),
        "ORG_PUBLIC_API_WHITELIST_REQUIRED" => Some("Require IP Whitelist for Public API Enabled"),
        "ORG_RENAMED" => Some("Organization has been renamed"),
        "ORG_TWO_FACTOR_AUTH_OPTIONAL" => Some("Two-factor Authentication Optional"),
        "ORG_TWO_FACTOR_AUTH_REQUIRED" => Some("Two-factor Authentication Required"),
        "ORG_USERS_WITHOUT_MFA" => Some("Organization users do not have two-factor authentication enabled"),
        "ALL_USERS_HAVE_MULTIFACTOR_AUTH" => Some("Users have two-factor authentication enabled"),
        "USERS_WITHOUT_MULTIFACTOR_AUTH" => Some("Users do not have two-factor authentication enabled"),
        "CONFIGURATION_CHANGED" => Some("Replica set has an updated configuration"),
        "ENOUGH_HEALTHY_MEMBERS" => Some("Replica set has enough healthy members"),
        "MEMBER_ADDED" => Some("Replica set member added"),
        "MEMBER_REMOVED" => Some("Replica set member removed"),
        "MULTIPLE_PRIMARIES" => Some("Replica set elected multiple primaries"),
        "NO_PRIMARY" => Some("Replica set has no primary"),
        "ONE_PRIMARY" => Some("Replica set elected one primary"),
        "PRIMARY_ELECTED" => Some("Replica set elected a new primary"),
        "TOO_FEW_HEALTHY_MEMBERS" => Some("Replica set has too few healthy members"),
        "TOO_MANY_ELECTIONS" => Some("Replica set has too many election events"),
        "TOO_MANY_UNHEALTHY_MEMBERS" => Some("Replica set has too many unhealthy members"),
        "TEAM_ADDED_TO_GROUP" => Some("Team added to project"),
        "TEAM_CREATED" => Some("Team created"),
        "TEAM_DELETED" => Some("Team deleted"),
        "TEAM_NAME_CHANGED" => Some("Team name changed"),
        "TEAM_REMOVED_FROM_GROUP" => Some("Team removed from project"),
        "TEAM_ROLES_MODIFIED" => Some("Team roles modified in project"),
        "TEAM_UPDATED" => Some("Team updated"),
        "USER_ADDED_TO_TEAM" => Some("User added to team"),
        "INVITED_TO_GROUP" => Some("User was invited to project"),
        "INVITED_TO_ORG" => Some("User was invited to organization"),
        "JOIN_GROUP_REQUEST_APPROVED_AUDIT" => Some("Request to join project was approved"),
        "JOIN_GROUP_REQUEST_DENIED_AUDIT" => Some("Request to join project was denied"),
        "JOINED_GROUP" => Some("User joined the project"),
        "JOINED_ORG" => Some("User joined the organization"),
        "JOINED_TEAM" => Some("User joined the team"),
        "REMOVED_FROM_GROUP" => Some("User left the project"),
        "REMOVED_FROM_ORG" => Some("User left the organization"),
        "REMOVED_FROM_TEAM" => Some("User left the team"),
        "REQUESTED_TO_JOIN_GROUP" => Some("User requested to join project"),
        "USER_ROLES_CHANGED_AUDIT" => Some("User had their role changed"),
        _ => None
    }
}
