// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/monitoring.openshift.io/v1/alertrelabelconfigs.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec describes the desired state of this AlertRelabelConfig object.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "monitoring.openshift.io", version = "v1", kind = "AlertRelabelConfig", plural = "alertrelabelconfigs")]
#[kube(namespaced)]
#[kube(status = "AlertRelabelConfigStatus")]
#[kube(schema = "disabled")]
pub struct AlertRelabelConfigSpec {
    /// configs is a list of sequentially evaluated alert relabel configs.
    pub configs: Vec<AlertRelabelConfigConfigs>,
}

/// RelabelConfig allows dynamic rewriting of label sets for alerts. See Prometheus documentation: - https://prometheus.io/docs/prometheus/latest/configuration/configuration/#alert_relabel_configs - https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertRelabelConfigConfigs {
    /// action to perform based on regex matching. Must be one of: 'Replace', 'Keep', 'Drop', 'HashMod', 'LabelMap', 'LabelDrop', or 'LabelKeep'. Default is: 'Replace'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<AlertRelabelConfigConfigsAction>,
    /// modulus to take of the hash of the source label values.  This can be combined with the 'HashMod' action to set 'target_label' to the 'modulus' of a hash of the concatenated 'source_labels'. This is only valid if sourceLabels is not empty and action is not 'LabelKeep' or 'LabelDrop'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// regex against which the extracted value is matched. Default is: '(.*)' regex is required for all actions except 'HashMod'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// replacement value against which a regex replace is performed if the regular expression matches. This is required if the action is 'Replace' or 'LabelMap' and forbidden for actions 'LabelKeep' and 'LabelDrop'. Regex capture groups are available. Default is: '$1'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// separator placed between concatenated source label values. When omitted, Prometheus will use its default value of ';'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// sourceLabels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the 'Replace', 'Keep', and 'Drop' actions. Not allowed for actions 'LabelKeep' and 'LabelDrop'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<Vec<String>>,
    /// targetLabel to which the resulting value is written in a 'Replace' action. It is required for 'Replace' and 'HashMod' actions and forbidden for actions 'LabelKeep' and 'LabelDrop'. Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of label sets for alerts. See Prometheus documentation: - https://prometheus.io/docs/prometheus/latest/configuration/configuration/#alert_relabel_configs - https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertRelabelConfigConfigsAction {
    Replace,
    Keep,
    Drop,
    HashMod,
    LabelMap,
    LabelDrop,
    LabelKeep,
}

/// status describes the current state of this AlertRelabelConfig object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertRelabelConfigStatus {
    /// conditions contains details on the state of the AlertRelabelConfig, may be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AlertRelabelConfigStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertRelabelConfigStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: AlertRelabelConfigStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertRelabelConfigStatusConditionsStatus {
    True,
    False,
    Unknown,
}
