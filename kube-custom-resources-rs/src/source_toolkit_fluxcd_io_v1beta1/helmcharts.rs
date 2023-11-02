// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta1/helmcharts.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// HelmChartSpec defines the desired state of a Helm chart.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta1", kind = "HelmChart", plural = "helmcharts")]
#[kube(namespaced)]
#[kube(status = "HelmChartStatus")]
#[kube(schema = "disabled")]
pub struct HelmChartSpec {
    /// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<HelmChartAccessFrom>,
    /// The name or path the Helm chart is available at in the SourceRef.
    pub chart: String,
    /// The interval at which to check the Source for updates.
    pub interval: String,
    /// Determines what enables the creation of a new artifact. Valid values are ('ChartVersion', 'Revision'). See the documentation of the values for an explanation on their behavior. Defaults to ChartVersion when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileStrategy")]
    pub reconcile_strategy: Option<HelmChartReconcileStrategy>,
    /// The reference to the Source the chart is available at.
    #[serde(rename = "sourceRef")]
    pub source_ref: HelmChartSourceRef,
    /// This flag tells the controller to suspend the reconciliation of this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Alternative values file to use as the default chart values, expected to be a relative path in the SourceRef. Deprecated in favor of ValuesFiles, for backwards compatibility the file defined here is merged before the ValuesFiles items. Ignored when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFile")]
    pub values_file: Option<String>,
    /// Alternative list of values files to use as the chart values (values.yaml is not included by default), expected to be a relative path in the SourceRef. Values files are merged in the order of this list with the last file overriding the first. Ignored when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFiles")]
    pub values_files: Option<Vec<String>>,
    /// The chart version semver expression, ignored for charts from GitRepository and Bucket sources. Defaults to latest when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies. Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<HelmChartAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies. An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// HelmChartSpec defines the desired state of a Helm chart.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HelmChartReconcileStrategy {
    ChartVersion,
    Revision,
}

/// The reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartSourceRef {
    /// APIVersion of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent, valid values are ('HelmRepository', 'GitRepository', 'Bucket').
    pub kind: HelmChartSourceRefKind,
    /// Name of the referent.
    pub name: String,
}

/// The reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HelmChartSourceRefKind {
    HelmRepository,
    GitRepository,
    Bucket,
}

/// HelmChartStatus defines the observed state of the HelmChart.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartStatus {
    /// Artifact represents the output of the last successful chart sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmChartStatusArtifact>,
    /// Conditions holds the conditions for the HelmChart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<HelmChartStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the download link for the last chart pulled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful chart sync.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartStatusArtifact {
    /// Checksum is the SHA256 checksum of the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of this artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// Path is the relative file path of this artifact.
    pub path: String,
    /// Revision is a human readable identifier traceable in the origin source system. It can be a Git commit SHA, Git tag, a Helm index timestamp, a Helm chart version, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// URL is the HTTP address of this artifact.
    pub url: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelmChartStatusConditions {
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
    pub status: HelmChartStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HelmChartStatusConditionsStatus {
    True,
    False,
    Unknown,
}

