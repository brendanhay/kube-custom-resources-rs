// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/aws-controllers-k8s/lambda-controller/lambda.services.k8s.aws/v1alpha1/functions.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// FunctionSpec defines the desired state of Function.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "lambda.services.k8s.aws", version = "v1alpha1", kind = "Function", plural = "functions")]
#[kube(namespaced)]
#[kube(status = "FunctionStatus")]
pub struct FunctionSpec {
    /// The instruction set architecture that the function supports. Enter a string array with one of the valid values (arm64 or x86_64). The default value is x86_64.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// The code for the function.
    pub code: FunctionCode,
    /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSigningConfigARN")]
    pub code_signing_config_arn: Option<String>,
    /// A dead-letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see Dead-letter queues (https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-dlq).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deadLetterConfig")]
    pub dead_letter_config: Option<FunctionDeadLetterConfig>,
    /// A description of the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Environment variables that are accessible from function code during execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<FunctionEnvironment>,
    /// The size of the function's /tmp directory in MB. The default value is 512, but can be any whole number between 512 and 10,240 MB.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ephemeralStorage")]
    pub ephemeral_storage: Option<FunctionEphemeralStorage>,
    /// Connection settings for an Amazon EFS file system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemConfigs")]
    pub file_system_configs: Option<Vec<FunctionFileSystemConfigs>>,
    /// Configures options for asynchronous invocation on a function. 
    ///  - DestinationConfig A destination for events after they have been sent to a function for processing. 
    ///  Types of Destinations: Function - The Amazon Resource Name (ARN) of a Lambda function. Queue - The ARN of a standard SQS queue. Topic - The ARN of a standard SNS topic. Event Bus - The ARN of an Amazon EventBridge event bus. 
    ///  - MaximumEventAgeInSeconds The maximum age of a request that Lambda sends to a function for processing. 
    ///  - MaximumRetryAttempts The maximum number of times to retry when the function returns an error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionEventInvokeConfig")]
    pub function_event_invoke_config: Option<FunctionFunctionEventInvokeConfig>,
    /// The name of the method within your code that Lambda calls to run your function. Handler is required if the deployment package is a .zip file archive. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information, see Lambda programming model (https://docs.aws.amazon.com/lambda/latest/dg/foundation-progmodel.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// Container image configuration values (https://docs.aws.amazon.com/lambda/latest/dg/configuration-images.html#configuration-images-settings) that override the values in the container image Dockerfile.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfig")]
    pub image_config: Option<FunctionImageConfig>,
    /// The ARN of the Key Management Service (KMS) key that's used to encrypt your function's environment variables. If it's not provided, Lambda uses a default service key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyARN")]
    pub kms_key_arn: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
    ///  from: name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyRef")]
    pub kms_key_ref: Option<FunctionKmsKeyRef>,
    /// A list of function layers (https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html) to add to the function's execution environment. Specify each layer by its ARN, including the version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// The amount of memory available to the function (https://docs.aws.amazon.com/lambda/latest/dg/configuration-function-common.html#configuration-memory-console) at runtime. Increasing the function memory also increases its CPU allocation. The default value is 128 MB. The value can be any multiple of 1 MB.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySize")]
    pub memory_size: Option<i64>,
    /// The name of the Lambda function. 
    ///  Name formats 
    ///  * Function name – my-function. 
    ///  * Function ARN – arn:aws:lambda:us-west-2:123456789012:function:my-function. 
    ///  * Partial ARN – 123456789012:function:my-function. 
    ///  The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.
    pub name: String,
    /// The type of deployment package. Set to Image for container image and set to Zip for .zip file archive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packageType")]
    pub package_type: Option<String>,
    /// Set to true to publish the first version of the function during creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// The number of simultaneous executions to reserve for the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservedConcurrentExecutions")]
    pub reserved_concurrent_executions: Option<i64>,
    /// The Amazon Resource Name (ARN) of the function's execution role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
    ///  from: name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleRef")]
    pub role_ref: Option<FunctionRoleRef>,
    /// The identifier of the function's runtime (https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html). Runtime is required if the deployment package is a .zip file archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// The function's SnapStart (https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html) setting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapStart")]
    pub snap_start: Option<FunctionSnapStart>,
    /// A list of tags (https://docs.aws.amazon.com/lambda/latest/dg/tagging.html) to apply to the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    /// The amount of time (in seconds) that Lambda allows a function to run before stopping it. The default is 3 seconds. The maximum allowed value is 900 seconds. For more information, see Lambda execution environment (https://docs.aws.amazon.com/lambda/latest/dg/runtimes-context.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Set Mode to Active to sample and trace a subset of incoming requests with X-Ray (https://docs.aws.amazon.com/lambda/latest/dg/services-xray.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfig")]
    pub tracing_config: Option<FunctionTracingConfig>,
    /// For network connectivity to Amazon Web Services resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can access resources and the internet only through that VPC. For more information, see Configuring a Lambda function to access resources in a VPC (https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcConfig")]
    pub vpc_config: Option<FunctionVpcConfig>,
}

