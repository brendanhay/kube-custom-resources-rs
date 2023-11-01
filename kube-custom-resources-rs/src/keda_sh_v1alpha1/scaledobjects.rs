// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kedacore/keda/keda.sh/v1alpha1/scaledobjects.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ScaledObjectSpec is the spec for a ScaledObject resource
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "keda.sh", version = "v1alpha1", kind = "ScaledObject", plural = "scaledobjects")]
#[kube(namespaced)]
#[kube(status = "ScaledObjectStatus")]
pub struct ScaledObjectSpec {
    /// AdvancedConfig specifies advance scaling options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced: Option<ScaledObjectAdvanced>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cooldownPeriod")]
    pub cooldown_period: Option<i32>,
    /// Fallback is the spec for fallback options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ScaledObjectFallback>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleReplicaCount")]
    pub idle_replica_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxReplicaCount")]
    pub max_replica_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicaCount")]
    pub min_replica_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingInterval")]
    pub polling_interval: Option<i32>,
    /// ScaleTarget holds the reference to the scale target Object
    #[serde(rename = "scaleTargetRef")]
    pub scale_target_ref: ScaledObjectScaleTargetRef,
    pub triggers: Vec<ScaledObjectTriggers>,
}

/// AdvancedConfig specifies advance scaling options
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvanced {
    /// HorizontalPodAutoscalerConfig specifies horizontal scale config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "horizontalPodAutoscalerConfig")]
    pub horizontal_pod_autoscaler_config: Option<ScaledObjectAdvancedHorizontalPodAutoscalerConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreToOriginalReplicaCount")]
    pub restore_to_original_replica_count: Option<bool>,
    /// ScalingModifiers describes advanced scaling logic options like formula
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scalingModifiers")]
    pub scaling_modifiers: Option<ScaledObjectAdvancedScalingModifiers>,
}

/// HorizontalPodAutoscalerConfig specifies horizontal scale config
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfig {
    /// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehavior>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehavior {
    /// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleDown")]
    pub scale_down: Option<ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown>,
    /// scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of: * increase no more than 4 pods per 60 seconds * double the number of pods per 60 seconds No stabilization is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleUp")]
    pub scale_up: Option<ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleUp>,
}

/// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDownPolicies>>,
    /// selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selectPolicy")]
    pub select_policy: Option<String>,
    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stabilizationWindowSeconds")]
    pub stabilization_window_seconds: Option<i32>,
}

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDownPolicies {
    /// periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    #[serde(rename = "periodSeconds")]
    pub period_seconds: i32,
    /// type is used to specify the scaling policy.
    #[serde(rename = "type")]
    pub r#type: String,
    /// value contains the amount of change which is permitted by the policy. It must be greater than zero
    pub value: i32,
}

/// scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of: * increase no more than 4 pods per 60 seconds * double the number of pods per 60 seconds No stabilization is used.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleUp {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleUpPolicies>>,
    /// selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selectPolicy")]
    pub select_policy: Option<String>,
    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stabilizationWindowSeconds")]
    pub stabilization_window_seconds: Option<i32>,
}

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleUpPolicies {
    /// periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    #[serde(rename = "periodSeconds")]
    pub period_seconds: i32,
    /// type is used to specify the scaling policy.
    #[serde(rename = "type")]
    pub r#type: String,
    /// value contains the amount of change which is permitted by the policy. It must be greater than zero
    pub value: i32,
}

/// ScalingModifiers describes advanced scaling logic options like formula
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectAdvancedScalingModifiers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activationTarget")]
    pub activation_target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    /// MetricTargetType specifies the type of metric being targeted, and should be either "Value", "AverageValue", or "Utilization"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricType")]
    pub metric_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Fallback is the spec for fallback options
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectFallback {
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: i32,
    pub replicas: i32,
}

/// ScaleTarget holds the reference to the scale target Object
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectScaleTargetRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envSourceContainerName")]
    pub env_source_container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
}

/// ScaleTriggers reference the scaler that will be used
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectTriggers {
    /// AuthenticationRef points to the TriggerAuthentication or ClusterTriggerAuthentication object that is used to authenticate the scaler with the environment
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authenticationRef")]
    pub authentication_ref: Option<ScaledObjectTriggersAuthenticationRef>,
    pub metadata: BTreeMap<String, String>,
    /// MetricTargetType specifies the type of metric being targeted, and should be either "Value", "AverageValue", or "Utilization"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricType")]
    pub metric_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useCachedMetrics")]
    pub use_cached_metrics: Option<bool>,
}

/// AuthenticationRef points to the TriggerAuthentication or ClusterTriggerAuthentication object that is used to authenticate the scaler with the environment
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectTriggersAuthenticationRef {
    /// Kind of the resource being referred to. Defaults to TriggerAuthentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
}

/// ScaledObjectStatus is the status for a ScaledObject resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compositeScalerName")]
    pub composite_scaler_name: Option<String>,
    /// Conditions an array representation to store multiple Conditions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ScaledObjectStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalMetricNames")]
    pub external_metric_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<BTreeMap<String, ScaledObjectStatusHealth>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hpaName")]
    pub hpa_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastActiveTime")]
    pub last_active_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "originalReplicaCount")]
    pub original_replica_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pausedReplicaCount")]
    pub paused_replica_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceMetricNames")]
    pub resource_metric_names: Option<Vec<String>>,
    /// GroupVersionKindResource provides unified structure for schema.GroupVersionKind and Resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleTargetGVKR")]
    pub scale_target_gvkr: Option<ScaledObjectStatusScaleTargetGvkr>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleTargetKind")]
    pub scale_target_kind: Option<String>,
}

/// Condition to store the condition state
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectStatusConditions {
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of condition
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectStatusHealth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfFailures")]
    pub number_of_failures: Option<i32>,
    /// HealthStatusType is an indication of whether the health status is happy or failing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// GroupVersionKindResource provides unified structure for schema.GroupVersionKind and Resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScaledObjectStatusScaleTargetGvkr {
    pub group: String,
    pub kind: String,
    pub resource: String,
    pub version: String,
}

