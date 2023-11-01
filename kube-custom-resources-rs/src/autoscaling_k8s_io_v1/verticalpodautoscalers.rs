// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kubernetes/autoscaler/autoscaling.k8s.io/v1/verticalpodautoscalers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Specification of the behavior of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "autoscaling.k8s.io", version = "v1", kind = "VerticalPodAutoscaler", plural = "verticalpodautoscalers")]
#[kube(namespaced)]
#[kube(status = "VerticalPodAutoscalerStatus")]
pub struct VerticalPodAutoscalerSpec {
    /// Recommender responsible for generating recommendation for this object. List should be empty (then the default recommender will generate the recommendation) or contain exactly one recommender.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommenders: Option<Vec<VerticalPodAutoscalerRecommenders>>,
    /// Controls how the autoscaler computes recommended resources. The resource policy may be used to set constraints on the recommendations for individual containers. If not specified, the autoscaler computes recommended resources for all containers in the pod, without additional constraints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePolicy")]
    pub resource_policy: Option<VerticalPodAutoscalerResourcePolicy>,
    /// TargetRef points to the controller managing the set of pods for the autoscaler to control - e.g. Deployment, StatefulSet. VerticalPodAutoscaler can be targeted at controller implementing scale subresource (the pod set is retrieved from the controller's ScaleStatus) or some well known controllers (e.g. for DaemonSet the pod set is read from the controller's spec). If VerticalPodAutoscaler cannot use specified target it will report ConfigUnsupported condition. Note that VerticalPodAutoscaler does not require full implementation of scale subresource - it will not use it to modify the replica count. The only thing retrieved is a label selector matching pods grouped by the target resource.
    #[serde(rename = "targetRef")]
    pub target_ref: VerticalPodAutoscalerTargetRef,
    /// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatePolicy")]
    pub update_policy: Option<VerticalPodAutoscalerUpdatePolicy>,
}

/// VerticalPodAutoscalerRecommenderSelector points to a specific Vertical Pod Autoscaler recommender. In the future it might pass parameters to the recommender.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerRecommenders {
    /// Name of the recommender responsible for generating recommendation for this object.
    pub name: String,
}

/// Controls how the autoscaler computes recommended resources. The resource policy may be used to set constraints on the recommendations for individual containers. If not specified, the autoscaler computes recommended resources for all containers in the pod, without additional constraints.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerResourcePolicy {
    /// Per-container resource policies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPolicies")]
    pub container_policies: Option<Vec<VerticalPodAutoscalerResourcePolicyContainerPolicies>>,
}

/// ContainerResourcePolicy controls how autoscaler computes the recommended resources for a specific container.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerResourcePolicyContainerPolicies {
    /// Name of the container or DefaultContainerResourcePolicy, in which case the policy is used by the containers that don't have their own policy specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the type of recommendations that will be computed (and possibly applied) by VPA. If not specified, the default of [ResourceCPU, ResourceMemory] will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlledResources")]
    pub controlled_resources: Option<Vec<String>>,
    /// Specifies which resource values should be controlled. The default is "RequestsAndLimits".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlledValues")]
    pub controlled_values: Option<VerticalPodAutoscalerResourcePolicyContainerPoliciesControlledValues>,
    /// Specifies the maximum amount of resources that will be recommended for the container. The default is no maximum.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAllowed")]
    pub max_allowed: Option<BTreeMap<String, IntOrString>>,
    /// Specifies the minimal amount of resources that will be recommended for the container. The default is no minimum.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minAllowed")]
    pub min_allowed: Option<BTreeMap<String, IntOrString>>,
    /// Whether autoscaler is enabled for the container. The default is "Auto".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<VerticalPodAutoscalerResourcePolicyContainerPoliciesMode>,
}

/// ContainerResourcePolicy controls how autoscaler computes the recommended resources for a specific container.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum VerticalPodAutoscalerResourcePolicyContainerPoliciesControlledValues {
    RequestsAndLimits,
    RequestsOnly,
}

/// ContainerResourcePolicy controls how autoscaler computes the recommended resources for a specific container.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum VerticalPodAutoscalerResourcePolicyContainerPoliciesMode {
    Auto,
    Off,
}

