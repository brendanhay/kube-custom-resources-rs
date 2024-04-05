// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kernel-module-management/kmm.sigs.x-k8s.io/v1beta1/preflightvalidations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// PreflightValidationSpec describes the desired state of the resource, such as the kernel version
/// that Module CRs need to be verified against as well as the debug configuration of the logs
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kmm.sigs.x-k8s.io", version = "v1beta1", kind = "PreflightValidation", plural = "preflightvalidations")]
#[kube(status = "PreflightValidationStatus")]
#[kube(schema = "disabled")]
pub struct PreflightValidationSpec {
    /// KernelVersion describes the kernel image that all Modules need to be checked against.
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,
    /// Boolean flag that determines whether images build during preflight must also
    /// be pushed to a defined repository
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pushBuiltImage")]
    pub push_built_image: Option<bool>,
}

/// PreflightValidationStatus is the most recently observed status of the PreflightValidation.
/// It is populated by the system and is read-only.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PreflightValidationStatus {
    /// CRStatuses contain observations about each Module's preflight upgradability validation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crStatuses")]
    pub cr_statuses: Option<BTreeMap<String, PreflightValidationStatusCrStatuses>>,
}

/// CRStatuses contain observations about each Module's preflight upgradability validation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PreflightValidationStatusCrStatuses {
    /// LastTransitionTime is the last time the CR status transitioned from one status to another.
    /// This should be when the underlying status changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// StatusReason contains a string describing the status source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusReason")]
    pub status_reason: Option<String>,
    /// Current stage of the verification process:
    /// image (image existence verification), build(build process verification)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verificationStage")]
    pub verification_stage: Option<PreflightValidationStatusCrStatusesVerificationStage>,
    /// Status of Module CR verification: true (verified), false (verification failed),
    /// error (error during verification process), unknown (verification has not started yet)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verificationStatus")]
    pub verification_status: Option<PreflightValidationStatusCrStatusesVerificationStatus>,
}

/// CRStatuses contain observations about each Module's preflight upgradability validation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreflightValidationStatusCrStatusesVerificationStage {
    Image,
    Build,
    Sign,
    Requeued,
    Done,
}

/// CRStatuses contain observations about each Module's preflight upgradability validation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreflightValidationStatusCrStatusesVerificationStatus {
    True,
    False,
}

