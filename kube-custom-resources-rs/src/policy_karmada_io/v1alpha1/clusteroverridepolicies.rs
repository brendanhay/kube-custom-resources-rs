// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/policy.karmada.io/v1alpha1/clusteroverridepolicies.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Spec represents the desired behavior of ClusterOverridePolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "policy.karmada.io", version = "v1alpha1", kind = "ClusterOverridePolicy", plural = "clusteroverridepolicies")]
#[kube(schema = "disabled")]
pub struct ClusterOverridePolicySpec {
    /// OverrideRules defines a collection of override rules on target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overrideRules")]
    pub override_rules: Option<Vec<ClusterOverridePolicyOverrideRules>>,
    /// Overriders represents the override rules that would apply on resources 
    ///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriders: Option<ClusterOverridePolicyOverriders>,
    /// ResourceSelectors restricts resource types that this override policy applies to. nil means matching all resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceSelectors")]
    pub resource_selectors: Option<Vec<ClusterOverridePolicyResourceSelectors>>,
    /// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters. 
    ///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetCluster")]
    pub target_cluster: Option<ClusterOverridePolicyTargetCluster>,
}

/// RuleWithCluster defines the override rules on clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRules {
    /// Overriders represents the override rules that would apply on resources
    pub overriders: ClusterOverridePolicyOverrideRulesOverriders,
    /// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetCluster")]
    pub target_cluster: Option<ClusterOverridePolicyOverrideRulesTargetCluster>,
}

/// Overriders represents the override rules that would apply on resources
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverriders {
    /// AnnotationsOverrider represents the rules dedicated to handling workload annotations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationsOverrider")]
    pub annotations_overrider: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersAnnotationsOverrider>>,
    /// ArgsOverrider represents the rules dedicated to handling container args
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "argsOverrider")]
    pub args_overrider: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersArgsOverrider>>,
    /// CommandOverrider represents the rules dedicated to handling container command
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commandOverrider")]
    pub command_overrider: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersCommandOverrider>>,
    /// ImageOverrider represents the rules dedicated to handling image overrides.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageOverrider")]
    pub image_overrider: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersImageOverrider>>,
    /// LabelsOverrider represents the rules dedicated to handling workload labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsOverrider")]
    pub labels_overrider: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersLabelsOverrider>>,
    /// Plaintext represents override rules defined with plaintext overriders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersPlaintext>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersAnnotationsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersAnnotationsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersAnnotationsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersArgsOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersArgsOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersArgsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersCommandOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersCommandOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersCommandOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersImageOverrider {
    /// Component is part of image name. Basically we presume an image can be made of '[registry/]repository[:tag]'. The registry could be: - registry.k8s.io - fictional.registry.example:10443 The repository could be: - kube-apiserver - fictional/nginx The tag cloud be: - latest - v1.19.1 - @sha256:dbcc1c35ac38df41fd2f5e4130b32ffdb93ebae8b3dbe638c23575912276fc9c
    pub component: ClusterOverridePolicyOverrideRulesOverridersImageOverriderComponent,
    /// Operator represents the operator which will apply on the image.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersImageOverriderOperator,
    /// Predicate filters images before applying the rule. 
    ///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
    ///  If not nil, only images matches the filters will be processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<ClusterOverridePolicyOverrideRulesOverridersImageOverriderPredicate>,
    /// Value to be applied to image. Must not be empty when operator is 'add' or 'replace'. Defaults to empty and ignored when operator is 'remove'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersImageOverriderComponent {
    Registry,
    Repository,
    Tag,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersImageOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// Predicate filters images before applying the rule. 
