// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/rook/rook/ceph.rook.io/v1/cephobjectzones.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ObjectZoneSpec represent the spec of an ObjectZone
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephObjectZone", plural = "cephobjectzones")]
#[kube(namespaced)]
#[kube(status = "CephObjectZoneStatus")]
#[kube(schema = "disabled")]
pub struct CephObjectZoneSpec {
    /// If this zone cannot be accessed from other peer Ceph clusters via the ClusterIP Service endpoint created by Rook, you must set this to the externally reachable endpoint(s). You may include the port in the definition. For example: "https://my-object-store.my-domain.net:443". In many cases, you should set this to the endpoint of the ingress resource that makes the CephObjectStore associated with this CephObjectStoreZone reachable to peer clusters. The list can have one or more endpoints pointing to different RGW servers in the zone. 
    ///  If a CephObjectStore endpoint is omitted from this list, that object store's gateways will not receive multisite replication data (see CephObjectStore.spec.gateway.disableMultisiteSyncTraffic).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customEndpoints")]
    pub custom_endpoints: Option<Vec<String>>,
    /// The data pool settings
    #[serde(rename = "dataPool")]
    pub data_pool: CephObjectZoneDataPool,
    /// The metadata pool settings
    #[serde(rename = "metadataPool")]
    pub metadata_pool: CephObjectZoneMetadataPool,
    /// Preserve pools on object zone deletion
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preservePoolsOnDelete")]
    pub preserve_pools_on_delete: Option<bool>,
    /// The display name for the ceph users
    #[serde(rename = "zoneGroup")]
    pub zone_group: String,
}

/// The data pool settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPool {
    /// DEPRECATED: use Parameters instead, e.g., Parameters["compression_mode"] = "force" The inline compression mode in Bluestore OSD to set to (options are: none, passive, aggressive, force) Do NOT set a default value for kubebuilder as this will override the Parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMode")]
    pub compression_mode: Option<CephObjectZoneDataPoolCompressionMode>,
    /// The root of the crush hierarchy utilized by the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crushRoot")]
    pub crush_root: Option<String>,
    /// The device class the OSD should set to for use in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceClass")]
    pub device_class: Option<String>,
    /// EnableRBDStats is used to enable gathering of statistics for all RBD images in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRBDStats")]
    pub enable_rbd_stats: Option<bool>,
    /// The erasure code settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "erasureCoded")]
    pub erasure_coded: Option<CephObjectZoneDataPoolErasureCoded>,
    /// The failure domain: osd/host/(region or zone if available) - technically also any type in the crush map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// The mirroring settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirroring: Option<CephObjectZoneDataPoolMirroring>,
    /// Parameters is a list of properties to enable on a given pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// The quota settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotas: Option<CephObjectZoneDataPoolQuotas>,
    /// The replication settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicated: Option<CephObjectZoneDataPoolReplicated>,
    /// The mirroring statusCheck
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCheck")]
    pub status_check: Option<CephObjectZoneDataPoolStatusCheck>,
}

/// The data pool settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectZoneDataPoolCompressionMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// The erasure code settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolErasureCoded {
    /// The algorithm for erasure coding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Number of coding chunks per object in an erasure coded storage pool (required for erasure-coded pool type). This is the number of OSDs that can be lost simultaneously before data cannot be recovered.
    #[serde(rename = "codingChunks")]
    pub coding_chunks: i64,
    /// Number of data chunks per object in an erasure coded storage pool (required for erasure-coded pool type). The number of chunks required to recover an object when any single OSD is lost is the same as dataChunks so be aware that the larger the number of data chunks, the higher the cost of recovery.
    #[serde(rename = "dataChunks")]
    pub data_chunks: i64,
}

/// The mirroring settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolMirroring {
    /// Enabled whether this pool is mirrored or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Mode is the mirroring mode: either pool or image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Peers represents the peers spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<CephObjectZoneDataPoolMirroringPeers>,
    /// SnapshotSchedules is the scheduling of snapshot for mirrored images/pools
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephObjectZoneDataPoolMirroringSnapshotSchedules>>,
}

