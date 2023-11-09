// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/kubedl-io/kubedl/cache.kubedl.io/v1alpha1/cachebackends.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cache.kubedl.io", version = "v1alpha1", kind = "CacheBackend", plural = "cachebackends")]
#[kube(namespaced)]
#[kube(status = "CacheBackendStatus")]
#[kube(schema = "disabled")]
pub struct CacheBackendSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheEngine")]
    pub cache_engine: Option<CacheBackendCacheEngine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<CacheBackendDataset>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CacheBackendOptions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendCacheEngine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fluid: Option<CacheBackendCacheEngineFluid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendCacheEngineFluid {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alluxioRuntime")]
    pub alluxio_runtime: Option<CacheBackendCacheEngineFluidAlluxioRuntime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendCacheEngineFluidAlluxioRuntime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tieredStorage")]
    pub tiered_storage: Option<Vec<CacheBackendCacheEngineFluidAlluxioRuntimeTieredStorage>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendCacheEngineFluidAlluxioRuntimeTieredStorage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachePath")]
    pub cache_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediumType")]
    pub medium_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendDataset {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSources")]
    pub data_sources: Option<Vec<CacheBackendDatasetDataSources>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendDatasetDataSources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subDirName")]
    pub sub_dir_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTime")]
    pub idle_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheBackendStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheEngine")]
    pub cache_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheStatus")]
    pub cache_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUsedTime")]
    pub last_used_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usedBy")]
    pub used_by: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usedNum")]
    pub used_num: Option<i64>,
}

