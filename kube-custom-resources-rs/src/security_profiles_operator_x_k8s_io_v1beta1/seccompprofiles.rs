// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/kubernetes-sigs/security-profiles-operator/security-profiles-operator.x-k8s.io/v1beta1/seccompprofiles.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SeccompProfileSpec defines the desired state of SeccompProfile.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security-profiles-operator.x-k8s.io", version = "v1beta1", kind = "SeccompProfile", plural = "seccompprofiles")]
#[kube(namespaced)]
#[kube(status = "SeccompProfileStatus")]
#[kube(schema = "disabled")]
pub struct SeccompProfileSpec {
    /// the architecture used for system calls
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// BaseProfileName is the name of base profile (in the same namespace) that will be unioned into this profile. Base profiles can be references as remote OCI artifacts as well when prefixed with `oci://`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseProfileName")]
    pub base_profile_name: Option<String>,
    /// the default action for seccomp
    #[serde(rename = "defaultAction")]
    pub default_action: SeccompProfileDefaultAction,
    /// Whether the profile is disabled and should be skipped during reconciliation.
    pub disabled: bool,
    /// list of flags to use with seccomp(2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    /// opaque data to pass to the seccomp agent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerMetadata")]
    pub listener_metadata: Option<String>,
    /// path of UNIX domain socket to contact a seccomp agent for SCMP_ACT_NOTIFY
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerPath")]
    pub listener_path: Option<String>,
    /// match a syscall in seccomp. While this property is OPTIONAL, some values of defaultAction are not useful without syscalls entries. For example, if defaultAction is SCMP_ACT_KILL and syscalls is empty or unset, the kernel will kill the container process on its first syscall
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syscalls: Option<Vec<SeccompProfileSyscalls>>,
}

/// SeccompProfileSpec defines the desired state of SeccompProfile.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SeccompProfileDefaultAction {
    #[serde(rename = "SCMP_ACT_KILL")]
    ScmpActKill,
    #[serde(rename = "SCMP_ACT_KILL_PROCESS")]
    ScmpActKillProcess,
    #[serde(rename = "SCMP_ACT_KILL_THREAD")]
    ScmpActKillThread,
    #[serde(rename = "SCMP_ACT_TRAP")]
    ScmpActTrap,
    #[serde(rename = "SCMP_ACT_ERRNO")]
    ScmpActErrno,
    #[serde(rename = "SCMP_ACT_TRACE")]
    ScmpActTrace,
    #[serde(rename = "SCMP_ACT_ALLOW")]
    ScmpActAllow,
    #[serde(rename = "SCMP_ACT_LOG")]
    ScmpActLog,
    #[serde(rename = "SCMP_ACT_NOTIFY")]
    ScmpActNotify,
}

/// Syscall defines a syscall in seccomp.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SeccompProfileSyscalls {
    /// the action for seccomp rules
    pub action: SeccompProfileSyscallsAction,
    /// the specific syscall in seccomp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<SeccompProfileSyscallsArgs>>,
    /// the errno return code to use. Some actions like SCMP_ACT_ERRNO and SCMP_ACT_TRACE allow to specify the errno code to return
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errnoRet")]
    pub errno_ret: Option<i64>,
    /// the names of the syscalls
    pub names: Vec<String>,
}

/// Syscall defines a syscall in seccomp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SeccompProfileSyscallsAction {
    #[serde(rename = "SCMP_ACT_KILL")]
    ScmpActKill,
    #[serde(rename = "SCMP_ACT_KILL_PROCESS")]
    ScmpActKillProcess,
    #[serde(rename = "SCMP_ACT_KILL_THREAD")]
    ScmpActKillThread,
    #[serde(rename = "SCMP_ACT_TRAP")]
    ScmpActTrap,
    #[serde(rename = "SCMP_ACT_ERRNO")]
    ScmpActErrno,
    #[serde(rename = "SCMP_ACT_TRACE")]
    ScmpActTrace,
    #[serde(rename = "SCMP_ACT_ALLOW")]
    ScmpActAllow,
    #[serde(rename = "SCMP_ACT_LOG")]
    ScmpActLog,
    #[serde(rename = "SCMP_ACT_NOTIFY")]
    ScmpActNotify,
}

/// Arg defines the specific syscall in seccomp.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SeccompProfileSyscallsArgs {
    /// the index for syscall arguments in seccomp
    pub index: i64,
    /// the operator for syscall arguments in seccomp
    pub op: SeccompProfileSyscallsArgsOp,
    /// the value for syscall arguments in seccomp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// the value for syscall arguments in seccomp
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueTwo")]
    pub value_two: Option<i64>,
}

/// Arg defines the specific syscall in seccomp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SeccompProfileSyscallsArgsOp {
    #[serde(rename = "SCMP_CMP_NE")]
    ScmpCmpNe,
    #[serde(rename = "SCMP_CMP_LT")]
    ScmpCmpLt,
    #[serde(rename = "SCMP_CMP_LE")]
    ScmpCmpLe,
    #[serde(rename = "SCMP_CMP_EQ")]
    ScmpCmpEq,
    #[serde(rename = "SCMP_CMP_GE")]
    ScmpCmpGe,
    #[serde(rename = "SCMP_CMP_GT")]
    ScmpCmpGt,
    #[serde(rename = "SCMP_CMP_MASKED_EQ")]
    ScmpCmpMaskedEq,
}

/// SeccompProfileStatus contains status of the deployed SeccompProfile.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SeccompProfileStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeWorkloads")]
    pub active_workloads: Option<Vec<String>>,
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SeccompProfileStatusConditions>>,
    /// The path that should be provided to the `securityContext.seccompProfile.localhostProfile` field of a Pod or container spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// ProfileState defines the state that the profile is in. A profile in this context refers to a SeccompProfile or a SELinux profile, the states are shared between them as well as the management API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SeccompProfileStatusConditions {
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
    pub status: SeccompProfileStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SeccompProfileStatusConditionsStatus {
    True,
    False,
    Unknown,
}