/// The code for the function.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionCode {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageURI")]
    pub image_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Bucket")]
    pub s3_bucket: Option<String>,
    /// Reference field for S3Bucket
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3BucketRef")]
    pub s3_bucket_ref: Option<FunctionCodeS3BucketRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Key")]
    pub s3_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zipFile")]
    pub zip_file: Option<String>,
}

/// Reference field for S3Bucket
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionCodeS3BucketRef {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionCodeS3BucketRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionCodeS3BucketRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A dead-letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see Dead-letter queues (https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-dlq).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionDeadLetterConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetARN")]
    pub target_arn: Option<String>,
}

/// Environment variables that are accessible from function code during execution.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionEnvironment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, String>>,
}

/// The size of the function's /tmp directory in MB. The default value is 512, but can be any whole number between 512 and 10,240 MB.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionEphemeralStorage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Details about the connection between a Lambda function and an Amazon EFS file system (https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionFileSystemConfigs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localMountPath")]
    pub local_mount_path: Option<String>,
}

/// Configures options for asynchronous invocation on a function. 
///  - DestinationConfig A destination for events after they have been sent to a function for processing. 
///  Types of Destinations: Function - The Amazon Resource Name (ARN) of a Lambda function. Queue - The ARN of a standard SQS queue. Topic - The ARN of a standard SNS topic. Event Bus - The ARN of an Amazon EventBridge event bus. 
///  - MaximumEventAgeInSeconds The maximum age of a request that Lambda sends to a function for processing. 
///  - MaximumRetryAttempts The maximum number of times to retry when the function returns an error.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionFunctionEventInvokeConfig {
    /// A configuration object that specifies the destination of an event after Lambda processes it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationConfig")]
    pub destination_config: Option<FunctionFunctionEventInvokeConfigDestinationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionName")]
    pub function_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumEventAgeInSeconds")]
    pub maximum_event_age_in_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumRetryAttempts")]
    pub maximum_retry_attempts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// A configuration object that specifies the destination of an event after Lambda processes it.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionFunctionEventInvokeConfigDestinationConfig {
    /// A destination for events that failed processing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onFailure")]
    pub on_failure: Option<FunctionFunctionEventInvokeConfigDestinationConfigOnFailure>,
    /// A destination for events that were processed successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onSuccess")]
    pub on_success: Option<FunctionFunctionEventInvokeConfigDestinationConfigOnSuccess>,
}

/// A destination for events that failed processing.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionFunctionEventInvokeConfigDestinationConfigOnFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// A destination for events that were processed successfully.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionFunctionEventInvokeConfigDestinationConfigOnSuccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// Container image configuration values (https://docs.aws.amazon.com/lambda/latest/dg/configuration-images.html#configuration-images-settings) that override the values in the container image Dockerfile.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPoint")]
    pub entry_point: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workingDirectory")]
    pub working_directory: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionKmsKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionKmsKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionKmsKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionRoleRef {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionRoleRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionRoleRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The function's SnapStart (https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html) setting.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionSnapStart {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyOn")]
    pub apply_on: Option<String>,
}

