// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/percona-server-mysql-operator/ps.percona.com/v1alpha1/perconaservermysqlbackups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ps.percona.com", version = "v1alpha1", kind = "PerconaServerMySQLBackup", plural = "perconaservermysqlbackups")]
#[kube(namespaced)]
#[kube(status = "PerconaServerMySQLBackupStatus")]
#[kube(schema = "disabled")]
pub struct PerconaServerMySQLBackupSpec {
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[serde(rename = "storageName")]
    pub storage_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateDescription")]
    pub state_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<PerconaServerMySQLBackupStatusStorage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<PerconaServerMySQLBackupStatusStorageAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaServerMySQLBackupStatusStorageAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerSecurityContext")]
    pub container_security_context: Option<PerconaServerMySQLBackupStatusStorageContainerSecurityContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<PerconaServerMySQLBackupStatusStorageGcs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSecurityContext")]
    pub pod_security_context: Option<PerconaServerMySQLBackupStatusStoragePodSecurityContext>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityClassName")]
    pub priority_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PerconaServerMySQLBackupStatusStorageResources>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runtimeClassName")]
    pub runtime_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaServerMySQLBackupStatusStorageS3>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulerName")]
    pub scheduler_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<PerconaServerMySQLBackupStatusStorageTolerations>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSpec")]
    pub volume_spec: Option<PerconaServerMySQLBackupStatusStorageVolumeSpec>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAffinity")]
    pub pod_affinity: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAntiAffinity")]
    pub pod_anti_affinity: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinity>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    pub preference: PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageAzure {
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageContainerSecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowPrivilegeEscalation")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<PerconaServerMySQLBackupStatusStorageContainerSecurityContextCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "procMount")]
    pub proc_mount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsGroup")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUser")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seLinuxOptions")]
    pub se_linux_options: Option<PerconaServerMySQLBackupStatusStorageContainerSecurityContextSeLinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seccompProfile")]
    pub seccomp_profile: Option<PerconaServerMySQLBackupStatusStorageContainerSecurityContextSeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "windowsOptions")]
    pub windows_options: Option<PerconaServerMySQLBackupStatusStorageContainerSecurityContextWindowsOptions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageContainerSecurityContextCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageContainerSecurityContextSeLinuxOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageContainerSecurityContextSeccompProfile {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageContainerSecurityContextWindowsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpec")]
    pub gmsa_credential_spec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpecName")]
    pub gmsa_credential_spec_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostProcess")]
    pub host_process: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUserName")]
    pub run_as_user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageGcs {
    pub bucket: String,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStoragePodSecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroup")]
    pub fs_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroupChangePolicy")]
    pub fs_group_change_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsGroup")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUser")]
    pub run_as_user: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seLinuxOptions")]
    pub se_linux_options: Option<PerconaServerMySQLBackupStatusStoragePodSecurityContextSeLinuxOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seccompProfile")]
    pub seccomp_profile: Option<PerconaServerMySQLBackupStatusStoragePodSecurityContextSeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supplementalGroups")]
    pub supplemental_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Vec<PerconaServerMySQLBackupStatusStoragePodSecurityContextSysctls>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "windowsOptions")]
    pub windows_options: Option<PerconaServerMySQLBackupStatusStoragePodSecurityContextWindowsOptions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStoragePodSecurityContextSeLinuxOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStoragePodSecurityContextSeccompProfile {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStoragePodSecurityContextSysctls {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStoragePodSecurityContextWindowsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpec")]
    pub gmsa_credential_spec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpecName")]
    pub gmsa_credential_spec_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostProcess")]
    pub host_process: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUserName")]
    pub run_as_user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PerconaServerMySQLBackupStatusStorageResourcesClaims>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageResourcesClaims {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageS3 {
    pub bucket: String,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageTolerations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emptyDir")]
    pub empty_dir: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecEmptyDir>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostPath")]
    pub host_path: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecHostPath>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaim>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecEmptyDir {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeLimit")]
    pub size_limit: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecHostPath {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaim {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessModes")]
    pub access_modes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSource")]
    pub data_source: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSourceRef")]
    pub data_source_ref: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimDataSourceRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimResources>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeAttributesClassName")]
    pub volume_attributes_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeMode")]
    pub volume_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimDataSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimDataSourceRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMySQLBackupStatusStorageVolumeSpecPersistentVolumeClaimSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
