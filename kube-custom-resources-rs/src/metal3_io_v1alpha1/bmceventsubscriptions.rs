// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/metal3-io/baremetal-operator/metal3.io/v1alpha1/bmceventsubscriptions.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "metal3.io", version = "v1alpha1", kind = "BMCEventSubscription", plural = "bmceventsubscriptions")]
#[kube(namespaced)]
#[kube(status = "BMCEventSubscriptionStatus")]
pub struct BMCEventSubscriptionSpec {
    /// Arbitrary user-provided context for the event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// A webhook URL to send events to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// A reference to a BareMetalHost
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostName")]
    pub host_name: Option<String>,
    /// A secret containing HTTP headers which should be passed along to the Destination when making a request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpHeadersRef")]
    pub http_headers_ref: Option<BMCEventSubscriptionHttpHeadersRef>,
}

/// A secret containing HTTP headers which should be passed along to the Destination when making a request
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct BMCEventSubscriptionHttpHeadersRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct BMCEventSubscriptionStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionID")]
    pub subscription_id: Option<String>,
}