/// Set Mode to Active to sample and trace a subset of incoming requests with X-Ray (https://docs.aws.amazon.com/lambda/latest/dg/services-xray.html).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionTracingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// For network connectivity to Amazon Web Services resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can access resources and the internet only through that VPC. For more information, see Configuring a Lambda function to access resources in a VPC (https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionVpcConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    /// Reference field for SecurityGroupIDs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupRefs")]
    pub security_group_refs: Option<Vec<FunctionVpcConfigSecurityGroupRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    /// Reference field for SubnetIDs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRefs")]
    pub subnet_refs: Option<Vec<FunctionVpcConfigSubnetRefs>>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionVpcConfigSecurityGroupRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionVpcConfigSecurityGroupRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionVpcConfigSecurityGroupRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionVpcConfigSubnetRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionVpcConfigSubnetRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionVpcConfigSubnetRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// FunctionStatus defines the observed state of Function
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FunctionStatusAckResourceMetadata>,
    /// The SHA256 hash of the function's deployment package.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSHA256")]
    pub code_sha256: Option<String>,
    /// The size of the function's deployment package, in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSize")]
    pub code_size: Option<i64>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FunctionStatusConditions>>,
    /// The function's image configuration values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfigResponse")]
    pub image_config_response: Option<FunctionStatusImageConfigResponse>,
    /// The date and time that the function was last updated, in ISO-8601 format (https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModified")]
    pub last_modified: Option<String>,
    /// The status of the last update that was performed on the function. This is first set to Successful after function creation completes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatus")]
    pub last_update_status: Option<String>,
    /// The reason for the last update that was performed on the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatusReason")]
    pub last_update_status_reason: Option<String>,
    /// The reason code for the last update that was performed on the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatusReasonCode")]
    pub last_update_status_reason_code: Option<String>,
    /// The function's layers (https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "layerStatuses")]
    pub layer_statuses: Option<Vec<FunctionStatusLayerStatuses>>,
    /// For Lambda@Edge functions, the ARN of the main function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterARN")]
    pub master_arn: Option<String>,
    /// The latest updated revision of the function or alias.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionID")]
    pub revision_id: Option<String>,
    /// The ARN of the signing job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingJobARN")]
    pub signing_job_arn: Option<String>,
    /// The ARN of the signing profile version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingProfileVersionARN")]
    pub signing_profile_version_arn: Option<String>,
    /// The current state of the function. When the state is Inactive, you can reactivate the function by invoking it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The reason for the function's current state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateReason")]
    pub state_reason: Option<String>,
    /// The reason code for the function's current state. When the code is Creating, you can't invoke or modify the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateReasonCode")]
    pub state_reason_code: Option<String>,
    /// The version of the Lambda function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatusAckResourceMetadata {
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
pub struct FunctionStatusConditions {
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

/// The function's image configuration values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatusImageConfigResponse {
    /// Error response to GetFunctionConfiguration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<FunctionStatusImageConfigResponseError>,
    /// Configuration values that override the container image Dockerfile settings. For more information, see Container image settings (https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-parms).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfig")]
    pub image_config: Option<FunctionStatusImageConfigResponseImageConfig>,
}

/// Error response to GetFunctionConfiguration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatusImageConfigResponseError {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Configuration values that override the container image Dockerfile settings. For more information, see Container image settings (https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-parms).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatusImageConfigResponseImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPoint")]
    pub entry_point: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workingDirectory")]
    pub working_directory: Option<String>,
}

/// An Lambda layer (https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct FunctionStatusLayerStatuses {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSize")]
    pub code_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingJobARN")]
    pub signing_job_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingProfileVersionARN")]
    pub signing_profile_version_arn: Option<String>,
}

