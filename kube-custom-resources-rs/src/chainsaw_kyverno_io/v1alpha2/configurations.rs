// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kyverno/chainsaw/chainsaw.kyverno.io/v1alpha2/configurations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Configuration spec.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chainsaw.kyverno.io", version = "v1alpha2", kind = "Configuration", plural = "configurations")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConfigurationSpec {
    /// Cleanup contains cleanup configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<ConfigurationCleanup>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationClusters>>,
    /// Deletion contains the global deletion configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<ConfigurationDeletion>,
    /// Discovery contains tests discovery configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<ConfigurationDiscovery>,
    /// Error contains the global error configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfigurationError>,
    /// Execution contains tests execution configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<ConfigurationExecution>,
    /// Namespace contains properties for the namespace to use for tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<ConfigurationNamespace>,
    /// Report contains properties for the report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<ConfigurationReport>,
    /// Templating contains the templating config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templating: Option<ConfigurationTemplating>,
    /// Global timeouts configuration. Applies to all tests/test steps if not overridden.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<ConfigurationTimeouts>,
}

/// Cleanup contains cleanup configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationCleanup {
    /// DelayBeforeCleanup adds a delay between the time a test ends and the time cleanup starts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "delayBeforeCleanup")]
    pub delay_before_cleanup: Option<String>,
    /// If set, do not delete the resources after running a test.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipDelete")]
    pub skip_delete: Option<bool>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Deletion contains the global deletion configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDeletion {
    /// Propagation decides if a deletion will propagate to the dependents of
    /// the object, and how the garbage collector will handle the propagation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation: Option<ConfigurationDeletionPropagation>,
}

/// Deletion contains the global deletion configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationDeletionPropagation {
    Orphan,
    Background,
    Foreground,
}

/// Discovery contains tests discovery configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscovery {
    /// ExcludeTestRegex is used to exclude tests based on a regular expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeTestRegex")]
    pub exclude_test_regex: Option<String>,
    /// FullName makes use of the full test case folder path instead of the folder name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullName")]
    pub full_name: Option<bool>,
    /// IncludeTestRegex is used to include tests based on a regular expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeTestRegex")]
    pub include_test_regex: Option<String>,
    /// TestFile is the name of the file containing the test to run.
    /// If no extension is provided, chainsaw will try with .yaml first and .yml if needed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "testFile")]
    pub test_file: Option<String>,
}

/// Error contains the global error configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationError {
    /// Catch defines what the tests steps will execute when an error happens.
    /// This will be combined with catch handlers defined at the test and step levels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catch: Option<Vec<ConfigurationErrorCatch>>,
}

/// CatchFinally defines actions to be executed in catch, finally and cleanup blocks.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatch {
    /// Command defines a command to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<ConfigurationErrorCatchCommand>,
    /// Delete represents a deletion operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<ConfigurationErrorCatchDelete>,
    /// Describe determines the resource describe collector to execute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub describe: Option<ConfigurationErrorCatchDescribe>,
    /// Description contains a description of the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Events determines the events collector to execute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<ConfigurationErrorCatchEvents>,
    /// Get determines the resource get collector to execute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<ConfigurationErrorCatchGet>,
    /// PodLogs determines the pod logs collector to execute.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podLogs")]
    pub pod_logs: Option<ConfigurationErrorCatchPodLogs>,
    /// Script defines a script to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ConfigurationErrorCatchScript>,
    /// Sleep defines zzzz.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<ConfigurationErrorCatchSleep>,
    /// Wait determines the resource wait collector to execute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<ConfigurationErrorCatchWait>,
}

