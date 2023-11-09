// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/koordinator-sh/koordinator/scheduling.sigs.k8s.io/v1alpha1/elasticquotas.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// ElasticQuotaSpec defines the Min and Max for Quota.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scheduling.sigs.k8s.io", version = "v1alpha1", kind = "ElasticQuota", plural = "elasticquotas")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ElasticQuotaSpec {
    /// Max is the set of desired max limits for each named resource. The usage of max is based on the resource configurations of successfully scheduled pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<BTreeMap<String, IntOrString>>,
    /// Min is the set of desired guaranteed limits for each named resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<BTreeMap<String, IntOrString>>,
}

/// ElasticQuotaStatus defines the observed use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ElasticQuotaStatus {
    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<BTreeMap<String, IntOrString>>,
}

