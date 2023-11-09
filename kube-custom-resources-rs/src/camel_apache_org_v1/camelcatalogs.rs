// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/apache/camel-k/camel.apache.org/v1/camelcatalogs.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// the desired state of the catalog
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "camel.apache.org", version = "v1", kind = "CamelCatalog", plural = "camelcatalogs")]
#[kube(namespaced)]
#[kube(status = "CamelCatalogStatus")]
#[kube(schema = "disabled")]
pub struct CamelCatalogSpec {
    /// artifacts required by this catalog
    pub artifacts: BTreeMap<String, CamelCatalogArtifacts>,
    /// loaders required by this catalog
    pub loaders: BTreeMap<String, CamelCatalogLoaders>,
    /// the runtime targeted for the catalog
    pub runtime: CamelCatalogRuntime,
}

/// artifacts required by this catalog
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifacts {
    /// Maven Artifact
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "artifactId")]
    pub artifact_id: Option<String>,
    /// accepted data formats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataformats: Option<Vec<String>>,
    /// required dependencies
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CamelCatalogArtifactsDependencies>>,
    /// provide a list of artifacts to exclude for this dependency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<CamelCatalogArtifactsExclusions>>,
    /// Maven Group
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupId")]
    pub group_id: Option<String>,
    /// the Java types used by the artifact feature (ie, component, data format, ...)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "javaTypes")]
    pub java_types: Option<Vec<String>>,
    /// accepted languages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// accepted URI schemes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<CamelCatalogArtifactsSchemes>>,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// CamelArtifactDependency represent a maven's dependency.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// provide a list of artifacts to exclude for this dependency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<CamelCatalogArtifactsDependenciesExclusions>>,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// CamelArtifactExclusion represents an exclusion clause.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsDependenciesExclusions {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// CamelArtifactExclusion represents an exclusion clause.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsExclusions {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// CamelScheme represents the scheme used to identify a component in a URI (ie, timer in a timer:xyz endpoint URI).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemes {
    /// required scope for consumer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumer: Option<CamelCatalogArtifactsSchemesConsumer>,
    /// is a HTTP based scheme
    pub http: bool,
    /// the ID (ie, timer in a timer:xyz URI)
    pub id: String,
    /// is a passive scheme
    pub passive: bool,
    /// required scope for producers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<CamelCatalogArtifactsSchemesProducer>,
}

/// required scope for consumer
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesConsumer {
    /// list of dependencies needed for this scope
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CamelCatalogArtifactsSchemesConsumerDependencies>>,
}

/// CamelArtifactDependency represent a maven's dependency.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesConsumerDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// provide a list of artifacts to exclude for this dependency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<CamelCatalogArtifactsSchemesConsumerDependenciesExclusions>>,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// CamelArtifactExclusion represents an exclusion clause.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesConsumerDependenciesExclusions {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// required scope for producers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesProducer {
    /// list of dependencies needed for this scope
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CamelCatalogArtifactsSchemesProducerDependencies>>,
}

/// CamelArtifactDependency represent a maven's dependency.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesProducerDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// provide a list of artifacts to exclude for this dependency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<CamelCatalogArtifactsSchemesProducerDependenciesExclusions>>,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// CamelArtifactExclusion represents an exclusion clause.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogArtifactsSchemesProducerDependenciesExclusions {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// loaders required by this catalog
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogLoaders {
    /// Maven Artifact
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "artifactId")]
    pub artifact_id: Option<String>,
    /// a list of additional dependencies required beside the base one
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CamelCatalogLoadersDependencies>>,
    /// Maven Group
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupId")]
    pub group_id: Option<String>,
    /// a list of DSLs supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// the metadata of the loader
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// MavenArtifact defines a GAV (Group:Artifact:Version) Maven artifact.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogLoadersDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// the runtime targeted for the catalog
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogRuntime {
    /// application entry point (main) to be executed
    #[serde(rename = "applicationClass")]
    pub application_class: String,
    /// features offered by this runtime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<BTreeMap<String, CamelCatalogRuntimeCapabilities>>,
    /// list of dependencies needed to run the application
    pub dependencies: Vec<CamelCatalogRuntimeDependencies>,
    /// set of metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Camel main application provider, ie, Camel Quarkus
    pub provider: String,
    /// Camel K Runtime version
    pub version: String,
}

/// features offered by this runtime
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogRuntimeCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CamelCatalogRuntimeCapabilitiesDependencies>>,
}

/// MavenArtifact defines a GAV (Group:Artifact:Version) Maven artifact.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogRuntimeCapabilitiesDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// MavenArtifact defines a GAV (Group:Artifact:Version) Maven artifact.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogRuntimeDependencies {
    /// Maven Artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Maven Group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Maven Version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// the actual state of the catalog
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogStatus {
    /// a list of events happened for the CamelCatalog
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CamelCatalogStatusConditions>>,
    /// the container image available for building an application with this catalog
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ObservedGeneration is the most recent generation observed for this Catalog.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// the actual phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// CamelCatalogCondition describes the state of a resource at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CamelCatalogStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// The last time this condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// A human-readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of CamelCatalog condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

