// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/argoproj-labs/argocd-operator/argoproj.io/v1alpha1/argocdexports.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// ArgoCDExportSpec defines the desired state of ArgoCDExport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "argoproj.io", version = "v1alpha1", kind = "ArgoCDExport", plural = "argocdexports")]
#[kube(namespaced)]
#[kube(status = "ArgoCDExportStatus")]
pub struct ArgoCDExportSpec {
    /// Argocd is the name of the ArgoCD instance to export.
    pub argocd: String,
    /// Image is the container image to use for the export Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// Storage defines the storage configuration options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<ArgoCDExportStorage>,
    /// Version is the tag/digest to use for the export Job container image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Storage defines the storage configuration options.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStorage {
    /// Backend defines the storage backend to use, must be "local" (the default), "aws", "azure" or "gcp".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,
    /// PVC is the desired characteristics for a PersistentVolumeClaim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pvc: Option<ArgoCDExportStoragePvc>,
    /// SecretName is the name of a Secret with encryption key, credentials, etc.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// PVC is the desired characteristics for a PersistentVolumeClaim.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvc {
    /// accessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessModes")]
    pub access_modes: Option<Vec<String>>,
    /// dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSource")]
    pub data_source: Option<ArgoCDExportStoragePvcDataSource>,
    /// dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef   allows any non-core object, as well as PersistentVolumeClaim objects. * While dataSource ignores disallowed values (dropping them), dataSourceRef   preserves all values, and generates an error if a disallowed value is   specified. * While dataSource only allows local objects, dataSourceRef allows objects   in any namespaces. (Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSourceRef")]
    pub data_source_ref: Option<ArgoCDExportStoragePvcDataSourceRef>,
    /// resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ArgoCDExportStoragePvcResources>,
    /// selector is a label query over volumes to consider for binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ArgoCDExportStoragePvcSelector>,
    /// storageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeMode")]
    pub volume_mode: Option<String>,
    /// volumeName is the binding reference to the PersistentVolume backing this claim.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
}

/// dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcDataSource {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef   allows any non-core object, as well as PersistentVolumeClaim objects. * While dataSource ignores disallowed values (dropping them), dataSourceRef   preserves all values, and generates an error if a disallowed value is   specified. * While dataSource only allows local objects, dataSourceRef allows objects   in any namespaces. (Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcDataSourceRef {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
    /// Namespace is the namespace of resource being referenced Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. (Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ArgoCDExportStoragePvcResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// selector is a label query over volumes to consider for binding.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ArgoCDExportStoragePvcSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStoragePvcSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ArgoCDExportStatus defines the observed state of ArgoCDExport
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ArgoCDExportStatus {
    /// Phase is a simple, high-level summary of where the ArgoCDExport is in its lifecycle. There are five possible phase values: Pending: The ArgoCDExport has been accepted by the Kubernetes system, but one or more of the required resources have not been created. Running: All of the containers for the ArgoCDExport are still running, or in the process of starting or restarting. Succeeded: All containers for the ArgoCDExport have terminated in success, and will not be restarted. Failed: At least one container has terminated in failure, either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the ArgoCDExport could not be obtained.
    pub phase: String,
}