///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
///  If not nil, only images matches the filters will be processed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersImageOverriderPredicate {
    /// Path indicates the path of target field
    pub path: String,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersLabelsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersLabelsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersLabelsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersPlaintext {
    /// Operator indicates the operation on target field. Available operators are: add, replace and remove.
    pub operator: ClusterOverridePolicyOverrideRulesOverridersPlaintextOperator,
    /// Path indicates the path of target field
    pub path: String,
    /// Value to be applied to target field. Must be empty when operator is Remove.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, serde_json::Value>>,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersPlaintextOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetCluster {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<ClusterOverridePolicyOverrideRulesTargetClusterFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ClusterOverridePolicyOverrideRulesTargetClusterLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClusterFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyOverrideRulesTargetClusterFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClusterFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClusterLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyOverrideRulesTargetClusterLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClusterLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Overriders represents the override rules that would apply on resources 
///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverriders {
    /// AnnotationsOverrider represents the rules dedicated to handling workload annotations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationsOverrider")]
    pub annotations_overrider: Option<Vec<ClusterOverridePolicyOverridersAnnotationsOverrider>>,
    /// ArgsOverrider represents the rules dedicated to handling container args
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "argsOverrider")]
    pub args_overrider: Option<Vec<ClusterOverridePolicyOverridersArgsOverrider>>,
    /// CommandOverrider represents the rules dedicated to handling container command
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commandOverrider")]
    pub command_overrider: Option<Vec<ClusterOverridePolicyOverridersCommandOverrider>>,
    /// ImageOverrider represents the rules dedicated to handling image overrides.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageOverrider")]
    pub image_overrider: Option<Vec<ClusterOverridePolicyOverridersImageOverrider>>,
    /// LabelsOverrider represents the rules dedicated to handling workload labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsOverrider")]
    pub labels_overrider: Option<Vec<ClusterOverridePolicyOverridersLabelsOverrider>>,
    /// Plaintext represents override rules defined with plaintext overriders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<Vec<ClusterOverridePolicyOverridersPlaintext>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersAnnotationsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: ClusterOverridePolicyOverridersAnnotationsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersAnnotationsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersArgsOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: ClusterOverridePolicyOverridersArgsOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersArgsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersCommandOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: ClusterOverridePolicyOverridersCommandOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersCommandOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersImageOverrider {
    /// Component is part of image name. Basically we presume an image can be made of '[registry/]repository[:tag]'. The registry could be: - registry.k8s.io - fictional.registry.example:10443 The repository could be: - kube-apiserver - fictional/nginx The tag cloud be: - latest - v1.19.1 - @sha256:dbcc1c35ac38df41fd2f5e4130b32ffdb93ebae8b3dbe638c23575912276fc9c
    pub component: ClusterOverridePolicyOverridersImageOverriderComponent,
    /// Operator represents the operator which will apply on the image.
    pub operator: ClusterOverridePolicyOverridersImageOverriderOperator,
    /// Predicate filters images before applying the rule. 
    ///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
    ///  If not nil, only images matches the filters will be processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<ClusterOverridePolicyOverridersImageOverriderPredicate>,
    /// Value to be applied to image. Must not be empty when operator is 'add' or 'replace'. Defaults to empty and ignored when operator is 'remove'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersImageOverriderComponent {
    Registry,
    Repository,
    Tag,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersImageOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// Predicate filters images before applying the rule. 
///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
///  If not nil, only images matches the filters will be processed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersImageOverriderPredicate {
    /// Path indicates the path of target field
    pub path: String,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersLabelsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: ClusterOverridePolicyOverridersLabelsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersLabelsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverridersPlaintext {
    /// Operator indicates the operation on target field. Available operators are: add, replace and remove.
    pub operator: ClusterOverridePolicyOverridersPlaintextOperator,
    /// Path indicates the path of target field
    pub path: String,
    /// Value to be applied to target field. Must be empty when operator is Remove.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, serde_json::Value>>,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverridersPlaintextOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// ResourceSelector the resources will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyResourceSelectors {
    /// APIVersion represents the API version of the target resources.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind represents the Kind of the target resources.
    pub kind: String,
    /// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ClusterOverridePolicyResourceSelectorsLabelSelector>,
    /// Name of the target resource. Default is empty, which means selecting all resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the target resource. Default is empty, which means inherit from the parent object scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyResourceSelectorsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyResourceSelectorsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyResourceSelectorsLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters. 
///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyTargetCluster {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<ClusterOverridePolicyTargetClusterFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ClusterOverridePolicyTargetClusterLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyTargetClusterFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyTargetClusterFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyTargetClusterFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyTargetClusterLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyTargetClusterLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyTargetClusterLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
