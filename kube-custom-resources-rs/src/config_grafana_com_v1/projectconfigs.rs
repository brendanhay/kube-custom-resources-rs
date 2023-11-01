// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/grafana/loki/config.grafana.com/v1/projectconfigs.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Controller contains global configuration options for controllers registered within this manager.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigController {
    /// CacheSyncTimeout refers to the time limit set to wait for syncing caches. Defaults to 2 minutes if not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheSyncTimeout")]
    pub cache_sync_timeout: Option<i64>,
    /// GroupKindConcurrency is a map from a Kind to the number of concurrent reconciliation allowed for that controller. 
    ///  When a controller is registered within this manager using the builder utilities, users have to specify the type the controller reconciles in the For(...) call. If the object's kind passed matches one of the keys in this map, the concurrency for that controller is set to the number specified. 
    ///  The key is expected to be consistent in form with GroupKind.String(), e.g. ReplicaSet in apps group (regardless of version) would be `ReplicaSet.apps`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupKindConcurrency")]
    pub group_kind_concurrency: Option<BTreeMap<String, i64>>,
}

/// FeatureFlags is a set of operator feature flags.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigFeatureFlags {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableAlertingRuleWebhook")]
    pub enable_alerting_rule_webhook: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableCertSigningService")]
    pub enable_cert_signing_service: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableGrafanaLabsStats")]
    pub enable_grafana_labs_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableLokiStackAlerts")]
    pub enable_loki_stack_alerts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableLokiStackGateway")]
    pub enable_loki_stack_gateway: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableLokiStackGatewayRoute")]
    pub enable_loki_stack_gateway_route: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRecordingRuleWebhook")]
    pub enable_recording_rule_webhook: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRulerConfigWebhook")]
    pub enable_ruler_config_webhook: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableServiceMonitors")]
    pub enable_service_monitors: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableTlsGrpcServices")]
    pub enable_tls_grpc_services: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableTlsHttpServices")]
    pub enable_tls_http_services: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableTlsServiceMonitorConfig")]
    pub enable_tls_service_monitor_config: Option<bool>,
}

/// Health contains the controller health configuration
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigHealth {
    /// HealthProbeBindAddress is the TCP address that the controller should bind to for serving health probes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthProbeBindAddress")]
    pub health_probe_bind_address: Option<String>,
    /// LivenessEndpointName, defaults to "healthz"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "livenessEndpointName")]
    pub liveness_endpoint_name: Option<String>,
    /// ReadinessEndpointName, defaults to "readyz"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readinessEndpointName")]
    pub readiness_endpoint_name: Option<String>,
}

/// LeaderElection is the LeaderElection config to be used when configuring the manager.Manager leader election
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigLeaderElection {
    /// leaderElect enables a leader election client to gain leadership before executing the main loop. Enable this when running replicated components for high availability.
    #[serde(rename = "leaderElect")]
    pub leader_elect: bool,
    /// leaseDuration is the duration that non-leader candidates will wait after observing a leadership renewal until attempting to acquire leadership of a led but unrenewed leader slot. This is effectively the maximum duration that a leader can be stopped before it is replaced by another candidate. This is only applicable if leader election is enabled.
    #[serde(rename = "leaseDuration")]
    pub lease_duration: String,
    /// renewDeadline is the interval between attempts by the acting master to renew a leadership slot before it stops leading. This must be less than or equal to the lease duration. This is only applicable if leader election is enabled.
    #[serde(rename = "renewDeadline")]
    pub renew_deadline: String,
    /// resourceLock indicates the resource object type that will be used to lock during leader election cycles.
    #[serde(rename = "resourceLock")]
    pub resource_lock: String,
    /// resourceName indicates the name of resource object that will be used to lock during leader election cycles.
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// resourceName indicates the namespace of resource object that will be used to lock during leader election cycles.
    #[serde(rename = "resourceNamespace")]
    pub resource_namespace: String,
    /// retryPeriod is the duration the clients should wait between attempting acquisition and renewal of a leadership. This is only applicable if leader election is enabled.
    #[serde(rename = "retryPeriod")]
    pub retry_period: String,
}

/// Metrics contains thw controller metrics configuration
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigMetrics {
    /// BindAddress is the TCP address that the controller should bind to for serving prometheus metrics. It can be set to "0" to disable the metrics serving.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindAddress")]
    pub bind_address: Option<String>,
}

/// Webhook contains the controllers webhook configuration
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProjectConfigWebhook {
    /// CertDir is the directory that contains the server key and certificate. if not set, webhook server would look up the server key and certificate in {TempDir}/k8s-webhook-server/serving-certs. The server key and certificate must be named tls.key and tls.crt, respectively.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certDir")]
    pub cert_dir: Option<String>,
    /// Host is the hostname that the webhook server binds to. It is used to set webhook.Server.Host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Port is the port that the webhook server serves at. It is used to set webhook.Server.Port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