/// Command defines a command to run.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchCommand {
    /// Args is the command arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Bindings defines additional binding key/values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ConfigurationErrorCatchCommandBindings>>,
    /// Check is an assertion tree to validate the operation outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<BTreeMap<String, serde_json::Value>>,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchCommandClusters>>,
    /// Entrypoint is the command entry point to run.
    pub entrypoint: String,
    /// Env defines additional environment variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ConfigurationErrorCatchCommandEnv>>,
    /// Outputs defines output bindings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ConfigurationErrorCatchCommandOutputs>>,
    /// SkipLogOutput removes the output from the command. Useful for sensitive logs or to reduce noise.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipLogOutput")]
    pub skip_log_output: Option<bool>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Binding represents a key/value set as a binding in an executing test.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchCommandBindings {
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchCommandClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Binding represents a key/value set as a binding in an executing test.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchCommandEnv {
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Output represents an output binding with a match to determine if the binding must be considered or not.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchCommandOutputs {
    /// Match defines the matching statement.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<BTreeMap<String, serde_json::Value>>,
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Delete represents a deletion operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDelete {
    /// Bindings defines additional binding key/values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ConfigurationErrorCatchDeleteBindings>>,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchDeleteClusters>>,
    /// DeletionPropagationPolicy decides if a deletion will propagate to the dependents of
    /// the object, and how the garbage collector will handle the propagation.
    /// Overrides the deletion propagation policy set in the Configuration, the Test and the TestStep.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionPropagationPolicy")]
    pub deletion_propagation_policy: Option<ConfigurationErrorCatchDeleteDeletionPropagationPolicy>,
    /// Expect defines a list of matched checks to validate the operation outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect: Option<Vec<ConfigurationErrorCatchDeleteExpect>>,
    /// File is the path to the referenced file. This can be a direct path to a file
    /// or an expression that matches multiple files, such as "manifest/*.yaml" for all YAML
    /// files within the "manifest" directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Ref determines objects to be deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ref")]
    pub r#ref: Option<ConfigurationErrorCatchDeleteRef>,
    /// Template determines whether resources should be considered for templating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Binding represents a key/value set as a binding in an executing test.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDeleteBindings {
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDeleteClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Delete represents a deletion operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationErrorCatchDeleteDeletionPropagationPolicy {
    Orphan,
    Background,
    Foreground,
}

/// Expectation represents a check to be applied on the result of an operation
/// with a match filter to determine if the verification should be considered.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDeleteExpect {
    /// Check defines the verification statement.
    pub check: BTreeMap<String, serde_json::Value>,
    /// Match defines the matching statement.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<BTreeMap<String, serde_json::Value>>,
}

/// Ref determines objects to be deleted.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDeleteRef {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Label selector to match objects to delete
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Describe determines the resource describe collector to execute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDescribe {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchDescribeClusters>>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Selector defines labels selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Show Events indicates whether to include related events.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "showEvents")]
    pub show_events: Option<bool>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchDescribeClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Events determines the events collector to execute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchEvents {
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchEventsClusters>>,
    /// Format determines the output format (json or yaml).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Selector defines labels selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchEventsClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Get determines the resource get collector to execute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchGet {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchGetClusters>>,
    /// Format determines the output format (json or yaml).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Selector defines labels selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchGetClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// PodLogs determines the pod logs collector to execute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchPodLogs {
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchPodLogsClusters>>,
    /// Container in pod to get logs from else --all-containers is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Selector defines labels selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Tail is the number of last lines to collect from pods. If omitted or zero,
    /// then the default is 10 if you use a selector, or -1 (all) if you use a pod name.
    /// This matches default behavior of `kubectl logs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<i64>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchPodLogsClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Script defines a script to run.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchScript {
    /// Bindings defines additional binding key/values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ConfigurationErrorCatchScriptBindings>>,
    /// Check is an assertion tree to validate the operation outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<BTreeMap<String, serde_json::Value>>,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchScriptClusters>>,
    /// Content defines a shell script (run with "sh -c ...").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Env defines additional environment variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ConfigurationErrorCatchScriptEnv>>,
    /// Outputs defines output bindings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ConfigurationErrorCatchScriptOutputs>>,
    /// SkipLogOutput removes the output from the command. Useful for sensitive logs or to reduce noise.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipLogOutput")]
    pub skip_log_output: Option<bool>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Binding represents a key/value set as a binding in an executing test.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchScriptBindings {
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchScriptClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// Binding represents a key/value set as a binding in an executing test.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchScriptEnv {
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Output represents an output binding with a match to determine if the binding must be considered or not.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchScriptOutputs {
    /// Match defines the matching statement.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<BTreeMap<String, serde_json::Value>>,
    /// Name the name of the binding.
    pub name: String,
    /// Value value of the binding.
    pub value: serde_json::Value,
}

/// Sleep defines zzzz.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchSleep {
    /// Duration is the delay used for sleeping.
    pub duration: String,
}

/// Wait determines the resource wait collector to execute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWait {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Cluster defines the target cluster (will be inherited if not specified).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Clusters holds a registry to clusters to support multi-cluster tests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, ConfigurationErrorCatchWaitClusters>>,
    /// WaitFor specifies the condition to wait for.
    #[serde(rename = "for")]
    pub r#for: ConfigurationErrorCatchWaitFor,
    /// Format determines the output format (json or yaml).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Selector defines labels selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Timeout for the operation. Overrides the global timeout set in the Configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Clusters holds a registry to clusters to support multi-cluster tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWaitClusters {
    /// Context is the name of the context to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Kubeconfig is the path to the referenced file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
}

/// WaitFor specifies the condition to wait for.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWaitFor {
    /// Condition specifies the condition to wait for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConfigurationErrorCatchWaitForCondition>,
    /// Deletion specifies parameters for waiting on a resource's deletion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<ConfigurationErrorCatchWaitForDeletion>,
    /// JsonPath specifies the json path condition to wait for.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsonPath")]
    pub json_path: Option<ConfigurationErrorCatchWaitForJsonPath>,
}

/// Condition specifies the condition to wait for.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWaitForCondition {
    /// Name defines the specific condition to wait for, e.g., "Available", "Ready".
    pub name: String,
    /// Value defines the specific condition status to wait for, e.g., "True", "False".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Deletion specifies parameters for waiting on a resource's deletion.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWaitForDeletion {
}

