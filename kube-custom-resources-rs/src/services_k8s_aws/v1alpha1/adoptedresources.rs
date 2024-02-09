// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/eks-controller/services.k8s.aws/v1alpha1/adoptedresources.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// AdoptedResourceSpec defines the desired state of the AdoptedResource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "services.k8s.aws", version = "v1alpha1", kind = "AdoptedResource", plural = "adoptedresources")]
#[kube(namespaced)]
#[kube(status = "AdoptedResourceStatus")]
#[kube(schema = "disabled")]
pub struct AdoptedResourceSpec {
    /// AWSIdentifiers provide all unique ways to reference an AWS resource.
    pub aws: AdoptedResourceAws,
    /// ResourceWithMetadata provides the values necessary to create a
    /// Kubernetes resource and override any of its metadata values.
    pub kubernetes: AdoptedResourceKubernetes,
}

/// AWSIdentifiers provide all unique ways to reference an AWS resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceAws {
    /// AdditionalKeys represents any additional arbitrary identifiers used when
    /// describing the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalKeys")]
    pub additional_keys: Option<BTreeMap<String, String>>,
    /// ARN is the AWS Resource Name for the resource. It is a globally
    /// unique identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// NameOrId is a user-supplied string identifier for the resource. It may
    /// or may not be globally unique, depending on the type of resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameOrID")]
    pub name_or_id: Option<String>,
}

/// ResourceWithMetadata provides the values necessary to create a
/// Kubernetes resource and override any of its metadata values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceKubernetes {
    pub group: String,
    pub kind: String,
    /// ObjectMeta is metadata that all persisted resources must have, which includes all objects
    /// users must create.
    /// It is not possible to use `metav1.ObjectMeta` inside spec, as the controller-gen
    /// automatically converts this to an arbitrary string-string map.
    /// https://github.com/kubernetes-sigs/controller-tools/issues/385
    /// 
    /// 
    /// Active discussion about inclusion of this field in the spec is happening in this PR:
    /// https://github.com/kubernetes-sigs/controller-tools/pull/395
    /// 
    /// 
    /// Until this is allowed, or if it never is, we will produce a subset of the object meta
    /// that contains only the fields which the user is allowed to modify in the metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AdoptedResourceKubernetesMetadata>,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
/// It is not possible to use `metav1.ObjectMeta` inside spec, as the controller-gen
/// automatically converts this to an arbitrary string-string map.
/// https://github.com/kubernetes-sigs/controller-tools/issues/385
/// 
/// 
/// Active discussion about inclusion of this field in the spec is happening in this PR:
/// https://github.com/kubernetes-sigs/controller-tools/pull/395
/// 
/// 
/// Until this is allowed, or if it never is, we will produce a subset of the object meta
/// that contains only the fields which the user is allowed to modify in the metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceKubernetesMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// GenerateName is an optional prefix, used by the server, to generate a unique
    /// name ONLY IF the Name field has not been provided.
    /// If this field is used, the name returned to the client will be different
    /// than the name passed. This value will also be combined with a unique suffix.
    /// The provided value has the same validation rules as the Name field,
    /// and may be truncated by the length of the suffix required to make the value
    /// unique on the server.
    /// 
    /// 
    /// If this field is specified and the generated name exists, the server will
    /// NOT return a 409 - instead, it will either return 201 Created or 500 with Reason
    /// ServerTimeout indicating a unique name could not be found in the time allotted, and the client
    /// should retry (optionally after the time indicated in the Retry-After header).
    /// 
    /// 
    /// Applied only if Name is not specified.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateName")]
    pub generate_name: Option<String>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name must be unique within a namespace. Is required when creating resources, although
    /// some resources may allow a client to request the generation of an appropriate name
    /// automatically. Name is primarily intended for creation idempotence and configuration
    /// definition.
    /// Cannot be updated.
    /// More info: http://kubernetes.io/docs/user-guide/identifiers#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within each name must be unique. An empty namespace is
    /// equivalent to the "default" namespace, but "default" is the canonical representation.
    /// Not all objects are required to be scoped to a namespace - the value of this field for
    /// those objects will be empty.
    /// 
    /// 
    /// Must be a DNS_LABEL.
    /// Cannot be updated.
    /// More info: http://kubernetes.io/docs/user-guide/namespaces
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// List of objects depended by this object. If ALL objects in the list have
    /// been deleted, this object will be garbage collected. If this object is managed by a controller,
    /// then an entry in this list will point to this controller, with the controller field set to true.
    /// There cannot be more than one managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerReferences")]
    pub owner_references: Option<Vec<AdoptedResourceKubernetesMetadataOwnerReferences>>,
}

/// OwnerReference contains enough information to let you identify an owning
/// object. An owning object must be in the same namespace as the dependent, or
/// be cluster-scoped, so there is no namespace field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceKubernetesMetadataOwnerReferences {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then
    /// the owner cannot be deleted from the key-value store until this
    /// reference is removed.
    /// See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion
    /// for how the garbage collector interacts with this field and enforces the foreground deletion.
    /// Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner,
    /// otherwise 422 (Unprocessable Entity) will be returned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blockOwnerDeletion")]
    pub block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names
    pub name: String,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    pub uid: String,
}

/// AdoptedResourceStatus defines the observed status of the AdoptedResource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceStatus {
    /// A collection of `ackv1alpha1.Condition` objects that describe the various
    /// terminal states of the adopted resource CR and its target custom resource
    pub conditions: Vec<AdoptedResourceStatusConditions>,
}

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdoptedResourceStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

