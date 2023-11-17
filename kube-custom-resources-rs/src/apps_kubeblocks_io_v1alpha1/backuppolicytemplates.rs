// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/backuppolicytemplates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// BackupPolicyTemplateSpec defines the desired state of BackupPolicyTemplate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "BackupPolicyTemplate", plural = "backuppolicytemplates")]
#[kube(status = "BackupPolicyTemplateStatus")]
#[kube(schema = "disabled")]
pub struct BackupPolicyTemplateSpec {
    /// backupPolicies is a list of backup policy template for the specified componentDefinition.
    #[serde(rename = "backupPolicies")]
    pub backup_policies: Vec<BackupPolicyTemplateBackupPolicies>,
    /// clusterDefinitionRef references ClusterDefinition name, this is an immutable attribute.
    #[serde(rename = "clusterDefinitionRef")]
    pub cluster_definition_ref: String,
    /// Identifier is a unique identifier for this BackupPolicyTemplate. this identifier will be the suffix of the automatically generated backupPolicy name. and must be added when multiple BackupPolicyTemplates exist, otherwise the generated backupPolicy override will occur.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPolicies {
    /// backupMethods defines the backup methods.
    #[serde(rename = "backupMethods")]
    pub backup_methods: Vec<BackupPolicyTemplateBackupPoliciesBackupMethods>,
    /// componentDefRef references componentDef defined in ClusterDefinition spec. Need to comply with IANA Service Naming rule.
    #[serde(rename = "componentDefRef")]
    pub component_def_ref: String,
    /// schedule policy for backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<BackupPolicyTemplateBackupPoliciesSchedules>>,
    /// target instance for backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<BackupPolicyTemplateBackupPoliciesTarget>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethods {
    /// actionSetName refers to the ActionSet object that defines the backup actions. For volume snapshot backup, the actionSet is not required, the controller will use the CSI volume snapshotter to create the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSetName")]
    pub action_set_name: Option<String>,
    /// env specifies the environment variables for the backup workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<BackupPolicyTemplateBackupPoliciesBackupMethodsEnv>>,
    /// envMapping defines the variables of cluster mapped to env values' keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envMapping")]
    pub env_mapping: Option<Vec<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMapping>>,
    /// the name of backup method.
    pub name: String,
    /// runtimeSettings specifies runtime settings for the backup workload container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runtimeSettings")]
    pub runtime_settings: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettings>,
    /// snapshotVolumes specifies whether to take snapshots of persistent volumes. if true, the BackupScript is not required, the controller will use the CSI volume snapshotter to create the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotVolumes")]
    pub snapshot_volumes: Option<bool>,
    /// targetVolumes specifies which volumes from the target should be mounted in the backup workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetVolumes")]
    pub target_volumes: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsTargetVolumes>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMapping {
    /// env key which needs to mapping.
    pub key: String,
    /// valueFrom defines source of the env value.
    #[serde(rename = "valueFrom")]
    pub value_from: BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMappingValueFrom,
}

/// valueFrom defines source of the env value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMappingValueFrom {
    /// mapped ClusterVersionRef to env value.
    #[serde(rename = "clusterVersionRef")]
    pub cluster_version_ref: Vec<BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMappingValueFromClusterVersionRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsEnvMappingValueFromClusterVersionRef {
    /// mapping value for the specified ClusterVersion names.
    #[serde(rename = "mappingValue")]
    pub mapping_value: String,
    /// the array of ClusterVersion name which can be mapped to the env value.
    pub names: Vec<String>,
}

/// runtimeSettings specifies runtime settings for the backup workload container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettings {
    /// resources specifies the resource required by container. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettingsResources>,
}

/// resources specifies the resource required by container. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettingsResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettingsResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsRuntimeSettingsResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// targetVolumes specifies which volumes from the target should be mounted in the backup workload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsTargetVolumes {
    /// volumeMounts specifies the mount for the volumes specified in `Volumes` section.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeMounts")]
    pub volume_mounts: Option<Vec<BackupPolicyTemplateBackupPoliciesBackupMethodsTargetVolumesVolumeMounts>>,
    /// Volumes indicates the list of volumes of targeted application that should be mounted on the backup job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesBackupMethodsTargetVolumesVolumeMounts {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPropagation")]
    pub mount_propagation: Option<String>,
    /// This must match the Name of a Volume.
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subPath")]
    pub sub_path: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subPathExpr")]
    pub sub_path_expr: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesSchedules {
    /// backupMethod specifies the backup method name that is defined in backupPolicy.
    #[serde(rename = "backupMethod")]
    pub backup_method: String,
    /// the cron expression for schedule, the timezone is in UTC. see https://en.wikipedia.org/wiki/Cron.
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// enabled specifies whether the backup schedule is enabled or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// retentionPeriod determines a duration up to which the backup should be kept. controller will remove all backups that are older than the RetentionPeriod. For example, RetentionPeriod of `30d` will keep only the backups of last 30 days. Sample duration format: - years: 	2y - months: 	6mo - days: 		30d - hours: 	12h - minutes: 	30m You can also combine the above durations. For example: 30d12h30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionPeriod")]
    pub retention_period: Option<String>,
}

/// target instance for backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesTarget {
    /// refer to spec.componentDef.systemAccounts.accounts[*].name in ClusterDefinition. the secret created by this account will be used to connect the database. if not set, the secret created by spec.ConnectionCredential of the ClusterDefinition will be used. it will be transformed to a secret for BackupPolicy's target secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// connectionCredentialKey defines connection credential key in secret which created by spec.ConnectionCredential of the ClusterDefinition. it will be ignored when "account" is set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionCredentialKey")]
    pub connection_credential_key: Option<BackupPolicyTemplateBackupPoliciesTargetConnectionCredentialKey>,
    /// select instance of corresponding role for backup, role are: - the name of Leader/Follower/Leaner for Consensus component. - primary or secondary for Replication component. finally, invalid role of the component will be ignored. such as if workload type is Replication and component's replicas is 1, the secondary role is invalid. and it also will be ignored when component is Stateful/Stateless. the role will be transformed to a role LabelSelector for BackupPolicy's target attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// connectionCredentialKey defines connection credential key in secret which created by spec.ConnectionCredential of the ClusterDefinition. it will be ignored when "account" is set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateBackupPoliciesTargetConnectionCredentialKey {
    /// hostKey specifies the map key of the host in the connection credential secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostKey")]
    pub host_key: Option<String>,
    /// the key of password in the ConnectionCredential secret. if not set, the default key is "password".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordKey")]
    pub password_key: Option<String>,
    /// portKey specifies the map key of the port in the connection credential secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portKey")]
    pub port_key: Option<String>,
    /// the key of username in the ConnectionCredential secret. if not set, the default key is "username".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernameKey")]
    pub username_key: Option<String>,
}

/// BackupPolicyTemplateStatus defines the observed state of BackupPolicyTemplate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupPolicyTemplateStatus {
}