/// JsonPath specifies the json path condition to wait for.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationErrorCatchWaitForJsonPath {
    /// Path defines the json path to wait for, e.g. '{.status.phase}'.
    pub path: String,
    /// Value defines the expected value to wait for, e.g., "Running".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Execution contains tests execution configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationExecution {
    /// FailFast determines whether the test should stop upon encountering the first failure.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failFast")]
    pub fail_fast: Option<bool>,
    /// ForceTerminationGracePeriod forces the termination grace period on pods, statefulsets, daemonsets and deployments.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forceTerminationGracePeriod")]
    pub force_termination_grace_period: Option<String>,
    /// The maximum number of tests to run at once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel: Option<i64>,
    /// RepeatCount indicates how many times the tests should be executed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repeatCount")]
    pub repeat_count: Option<i64>,
}

/// Namespace contains properties for the namespace to use for tests.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationNamespace {
    /// Name defines the namespace to use for tests.
    /// If not specified, every test will execute in a random ephemeral namespace
    /// unless the namespace is overridden in a the test spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Template defines a template to create the test namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<BTreeMap<String, serde_json::Value>>,
}

/// Report contains properties for the report.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationReport {
    /// ReportFormat determines test report format (JSON|XML).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ConfigurationReportFormat>,
    /// ReportName defines the name of report to create. It defaults to "chainsaw-report".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ReportPath defines the path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Report contains properties for the report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationReportFormat {
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "XML")]
    Xml,
}

/// Templating contains the templating config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationTemplating {
    /// Enabled determines whether resources should be considered for templating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Global timeouts configuration. Applies to all tests/test steps if not overridden.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationTimeouts {
    /// Apply defines the timeout for the apply operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply: Option<String>,
    /// Assert defines the timeout for the assert operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assert: Option<String>,
    /// Cleanup defines the timeout for the cleanup operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<String>,
    /// Delete defines the timeout for the delete operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<String>,
    /// Error defines the timeout for the error operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Exec defines the timeout for exec operations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<String>,
}

