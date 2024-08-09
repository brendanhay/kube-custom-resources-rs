// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/efs-controller/efs.services.k8s.aws/v1alpha1/filesystems.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FileSystemSpec defines the desired state of FileSystem.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "efs.services.k8s.aws", version = "v1alpha1", kind = "FileSystem", plural = "filesystems")]
#[kube(namespaced)]
#[kube(status = "FileSystemStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FileSystemSpec {
    /// Used to create a One Zone file system. It specifies the Amazon Web Services
    /// Availability Zone in which to create the file system. Use the format us-east-1a
    /// to specify the Availability Zone. For more information about One Zone file
    /// systems, see Using EFS storage classes (https://docs.aws.amazon.com/efs/latest/ug/storage-classes.html)
    /// in the Amazon EFS User Guide.
    /// 
    /// 
    /// One Zone file systems are not available in all Availability Zones in Amazon
    /// Web Services Regions where Amazon EFS is available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZoneName")]
    pub availability_zone_name: Option<String>,
    /// Specifies whether automatic backups are enabled on the file system that you
    /// are creating. Set the value to true to enable automatic backups. If you are
    /// creating a One Zone file system, automatic backups are enabled by default.
    /// For more information, see Automatic backups (https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups)
    /// in the Amazon EFS User Guide.
    /// 
    /// 
    /// Default is false. However, if you specify an AvailabilityZoneName, the default
    /// is true.
    /// 
    /// 
    /// Backup is not available in all Amazon Web Services Regions where Amazon EFS
    /// is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<bool>,
    /// The backup policy included in the PutBackupPolicy request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPolicy")]
    pub backup_policy: Option<FileSystemBackupPolicy>,
    /// A Boolean value that, if true, creates an encrypted file system. When creating
    /// an encrypted file system, you have the option of specifying an existing Key
    /// Management Service key (KMS key). If you don't specify a KMS key, then the
    /// default KMS key for Amazon EFS, /aws/elasticfilesystem, is used to protect
    /// the encrypted file system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemProtection")]
    pub file_system_protection: Option<FileSystemFileSystemProtection>,
    /// The ID of the KMS key that you want to use to protect the encrypted file
    /// system. This parameter is required only if you want to use a non-default
    /// KMS key. If this parameter is not specified, the default KMS key for Amazon
    /// EFS is used. You can specify a KMS key ID using the following formats:
    /// 
    /// 
    ///    * Key ID - A unique identifier of the key, for example 1234abcd-12ab-34cd-56ef-1234567890ab.
    /// 
    /// 
    ///    * ARN - An Amazon Resource Name (ARN) for the key, for example arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab.
    /// 
    /// 
    ///    * Key alias - A previously created display name for a key, for example
    ///    alias/projectKey1.
    /// 
    /// 
    ///    * Key alias ARN - An ARN for a key alias, for example arn:aws:kms:us-west-2:444455556666:alias/projectKey1.
    /// 
    /// 
    /// If you use KmsKeyId, you must set the CreateFileSystemRequest$Encrypted parameter
    /// to true.
    /// 
    /// 
    /// EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with
    /// Amazon EFS file systems.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyRef")]
    pub kms_key_ref: Option<FileSystemKmsKeyRef>,
    /// An array of LifecyclePolicy objects that define the file system's LifecycleConfiguration
    /// object. A LifecycleConfiguration object informs EFS Lifecycle management
    /// of the following:
    /// 
    /// 
    ///    * TransitionToIA – When to move files in the file system from primary
    ///    storage (Standard storage class) into the Infrequent Access (IA) storage.
    /// 
    /// 
    ///    * TransitionToArchive – When to move files in the file system from their
    ///    current storage class (either IA or Standard storage) into the Archive
    ///    storage. File systems cannot transition into Archive storage before transitioning
    ///    into IA storage. Therefore, TransitionToArchive must either not be set
    ///    or must be later than TransitionToIA. The Archive storage class is available
    ///    only for file systems that use the Elastic Throughput mode and the General
    ///    Purpose Performance mode.
    /// 
    /// 
    ///    * TransitionToPrimaryStorageClass – Whether to move files in the file
    ///    system back to primary storage (Standard storage class) after they are
    ///    accessed in IA or Archive storage.
    /// 
    /// 
    /// When using the put-lifecycle-configuration CLI command or the PutLifecycleConfiguration
    /// API action, Amazon EFS requires that each LifecyclePolicy object have only
    /// a single transition. This means that in a request body, LifecyclePolicies
    /// must be structured as an array of LifecyclePolicy objects, one object for
    /// each storage transition. See the example requests in the following section
    /// for more information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecyclePolicies")]
    pub lifecycle_policies: Option<Vec<FileSystemLifecyclePolicies>>,
    /// The Performance mode of the file system. We recommend generalPurpose performance
    /// mode for all file systems. File systems using the maxIO performance mode
    /// can scale to higher levels of aggregate throughput and operations per second
    /// with a tradeoff of slightly higher latencies for most file operations. The
    /// performance mode can't be changed after the file system has been created.
    /// The maxIO mode is not supported on One Zone file systems.
    /// 
    /// 
    /// Due to the higher per-operation latencies with Max I/O, we recommend using
    /// General Purpose performance mode for all file systems.
    /// 
    /// 
    /// Default is generalPurpose.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "performanceMode")]
    pub performance_mode: Option<String>,
    /// The FileSystemPolicy that you're creating. Accepts a JSON formatted policy
    /// definition. EFS file system policies have a 20,000 character limit. To find
    /// out more about the elements that make up a file system policy, see EFS Resource-based
    /// Policies (https://docs.aws.amazon.com/efs/latest/ug/access-control-overview.html#access-control-manage-access-intro-resource-policies).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// The throughput, measured in mebibytes per second (MiBps), that you want to
    /// provision for a file system that you're creating. Required if ThroughputMode
    /// is set to provisioned. Valid values are 1-3414 MiBps, with the upper limit
    /// depending on Region. To increase this limit, contact Amazon Web Services
    /// Support. For more information, see Amazon EFS quotas that you can increase
    /// (https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits) in the
    /// Amazon EFS User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughputInMiBps")]
    pub provisioned_throughput_in_mi_bps: Option<f64>,
    /// Use to create one or more tags associated with the file system. Each tag
    /// is a user-defined key-value pair. Name your file system on creation by including
    /// a "Key":"Name","Value":"{value}" key-value pair. Each key must be unique.
    /// For more information, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html)
    /// in the Amazon Web Services General Reference Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FileSystemTags>>,
    /// Specifies the throughput mode for the file system. The mode can be bursting,
    /// provisioned, or elastic. If you set ThroughputMode to provisioned, you must
    /// also set a value for ProvisionedThroughputInMibps. After you create the file
    /// system, you can decrease your file system's Provisioned throughput or change
    /// between the throughput modes, with certain time restrictions. For more information,
    /// see Specifying throughput with provisioned mode (https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput)
    /// in the Amazon EFS User Guide.
    /// 
    /// 
    /// Default is bursting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throughputMode")]
    pub throughput_mode: Option<String>,
}

