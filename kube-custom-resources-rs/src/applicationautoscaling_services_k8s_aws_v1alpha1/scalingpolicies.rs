// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/aws-controllers-k8s/applicationautoscaling-controller/applicationautoscaling.services.k8s.aws/v1alpha1/scalingpolicies.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ScalingPolicySpec defines the desired state of ScalingPolicy. 
///  Represents a scaling policy to use with Application Auto Scaling. 
///  For more information about configuring scaling policies for a specific service, see Getting started with Application Auto Scaling (https://docs.aws.amazon.com/autoscaling/application/userguide/getting-started.html) in the Application Auto Scaling User Guide.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "applicationautoscaling.services.k8s.aws", version = "v1alpha1", kind = "ScalingPolicy", plural = "scalingpolicies")]
#[kube(namespaced)]
#[kube(status = "ScalingPolicyStatus")]
#[kube(schema = "disabled")]
pub struct ScalingPolicySpec {
    /// The name of the scaling policy.
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// The policy type. This parameter is required if you are creating a scaling policy. 
    ///  The following policy types are supported: 
    ///  TargetTrackingScaling—Not supported for Amazon EMR 
    ///  StepScaling—Not supported for DynamoDB, Amazon Comprehend, Lambda, Amazon Keyspaces, Amazon MSK, Amazon ElastiCache, or Neptune. 
    ///  For more information, see Target tracking scaling policies (https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html) and Step scaling policies (https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html) in the Application Auto Scaling User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyType")]
    pub policy_type: Option<String>,
    /// The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier. 
    ///  * ECS service - The resource type is service and the unique identifier is the cluster name and service name. Example: service/default/sample-webapp. 
    ///  * Spot Fleet - The resource type is spot-fleet-request and the unique identifier is the Spot Fleet request ID. Example: spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE. 
    ///  * EMR cluster - The resource type is instancegroup and the unique identifier is the cluster ID and instance group ID. Example: instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0. 
    ///  * AppStream 2.0 fleet - The resource type is fleet and the unique identifier is the fleet name. Example: fleet/sample-fleet. 
    ///  * DynamoDB table - The resource type is table and the unique identifier is the table name. Example: table/my-table. 
    ///  * DynamoDB global secondary index - The resource type is index and the unique identifier is the index name. Example: table/my-table/index/my-table-index. 
    ///  * Aurora DB cluster - The resource type is cluster and the unique identifier is the cluster name. Example: cluster:my-db-cluster. 
    ///  * SageMaker endpoint variant - The resource type is variant and the unique identifier is the resource ID. Example: endpoint/my-end-point/variant/KMeansClustering. 
    ///  * Custom resources are not supported with a resource type. This parameter must specify the OutputValue from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our GitHub repository (https://github.com/aws/aws-auto-scaling-custom-resource). 
    ///  * Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE. 
    ///  * Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE. 
    ///  * Lambda provisioned concurrency - The resource type is function and the unique identifier is the function name with a function version or alias name suffix that is not $LATEST. Example: function:my-function:prod or function:my-function:1. 
    ///  * Amazon Keyspaces table - The resource type is table and the unique identifier is the table name. Example: keyspace/mykeyspace/table/mytable. 
    ///  * Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN. Example: arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5. 
    ///  * Amazon ElastiCache replication group - The resource type is replication-group and the unique identifier is the replication group name. Example: replication-group/mycluster. 
    ///  * Neptune cluster - The resource type is cluster and the unique identifier is the cluster name. Example: cluster:mycluster.
    #[serde(rename = "resourceID")]
    pub resource_id: String,
    /// The scalable dimension. This string consists of the service namespace, resource type, and scaling property. 
    ///  * ecs:service:DesiredCount - The desired task count of an ECS service. 
    ///  * elasticmapreduce:instancegroup:InstanceCount - The instance count of an EMR Instance Group. 
    ///  * ec2:spot-fleet-request:TargetCapacity - The target capacity of a Spot Fleet. 
    ///  * appstream:fleet:DesiredCapacity - The desired capacity of an AppStream 2.0 fleet. 
    ///  * dynamodb:table:ReadCapacityUnits - The provisioned read capacity for a DynamoDB table. 
    ///  * dynamodb:table:WriteCapacityUnits - The provisioned write capacity for a DynamoDB table. 
    ///  * dynamodb:index:ReadCapacityUnits - The provisioned read capacity for a DynamoDB global secondary index. 
    ///  * dynamodb:index:WriteCapacityUnits - The provisioned write capacity for a DynamoDB global secondary index. 
    ///  * rds:cluster:ReadReplicaCount - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition. 
    ///  * sagemaker:variant:DesiredInstanceCount - The number of EC2 instances for an SageMaker model endpoint variant. 
    ///  * custom-resource:ResourceType:Property - The scalable dimension for a custom resource provided by your own application or service. 
    ///  * comprehend:document-classifier-endpoint:DesiredInferenceUnits - The number of inference units for an Amazon Comprehend document classification endpoint. 
    ///  * comprehend:entity-recognizer-endpoint:DesiredInferenceUnits - The number of inference units for an Amazon Comprehend entity recognizer endpoint. 
    ///  * lambda:function:ProvisionedConcurrency - The provisioned concurrency for a Lambda function. 
    ///  * cassandra:table:ReadCapacityUnits - The provisioned read capacity for an Amazon Keyspaces table. 
    ///  * cassandra:table:WriteCapacityUnits - The provisioned write capacity for an Amazon Keyspaces table. 
    ///  * kafka:broker-storage:VolumeSize - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster. 
    ///  * elasticache:replication-group:NodeGroups - The number of node groups for an Amazon ElastiCache replication group. 
    ///  * elasticache:replication-group:Replicas - The number of replicas per node group for an Amazon ElastiCache replication group. 
    ///  * neptune:cluster:ReadReplicaCount - The count of read replicas in an Amazon Neptune DB cluster.
    #[serde(rename = "scalableDimension")]
    pub scalable_dimension: String,
    /// The namespace of the Amazon Web Services service that provides the resource. For a resource provided by your own application or service, use custom-resource instead.
    #[serde(rename = "serviceNamespace")]
    pub service_namespace: String,
    /// A step scaling policy. 
    ///  This parameter is required if you are creating a policy and the policy type is StepScaling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stepScalingPolicyConfiguration")]
    pub step_scaling_policy_configuration: Option<ScalingPolicyStepScalingPolicyConfiguration>,
    /// A target tracking scaling policy. Includes support for predefined or customized metrics. 
    ///  This parameter is required if you are creating a policy and the policy type is TargetTrackingScaling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: Option<ScalingPolicyTargetTrackingScalingPolicyConfiguration>,
}

