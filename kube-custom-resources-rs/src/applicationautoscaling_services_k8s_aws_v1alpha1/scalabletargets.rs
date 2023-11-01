// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/aws-controllers-k8s/applicationautoscaling-controller/applicationautoscaling.services.k8s.aws/v1alpha1/scalabletargets.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// ScalableTargetSpec defines the desired state of ScalableTarget. 
///  Represents a scalable target.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "applicationautoscaling.services.k8s.aws", version = "v1alpha1", kind = "ScalableTarget", plural = "scalabletargets")]
#[kube(namespaced)]
#[kube(status = "ScalableTargetStatus")]
pub struct ScalableTargetSpec {
    /// The maximum value that you plan to scale out to. When a scaling policy is in effect, Application Auto Scaling can scale out (expand) as needed to the maximum capacity limit in response to changing demand. This property is required when registering a new scalable target. 
    ///  Although you can specify a large maximum capacity, note that service quotas may impose lower limits. Each service has its own default quotas for the maximum capacity of the resource. If you want to specify a higher limit, you can request an increase. For more information, consult the documentation for that service. For information about the default quotas for each service, see Service Endpoints and Quotas (https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html) in the Amazon Web Services General Reference.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxCapacity")]
    pub max_capacity: Option<i64>,
    /// The minimum value that you plan to scale in to. When a scaling policy is in effect, Application Auto Scaling can scale in (contract) as needed to the minimum capacity limit in response to changing demand. This property is required when registering a new scalable target. 
    ///  For certain resources, the minimum value allowed is 0. This includes Lambda provisioned concurrency, Spot Fleet, ECS services, Aurora DB clusters, EMR clusters, and custom resources. For all other resources, the minimum value allowed is 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minCapacity")]
    pub min_capacity: Option<i64>,
    /// The identifier of the resource that is associated with the scalable target. This string consists of the resource type and unique identifier. 
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
    /// This parameter is required for services that do not support service-linked roles (such as Amazon EMR), and it must specify the ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf. 
    ///  If the service supports service-linked roles, Application Auto Scaling uses a service-linked role, which it creates if it does not yet exist. For more information, see Application Auto Scaling IAM roles (https://docs.aws.amazon.com/autoscaling/application/userguide/security_iam_service-with-iam.html#security_iam_service-with-iam-roles).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
    /// The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property. 
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
    /// An embedded object that contains attributes and attribute values that are used to suspend and resume automatic scaling. Setting the value of an attribute to true suspends the specified scaling activities. Setting it to false (default) resumes the specified scaling activities. 
    ///  Suspension Outcomes 
    ///  * For DynamicScalingInSuspended, while a suspension is in effect, all scale-in activities that are triggered by a scaling policy are suspended. 
    ///  * For DynamicScalingOutSuspended, while a suspension is in effect, all scale-out activities that are triggered by a scaling policy are suspended. 
    ///  * For ScheduledScalingSuspended, while a suspension is in effect, all scaling activities that involve scheduled actions are suspended. 
    ///  For more information, see Suspending and resuming scaling (https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-suspend-resume-scaling.html) in the Application Auto Scaling User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "suspendedState")]
    pub suspended_state: Option<ScalableTargetSuspendedState>,
}

/// An embedded object that contains attributes and attribute values that are used to suspend and resume automatic scaling. Setting the value of an attribute to true suspends the specified scaling activities. Setting it to false (default) resumes the specified scaling activities. 
///  Suspension Outcomes 
///  * For DynamicScalingInSuspended, while a suspension is in effect, all scale-in activities that are triggered by a scaling policy are suspended. 
///  * For DynamicScalingOutSuspended, while a suspension is in effect, all scale-out activities that are triggered by a scaling policy are suspended. 
///  * For ScheduledScalingSuspended, while a suspension is in effect, all scaling activities that involve scheduled actions are suspended. 
///  For more information, see Suspending and resuming scaling (https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-suspend-resume-scaling.html) in the Application Auto Scaling User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScalableTargetSuspendedState {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicScalingInSuspended")]
    pub dynamic_scaling_in_suspended: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicScalingOutSuspended")]
    pub dynamic_scaling_out_suspended: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduledScalingSuspended")]
    pub scheduled_scaling_suspended: Option<bool>,
}

/// ScalableTargetStatus defines the observed state of ScalableTarget
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScalableTargetStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ScalableTargetStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ScalableTargetStatusConditions>>,
    /// The Unix timestamp for when the scalable target was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The Unix timestamp for when the scalable target was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScalableTargetStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a globally-unique identifier and is set only by the ACK service controller once the controller has orchestrated the creation of the resource OR when it has verified that an "adopted" resource (a resource where the ARN annotation was set by the Kubernetes user on the CR) exists and matches the supplied CR's Spec field values. TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ScalableTargetStatusConditions {
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