/// The backup policy included in the PutBackupPolicy request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemBackupPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemFileSystemProtection {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationOverwriteProtection")]
    pub replication_overwrite_protection: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemKmsKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FileSystemKmsKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemKmsKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Describes a policy used by Lifecycle management that specifies when to transition
/// files into and out of storage classes. For more information, see Managing
/// file system storage (https://docs.aws.amazon.com/efs/latest/ug/lifecycle-management-efs.html).
/// 
/// 
/// When using the put-lifecycle-configuration CLI command or the PutLifecycleConfiguration
/// API action, Amazon EFS requires that each LifecyclePolicy object have only
/// a single transition. This means that in a request body, LifecyclePolicies
/// must be structured as an array of LifecyclePolicy objects, one object for
/// each transition. For more information, see the request examples in PutLifecycleConfiguration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemLifecyclePolicies {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitionToArchive")]
    pub transition_to_archive: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitionToIA")]
    pub transition_to_ia: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitionToPrimaryStorageClass")]
    pub transition_to_primary_storage_class: Option<String>,
}

/// A tag is a key-value pair. Allowed characters are letters, white space, and
/// numbers that can be represented in UTF-8, and the following characters:+
/// - = . _ : /.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// FileSystemStatus defines the observed state of FileSystem
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FileSystemStatusAckResourceMetadata>,
    /// The unique and consistent identifier of the Availability Zone in which the
    /// file system is located, and is valid only for One Zone file systems. For
    /// example, use1-az1 is an Availability Zone ID for the us-east-1 Amazon Web
    /// Services Region, and it has the same location in every Amazon Web Services
    /// account.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZoneID")]
    pub availability_zone_id: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The time that the file system was created, in seconds (since 1970-01-01T00:00:00Z).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The ID of the file system, assigned by Amazon EFS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemID")]
    pub file_system_id: Option<String>,
    /// The lifecycle phase of the file system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifeCycleState")]
    pub life_cycle_state: Option<String>,
    /// You can add tags to a file system, including a Name tag. For more information,
    /// see CreateFileSystem. If the file system has a Name tag, Amazon EFS returns
    /// the value in this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The current number of mount targets that the file system has. For more information,
    /// see CreateMountTarget.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfMountTargets")]
    pub number_of_mount_targets: Option<i64>,
    /// The Amazon Web Services account that created the file system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The latest known metered size (in bytes) of data stored in the file system,
    /// in its Value field, and the time at which that size was determined in its
    /// Timestamp field. The Timestamp value is the integer number of seconds since
    /// 1970-01-01T00:00:00Z. The SizeInBytes value doesn't represent the size of
    /// a consistent snapshot of the file system, but it is eventually consistent
    /// when there are no writes to the file system. That is, SizeInBytes represents
    /// actual size only if the file system is not modified for a period longer than
    /// a couple of hours. Otherwise, the value is not the exact size that the file
    /// system was at any point in time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeInBytes")]
    pub size_in_bytes: Option<FileSystemStatusSizeInBytes>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// The latest known metered size (in bytes) of data stored in the file system,
/// in its Value field, and the time at which that size was determined in its
/// Timestamp field. The Timestamp value is the integer number of seconds since
/// 1970-01-01T00:00:00Z. The SizeInBytes value doesn't represent the size of
/// a consistent snapshot of the file system, but it is eventually consistent
/// when there are no writes to the file system. That is, SizeInBytes represents
/// actual size only if the file system is not modified for a period longer than
/// a couple of hours. Otherwise, the value is not the exact size that the file
/// system was at any point in time.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileSystemStatusSizeInBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueInArchive")]
    pub value_in_archive: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueInIA")]
    pub value_in_ia: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueInStandard")]
    pub value_in_standard: Option<i64>,
}