/// A step scaling policy. 
///  This parameter is required if you are creating a policy and the policy type is StepScaling.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStepScalingPolicyConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adjustmentType")]
    pub adjustment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricAggregationType")]
    pub metric_aggregation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minAdjustmentMagnitude")]
    pub min_adjustment_magnitude: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stepAdjustments")]
    pub step_adjustments: Option<Vec<ScalingPolicyStepScalingPolicyConfigurationStepAdjustments>>,
}

/// Represents a step adjustment for a StepScalingPolicyConfiguration (https://docs.aws.amazon.com/autoscaling/application/APIReference/API_StepScalingPolicyConfiguration.html). Describes an adjustment based on the difference between the value of the aggregated CloudWatch metric and the breach threshold that you've defined for the alarm. 
///  For the following examples, suppose that you have an alarm with a breach threshold of 50: 
///  * To trigger the adjustment when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10. 
///  * To trigger the adjustment when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0. 
///  There are a few rules for the step adjustments for your step policy: 
///  * The ranges of your step adjustments can't overlap or have a gap. 
///  * At most one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound. 
///  * At most one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound. 
///  * The upper and lower bound can't be null in the same step adjustment.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStepScalingPolicyConfigurationStepAdjustments {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricIntervalLowerBound")]
    pub metric_interval_lower_bound: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricIntervalUpperBound")]
    pub metric_interval_upper_bound: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scalingAdjustment")]
    pub scaling_adjustment: Option<i64>,
}

