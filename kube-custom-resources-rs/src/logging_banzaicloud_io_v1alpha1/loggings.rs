// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kube-logging/logging-operator/logging.banzaicloud.io/v1alpha1/loggings.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "logging.banzaicloud.io", version = "v1alpha1", kind = "Logging", plural = "loggings")]
#[kube(status = "LoggingStatus")]
pub struct LoggingSpec {
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LoggingStatus {
}

