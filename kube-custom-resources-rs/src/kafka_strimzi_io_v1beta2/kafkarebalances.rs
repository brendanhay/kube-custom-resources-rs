// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/strimzi/strimzi-kafka-operator/kafka.strimzi.io/v1beta2/kafkarebalances.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// The specification of the Kafka rebalance.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kafka.strimzi.io", version = "v1beta2", kind = "KafkaRebalance", plural = "kafkarebalances")]
#[kube(namespaced)]
#[kube(status = "KafkaRebalanceStatus")]
#[kube(schema = "disabled")]
pub struct KafkaRebalanceSpec {
    /// The list of newly added brokers in case of scaling up or the ones to be removed in case of scaling down to use for rebalancing. This list can be used only with rebalancing mode `add-brokers` and `removed-brokers`. It is ignored with `full` mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brokers: Option<Vec<i64>>,
    /// The upper bound of ongoing partition replica movements between disks within each broker. Default is 2.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentIntraBrokerPartitionMovements")]
    pub concurrent_intra_broker_partition_movements: Option<i64>,
    /// The upper bound of ongoing partition leadership movements. Default is 1000.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentLeaderMovements")]
    pub concurrent_leader_movements: Option<i64>,
    /// The upper bound of ongoing partition replica movements going into/out of each broker. Default is 5.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentPartitionMovementsPerBroker")]
    pub concurrent_partition_movements_per_broker: Option<i64>,
    /// A regular expression where any matching topics will be excluded from the calculation of optimization proposals. This expression will be parsed by the java.util.regex.Pattern class; for more information on the supported format consult the documentation for that class.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedTopics")]
    pub excluded_topics: Option<String>,
    /// A list of goals, ordered by decreasing priority, to use for generating and executing the rebalance proposal. The supported goals are available at https://github.com/linkedin/cruise-control#goals. If an empty goals list is provided, the goals declared in the default.goals Cruise Control configuration parameter are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goals: Option<Vec<String>>,
    /// Mode to run the rebalancing. The supported modes are `full`, `add-brokers`, `remove-brokers`.
    /// If not specified, the `full` mode is used by default. 
    /// 
    /// * `full` mode runs the rebalancing across all the brokers in the cluster.
    /// * `add-brokers` mode can be used after scaling up the cluster to move some replicas to the newly added brokers.
    /// * `remove-brokers` mode can be used before scaling down the cluster to move replicas out of the brokers to be removed.
    /// 
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<KafkaRebalanceMode>,
    /// Enables intra-broker disk balancing, which balances disk space utilization between disks on the same broker. Only applies to Kafka deployments that use JBOD storage with multiple disks. When enabled, inter-broker balancing is disabled. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rebalanceDisk")]
    pub rebalance_disk: Option<bool>,
    /// A list of strategy class names used to determine the execution order for the replica movements in the generated optimization proposal. By default BaseReplicaMovementStrategy is used, which will execute the replica movements in the order that they were generated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaMovementStrategies")]
    pub replica_movement_strategies: Option<Vec<String>>,
    /// The upper bound, in bytes per second, on the bandwidth used to move replicas. There is no limit by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationThrottle")]
    pub replication_throttle: Option<i64>,
    /// Whether to allow the hard goals specified in the Kafka CR to be skipped in optimization proposal generation. This can be useful when some of those hard goals are preventing a balance solution being found. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipHardGoalCheck")]
    pub skip_hard_goal_check: Option<bool>,
}

/// The specification of the Kafka rebalance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaRebalanceMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "add-brokers")]
    AddBrokers,
    #[serde(rename = "remove-brokers")]
    RemoveBrokers,
}

/// The status of the Kafka rebalance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaRebalanceStatus {
    /// List of status conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<KafkaRebalanceStatusConditions>>,
    /// The generation of the CRD that was last reconciled by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// A JSON object describing the optimization result.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optimizationResult")]
    pub optimization_result: Option<BTreeMap<String, serde_json::Value>>,
    /// The session identifier for requests to Cruise Control pertaining to this KafkaRebalance resource. This is used by the Kafka Rebalance operator to track the status of ongoing rebalancing operations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaRebalanceStatusConditions {
    /// Last time the condition of a type changed from one status to another. The required format is 'yyyy-MM-ddTHH:mm:ssZ', in the UTC time zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition (a single word in CamelCase).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The status of the condition, either True, False or Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The unique identifier of a condition, used to distinguish between other conditions in the resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