/// Peers represents the peers spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolMirroringPeers {
    /// SecretNames represents the Kubernetes Secret names to add rbd-mirror or cephfs-mirror peers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretNames")]
    pub secret_names: Option<Vec<String>>,
}

/// SnapshotScheduleSpec represents the snapshot scheduling settings of a mirrored pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolMirroringSnapshotSchedules {
    /// Interval represent the periodicity of the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to snapshot, only valid for CephFS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// StartTime indicates when to start the snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// The quota settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolQuotas {
    /// MaxBytes represents the quota in bytes Deprecated in favor of MaxSize
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBytes")]
    pub max_bytes: Option<i64>,
    /// MaxObjects represents the quota in objects
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxObjects")]
    pub max_objects: Option<i64>,
    /// MaxSize represents the quota in bytes as a string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<String>,
}

/// The replication settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolReplicated {
    /// HybridStorage represents hybrid storage tier settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hybridStorage")]
    pub hybrid_storage: Option<CephObjectZoneDataPoolReplicatedHybridStorage>,
    /// ReplicasPerFailureDomain the number of replica in the specified failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicasPerFailureDomain")]
    pub replicas_per_failure_domain: Option<i64>,
    /// RequireSafeReplicaSize if false allows you to set replica 1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireSafeReplicaSize")]
    pub require_safe_replica_size: Option<bool>,
    /// Size - Number of copies per object in a replicated storage pool, including the object itself (required for replicated pool type)
    pub size: i64,
    /// SubFailureDomain the name of the sub-failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFailureDomain")]
    pub sub_failure_domain: Option<String>,
    /// TargetSizeRatio gives a hint (%) to Ceph in terms of expected consumption of the total cluster capacity
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetSizeRatio")]
    pub target_size_ratio: Option<f64>,
}

/// HybridStorage represents hybrid storage tier settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolReplicatedHybridStorage {
    /// PrimaryDeviceClass represents high performance tier (for example SSD or NVME) for Primary OSD
    #[serde(rename = "primaryDeviceClass")]
    pub primary_device_class: String,
    /// SecondaryDeviceClass represents low performance tier (for example HDDs) for remaining OSDs
    #[serde(rename = "secondaryDeviceClass")]
    pub secondary_device_class: String,
}

/// The mirroring statusCheck
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolStatusCheck {
    /// HealthCheckSpec represents the health check of an object store bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<CephObjectZoneDataPoolStatusCheckMirror>,
}

/// HealthCheckSpec represents the health check of an object store bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneDataPoolStatusCheckMirror {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Interval is the internal in second or minute for the health check to run like 60s for 60 seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// The metadata pool settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPool {
    /// DEPRECATED: use Parameters instead, e.g., Parameters["compression_mode"] = "force" The inline compression mode in Bluestore OSD to set to (options are: none, passive, aggressive, force) Do NOT set a default value for kubebuilder as this will override the Parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMode")]
    pub compression_mode: Option<CephObjectZoneMetadataPoolCompressionMode>,
    /// The root of the crush hierarchy utilized by the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crushRoot")]
    pub crush_root: Option<String>,
    /// The device class the OSD should set to for use in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceClass")]
    pub device_class: Option<String>,
    /// EnableRBDStats is used to enable gathering of statistics for all RBD images in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRBDStats")]
    pub enable_rbd_stats: Option<bool>,
    /// The erasure code settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "erasureCoded")]
    pub erasure_coded: Option<CephObjectZoneMetadataPoolErasureCoded>,
    /// The failure domain: osd/host/(region or zone if available) - technically also any type in the crush map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// The mirroring settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirroring: Option<CephObjectZoneMetadataPoolMirroring>,
    /// Parameters is a list of properties to enable on a given pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// The quota settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotas: Option<CephObjectZoneMetadataPoolQuotas>,
    /// The replication settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicated: Option<CephObjectZoneMetadataPoolReplicated>,
    /// The mirroring statusCheck
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCheck")]
    pub status_check: Option<CephObjectZoneMetadataPoolStatusCheck>,
}