/// TargetRef points to the controller managing the set of pods for the autoscaler to control - e.g. Deployment, StatefulSet. VerticalPodAutoscaler can be targeted at controller implementing scale subresource (the pod set is retrieved from the controller's ScaleStatus) or some well known controllers (e.g. for DaemonSet the pod set is read from the controller's spec). If VerticalPodAutoscaler cannot use specified target it will report ConfigUnsupported condition. Note that VerticalPodAutoscaler does not require full implementation of scale subresource - it will not use it to modify the replica count. The only thing retrieved is a label selector matching pods grouped by the target resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerTargetRef {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}

/// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerUpdatePolicy {
    /// EvictionRequirements is a list of EvictionRequirements that need to evaluate to true in order for a Pod to be evicted. If more than one EvictionRequirement is specified, all of them need to be fulfilled to allow eviction.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionRequirements")]
    pub eviction_requirements: Option<Vec<VerticalPodAutoscalerUpdatePolicyEvictionRequirements>>,
    /// Minimal number of replicas which need to be alive for Updater to attempt pod eviction (pending other checks like PDB). Only positive values are allowed. Overrides global '--min-replicas' flag.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicas")]
    pub min_replicas: Option<i32>,
    /// Controls when autoscaler applies changes to the pod resources. The default is 'Auto'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateMode")]
    pub update_mode: Option<VerticalPodAutoscalerUpdatePolicyUpdateMode>,
}

/// EvictionRequirement defines a single condition which needs to be true in order to evict a Pod
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerUpdatePolicyEvictionRequirements {
    /// EvictionChangeRequirement refers to the relationship between the new target recommendation for a Pod and its current requests, what kind of change is necessary for the Pod to be evicted
    #[serde(rename = "changeRequirement")]
    pub change_requirement: VerticalPodAutoscalerUpdatePolicyEvictionRequirementsChangeRequirement,
    /// Resources is a list of one or more resources that the condition applies to. If more than one resource is given, the EvictionRequirement is fulfilled if at least one resource meets `changeRequirement`.
    pub resource: Vec<String>,
}

/// EvictionRequirement defines a single condition which needs to be true in order to evict a Pod
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum VerticalPodAutoscalerUpdatePolicyEvictionRequirementsChangeRequirement {
    TargetHigherThanRequests,
    TargetLowerThanRequests,
}

/// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum VerticalPodAutoscalerUpdatePolicyUpdateMode {
    Off,
    Initial,
    Recreate,
    Auto,
}

/// Current information about the autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerStatus {
    /// Conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VerticalPodAutoscalerStatusConditions>>,
    /// The most recently computed amount of resources recommended by the autoscaler for the controlled pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<VerticalPodAutoscalerStatusRecommendation>,
}

/// VerticalPodAutoscalerCondition describes the state of a VerticalPodAutoscaler at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// message is a human-readable explanation containing details about the transition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// status is the status of the condition (True, False, Unknown)
    pub status: String,
    /// type describes the current condition
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The most recently computed amount of resources recommended by the autoscaler for the controlled pods.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerStatusRecommendation {
    /// Resources recommended by the autoscaler for each container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecommendations")]
    pub container_recommendations: Option<Vec<VerticalPodAutoscalerStatusRecommendationContainerRecommendations>>,
}

/// RecommendedContainerResources is the recommendation of resources computed by autoscaler for a specific container. Respects the container resource policy if present in the spec. In particular the recommendation is not produced for containers with `ContainerScalingMode` set to 'Off'.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerticalPodAutoscalerStatusRecommendationContainerRecommendations {
    /// Name of the container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Minimum recommended amount of resources. Observes ContainerResourcePolicy. This amount is not guaranteed to be sufficient for the application to operate in a stable way, however running with less resources is likely to have significant impact on performance/availability.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lowerBound")]
    pub lower_bound: Option<BTreeMap<String, IntOrString>>,
    /// Recommended amount of resources. Observes ContainerResourcePolicy.
    pub target: BTreeMap<String, IntOrString>,
    /// The most recent recommended resources target computed by the autoscaler for the controlled pods, based only on actual resource usage, not taking into account the ContainerResourcePolicy. May differ from the Recommendation if the actual resource usage causes the target to violate the ContainerResourcePolicy (lower than MinAllowed or higher that MaxAllowed). Used only as status indication, will not affect actual resource assignment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uncappedTarget")]
    pub uncapped_target: Option<BTreeMap<String, IntOrString>>,
    /// Maximum recommended amount of resources. Observes ContainerResourcePolicy. Any resources allocated beyond this value are likely wasted. This value may be larger than the maximum amount of application is actually capable of consuming.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upperBound")]
    pub upper_bound: Option<BTreeMap<String, IntOrString>>,
}