/// A target tracking scaling policy. Includes support for predefined or customized metrics. 
///  This parameter is required if you are creating a policy and the policy type is TargetTrackingScaling.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyTargetTrackingScalingPolicyConfiguration {
    /// Represents a CloudWatch metric of your choosing for a target tracking scaling policy to use with Application Auto Scaling. 
    ///  For information about the available metrics for a service, see Amazon Web Services Services That Publish CloudWatch Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/aws-services-cloudwatch-metrics.html) in the Amazon CloudWatch User Guide. 
    ///  To create your customized metric specification: 
    ///  * Add values for each required parameter from CloudWatch. You can use an existing metric, or a new metric that you create. To use your own metric, you must first publish the metric to CloudWatch. For more information, see Publish Custom Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html) in the Amazon CloudWatch User Guide. 
    ///  * Choose a metric that changes proportionally with capacity. The value of the metric should increase or decrease in inverse proportion to the number of capacity units. That is, the value of the metric should decrease when capacity increases, and increase when capacity decreases. 
    ///  For more information about CloudWatch, see Amazon CloudWatch Concepts (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customizedMetricSpecification")]
    pub customized_metric_specification: Option<ScalingPolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableScaleIn")]
    pub disable_scale_in: Option<bool>,
    /// Represents a predefined metric for a target tracking scaling policy to use with Application Auto Scaling. 
    ///  Only the Amazon Web Services that you're using send metrics to Amazon CloudWatch. To determine whether a desired metric already exists by looking up its namespace and dimension using the CloudWatch metrics dashboard in the console, follow the procedure in Building dashboards with CloudWatch (https://docs.aws.amazon.com/autoscaling/application/userguide/monitoring-cloudwatch.html) in the Application Auto Scaling User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "predefinedMetricSpecification")]
    pub predefined_metric_specification: Option<ScalingPolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleInCooldown")]
    pub scale_in_cooldown: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleOutCooldown")]
    pub scale_out_cooldown: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetValue")]
    pub target_value: Option<f64>,
}

/// Represents a CloudWatch metric of your choosing for a target tracking scaling policy to use with Application Auto Scaling. 
///  For information about the available metrics for a service, see Amazon Web Services Services That Publish CloudWatch Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/aws-services-cloudwatch-metrics.html) in the Amazon CloudWatch User Guide. 
///  To create your customized metric specification: 
///  * Add values for each required parameter from CloudWatch. You can use an existing metric, or a new metric that you create. To use your own metric, you must first publish the metric to CloudWatch. For more information, see Publish Custom Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html) in the Amazon CloudWatch User Guide. 
///  * Choose a metric that changes proportionally with capacity. The value of the metric should increase or decrease in inverse proportion to the number of capacity units. That is, the value of the metric should decrease when capacity increases, and increase when capacity decreases. 
///  For more information about CloudWatch, see Amazon CloudWatch Concepts (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<ScalingPolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationDimensions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricName")]
    pub metric_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// Describes the dimension names and values associated with a metric.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationDimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Represents a predefined metric for a target tracking scaling policy to use with Application Auto Scaling. 
///  Only the Amazon Web Services that you're using send metrics to Amazon CloudWatch. To determine whether a desired metric already exists by looking up its namespace and dimension using the CloudWatch metrics dashboard in the console, follow the procedure in Building dashboards with CloudWatch (https://docs.aws.amazon.com/autoscaling/application/userguide/monitoring-cloudwatch.html) in the Application Auto Scaling User Guide.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "predefinedMetricType")]
    pub predefined_metric_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceLabel")]
    pub resource_label: Option<String>,
}

/// ScalingPolicyStatus defines the observed state of ScalingPolicy
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ScalingPolicyStatusAckResourceMetadata>,
    /// The CloudWatch alarms created for the target tracking scaling policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<ScalingPolicyStatusAlarms>>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ScalingPolicyStatusConditions>>,
    /// The Unix timestamp for when the scaling policy was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The Unix timestamp for when the scaling policy was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a globally-unique identifier and is set only by the ACK service controller once the controller has orchestrated the creation of the resource OR when it has verified that an "adopted" resource (a resource where the ARN annotation was set by the Kubernetes user on the CR) exists and matches the supplied CR's Spec field values. TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Represents a CloudWatch alarm associated with a scaling policy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStatusAlarms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmARN")]
    pub alarm_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmName")]
    pub alarm_name: Option<String>,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalingPolicyStatusConditions {
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