/// The metadata pool settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectZoneMetadataPoolCompressionMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// The erasure code settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolErasureCoded {
    /// The algorithm for erasure coding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Number of coding chunks per object in an erasure coded storage pool (required for erasure-coded pool type). This is the number of OSDs that can be lost simultaneously before data cannot be recovered.
    #[serde(rename = "codingChunks")]
    pub coding_chunks: i64,
    /// Number of data chunks per object in an erasure coded storage pool (required for erasure-coded pool type). The number of chunks required to recover an object when any single OSD is lost is the same as dataChunks so be aware that the larger the number of data chunks, the higher the cost of recovery.
    #[serde(rename = "dataChunks")]
    pub data_chunks: i64,
}

/// The mirroring settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolMirroring {
    /// Enabled whether this pool is mirrored or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Mode is the mirroring mode: either pool or image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Peers represents the peers spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<CephObjectZoneMetadataPoolMirroringPeers>,
    /// SnapshotSchedules is the scheduling of snapshot for mirrored images/pools
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephObjectZoneMetadataPoolMirroringSnapshotSchedules>>,
}

/// Peers represents the peers spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolMirroringPeers {
    /// SecretNames represents the Kubernetes Secret names to add rbd-mirror or cephfs-mirror peers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretNames")]
    pub secret_names: Option<Vec<String>>,
}

/// SnapshotScheduleSpec represents the snapshot scheduling settings of a mirrored pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolMirroringSnapshotSchedules {
    /// Interval represent the periodicity of the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to snapshot, only valid for CephFS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// StartTime indicates when to start the snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// The quota settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolQuotas {
    /// MaxBytes represents the quota in bytes Deprecated in favor of MaxSize
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBytes")]
    pub max_bytes: Option<i64>,
    /// MaxObjects represents the quota in objects
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxObjects")]
    pub max_objects: Option<i64>,
    /// MaxSize represents the quota in bytes as a string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<String>,
}

/// The replication settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolReplicated {
    /// HybridStorage represents hybrid storage tier settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hybridStorage")]
    pub hybrid_storage: Option<CephObjectZoneMetadataPoolReplicatedHybridStorage>,
    /// ReplicasPerFailureDomain the number of replica in the specified failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicasPerFailureDomain")]
    pub replicas_per_failure_domain: Option<i64>,
    /// RequireSafeReplicaSize if false allows you to set replica 1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireSafeReplicaSize")]
    pub require_safe_replica_size: Option<bool>,
    /// Size - Number of copies per object in a replicated storage pool, including the object itself (required for replicated pool type)
    pub size: i64,
    /// SubFailureDomain the name of the sub-failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFailureDomain")]
    pub sub_failure_domain: Option<String>,
    /// TargetSizeRatio gives a hint (%) to Ceph in terms of expected consumption of the total cluster capacity
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetSizeRatio")]
    pub target_size_ratio: Option<f64>,
}

/// HybridStorage represents hybrid storage tier settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolReplicatedHybridStorage {
    /// PrimaryDeviceClass represents high performance tier (for example SSD or NVME) for Primary OSD
    #[serde(rename = "primaryDeviceClass")]
    pub primary_device_class: String,
    /// SecondaryDeviceClass represents low performance tier (for example HDDs) for remaining OSDs
    #[serde(rename = "secondaryDeviceClass")]
    pub secondary_device_class: String,
}

/// The mirroring statusCheck
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolStatusCheck {
    /// HealthCheckSpec represents the health check of an object store bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<CephObjectZoneMetadataPoolStatusCheckMirror>,
}

/// HealthCheckSpec represents the health check of an object store bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneMetadataPoolStatusCheckMirror {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Interval is the internal in second or minute for the health check to run like 60s for 60 seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Status represents the status of an object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CephObjectZoneStatusConditions>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// Condition represents a status condition on any Rook-Ceph Custom Resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHeartbeatTime")]
    pub last_heartbeat_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ConditionReason is a reason for a condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

