// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/documentdb-controller/documentdb.services.k8s.aws/v1alpha1/dbclusters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// DBClusterSpec defines the desired state of DBCluster.
/// 
/// 
/// Detailed information about a cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "documentdb.services.k8s.aws", version = "v1alpha1", kind = "DBCluster", plural = "dbclusters")]
#[kube(namespaced)]
#[kube(status = "DBClusterStatus")]
#[kube(schema = "disabled")]
pub struct DBClusterSpec {
    /// A list of Amazon EC2 Availability Zones that instances in the cluster can
    /// be created in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// The number of days for which automated backups are retained. You must specify
    /// a minimum value of 1.
    /// 
    /// 
    /// Default: 1
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must be a value from 1 to 35.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupRetentionPeriod")]
    pub backup_retention_period: Option<i64>,
    /// The cluster identifier. This parameter is stored as a lowercase string.
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must contain from 1 to 63 letters, numbers, or hyphens.
    /// 
    /// 
    ///    * The first character must be a letter.
    /// 
    /// 
    ///    * Cannot end with a hyphen or contain two consecutive hyphens.
    /// 
    /// 
    /// Example: my-cluster
    #[serde(rename = "dbClusterIdentifier")]
    pub db_cluster_identifier: String,
    /// The name of the cluster parameter group to associate with this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterParameterGroupName")]
    pub db_cluster_parameter_group_name: Option<String>,
    /// A subnet group to associate with this cluster.
    /// 
    /// 
    /// Constraints: Must match the name of an existing DBSubnetGroup. Must not be
    /// default.
    /// 
    /// 
    /// Example: mySubnetgroup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupName")]
    pub db_subnet_group_name: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupRef")]
    pub db_subnet_group_ref: Option<DBClusterDbSubnetGroupRef>,
    /// Specifies whether this cluster can be deleted. If DeletionProtection is enabled,
    /// the cluster cannot be deleted unless it is modified and DeletionProtection
    /// is disabled. DeletionProtection protects clusters from being accidentally
    /// deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionProtection")]
    pub deletion_protection: Option<bool>,
    /// DestinationRegion is used for presigning the request to a given region.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationRegion")]
    pub destination_region: Option<String>,
    /// A list of log types that need to be enabled for exporting to Amazon CloudWatch
    /// Logs. You can enable audit logs or profiler logs. For more information, see
    /// Auditing Amazon DocumentDB Events (https://docs.aws.amazon.com/documentdb/latest/developerguide/event-auditing.html)
    /// and Profiling Amazon DocumentDB Operations (https://docs.aws.amazon.com/documentdb/latest/developerguide/profiling.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// The name of the database engine to be used for this cluster.
    /// 
    /// 
    /// Valid values: docdb
    pub engine: String,
    /// The version number of the database engine to use. The --engine-version will
    /// default to the latest major engine version. For production workloads, we
    /// recommend explicitly declaring this parameter with the intended major engine
    /// version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    /// The cluster identifier of the new global cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalClusterIdentifier")]
    pub global_cluster_identifier: Option<String>,
    /// The KMS key identifier for an encrypted cluster.
    /// 
    /// 
    /// The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption
    /// key. If you are creating a cluster using the same Amazon Web Services account
    /// that owns the KMS encryption key that is used to encrypt the new cluster,
    /// you can use the KMS key alias instead of the ARN for the KMS encryption key.
    /// 
    /// 
    /// If an encryption key is not specified in KmsKeyId:
    /// 
    /// 
    ///    * If the StorageEncrypted parameter is true, Amazon DocumentDB uses your
    ///    default encryption key.
    /// 
    /// 
    /// KMS creates the default encryption key for your Amazon Web Services account.
    /// Your Amazon Web Services account has a different default encryption key for
    /// each Amazon Web Services Regions.
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
    pub kms_key_ref: Option<DBClusterKmsKeyRef>,
    /// The password for the master database user. This password can contain any
    /// printable ASCII character except forward slash (/), double quote ("), or
    /// the "at" symbol (@).
    /// 
    /// 
    /// Constraints: Must contain from 8 to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserPassword")]
    pub master_user_password: Option<DBClusterMasterUserPassword>,
    /// The name of the master user for the cluster.
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must be from 1 to 63 letters or numbers.
    /// 
    /// 
    ///    * The first character must be a letter.
    /// 
    /// 
    ///    * Cannot be a reserved word for the chosen database engine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUsername")]
    pub master_username: Option<String>,
    /// The port number on which the instances in the cluster accept connections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Not currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preSignedURL")]
    pub pre_signed_url: Option<String>,
    /// The daily time range during which automated backups are created if automated
    /// backups are enabled using the BackupRetentionPeriod parameter.
    /// 
    /// 
    /// The default is a 30-minute window selected at random from an 8-hour block
    /// of time for each Amazon Web Services Region.
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must be in the format hh24:mi-hh24:mi.
    /// 
    /// 
    ///    * Must be in Universal Coordinated Time (UTC).
    /// 
    /// 
    ///    * Must not conflict with the preferred maintenance window.
    /// 
    /// 
    ///    * Must be at least 30 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// The weekly time range during which system maintenance can occur, in Universal
    /// Coordinated Time (UTC).
    /// 
    /// 
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    /// 
    /// 
    /// The default is a 30-minute window selected at random from an 8-hour block
    /// of time for each Amazon Web Services Region, occurring on a random day of
    /// the week.
    /// 
    /// 
    /// Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun
    /// 
    /// 
    /// Constraints: Minimum 30-minute window.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The identifier for the snapshot or cluster snapshot to restore from.
    /// 
    /// 
    /// You can use either the name or the Amazon Resource Name (ARN) to specify
    /// a cluster snapshot. However, you can use only the ARN to specify a snapshot.
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must match the identifier of an existing snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotIdentifier")]
    pub snapshot_identifier: Option<String>,
    /// SourceRegion is the source region where the resource exists. This is not
    /// sent over the wire and is only used for presigning. This value should always
    /// have the same region as the source ARN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRegion")]
    pub source_region: Option<String>,
    /// Specifies whether the cluster is encrypted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// The storage type to associate with the DB cluster.
    /// 
    /// 
    /// For information on storage types for Amazon DocumentDB clusters, see Cluster
    /// storage configurations in the Amazon DocumentDB Developer Guide.
    /// 
    /// 
    /// Valid values for storage type - standard | iopt1
    /// 
    /// 
    /// Default value is standard
    /// 
    /// 
    /// When you create a DocumentDB DB cluster with the storage type set to iopt1,
    /// the storage type is returned in the response. The storage type isn't returned
    /// when you set it to standard.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<String>,
    /// The tags to be assigned to the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DBClusterTags>>,
    /// A list of EC2 VPC security groups to associate with this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroupIDs")]
    pub vpc_security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroupRefs")]
    pub vpc_security_group_refs: Option<Vec<DBClusterVpcSecurityGroupRefs>>,
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
pub struct DBClusterDbSubnetGroupRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DBClusterDbSubnetGroupRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterDbSubnetGroupRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct DBClusterKmsKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DBClusterKmsKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterKmsKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The password for the master database user. This password can contain any
/// printable ASCII character except forward slash (/), double quote ("), or
/// the "at" symbol (@).
/// 
/// 
/// Constraints: Must contain from 8 to 100 characters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterMasterUserPassword {
    /// Key is the key within the secret
    pub key: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Metadata assigned to an Amazon DocumentDB resource consisting of a key-value
/// pair.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
pub struct DBClusterVpcSecurityGroupRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DBClusterVpcSecurityGroupRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterVpcSecurityGroupRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DBClusterStatus defines the observed state of DBCluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DBClusterStatusAckResourceMetadata>,
    /// Provides a list of the Identity and Access Management (IAM) roles that are
    /// associated with the cluster. (IAM) roles that are associated with a cluster
    /// grant permission for the cluster to access other Amazon Web Services services
    /// on your behalf.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatedRoles")]
    pub associated_roles: Option<Vec<DBClusterStatusAssociatedRoles>>,
    /// Identifies the clone group to which the DB cluster is associated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneGroupID")]
    pub clone_group_id: Option<String>,
    /// Specifies the time when the cluster was created, in Universal Coordinated
    /// Time (UTC).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterCreateTime")]
    pub cluster_create_time: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DBClusterStatusConditions>>,
    /// Provides the list of instances that make up the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterMembers")]
    pub db_cluster_members: Option<Vec<DBClusterStatusDbClusterMembers>>,
    /// Specifies the name of the cluster parameter group for the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterParameterGroup")]
    pub db_cluster_parameter_group: Option<String>,
    /// The Amazon Web Services Region-unique, immutable identifier for the cluster.
    /// This identifier is found in CloudTrail log entries whenever the KMS key for
    /// the cluster is accessed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterResourceID")]
    pub db_cluster_resource_id: Option<String>,
    /// Specifies information on the subnet group that is associated with the cluster,
    /// including the name, description, and subnets in the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroup")]
    pub db_subnet_group: Option<String>,
    /// The earliest time to which a database can be restored with point-in-time
    /// restore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "earliestRestorableTime")]
    pub earliest_restorable_time: Option<String>,
    /// A list of log types that this cluster is configured to export to Amazon CloudWatch
    /// Logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enabledCloudwatchLogsExports")]
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// Specifies the connection endpoint for the primary instance of the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostedZoneID")]
    pub hosted_zone_id: Option<String>,
    /// Specifies the latest time to which a database can be restored with point-in-time
    /// restore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    /// Specifies whether the cluster has instances in multiple Availability Zones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiAZ")]
    pub multi_az: Option<bool>,
    /// Specifies the progress of the operation as a percentage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "percentProgress")]
    pub percent_progress: Option<String>,
    /// Contains one or more identifiers of the secondary clusters that are associated
    /// with this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readReplicaIdentifiers")]
    pub read_replica_identifiers: Option<Vec<String>>,
    /// The reader endpoint for the cluster. The reader endpoint for a cluster load
    /// balances connections across the Amazon DocumentDB replicas that are available
    /// in a cluster. As clients request new connections to the reader endpoint,
    /// Amazon DocumentDB distributes the connection requests among the Amazon DocumentDB
    /// replicas in the cluster. This functionality can help balance your read workload
    /// across multiple Amazon DocumentDB replicas in your cluster.
    /// 
    /// 
    /// If a failover occurs, and the Amazon DocumentDB replica that you are connected
    /// to is promoted to be the primary instance, your connection is dropped. To
    /// continue sending your read workload to other Amazon DocumentDB replicas in
    /// the cluster, you can then reconnect to the reader endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readerEndpoint")]
    pub reader_endpoint: Option<String>,
    /// Contains the identifier of the source cluster if this cluster is a secondary
    /// cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationSourceIdentifier")]
    pub replication_source_identifier: Option<String>,
    /// Specifies the current state of this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Provides a list of virtual private cloud (VPC) security groups that the cluster
    /// belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroups")]
    pub vpc_security_groups: Option<Vec<DBClusterStatusVpcSecurityGroups>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatusAckResourceMetadata {
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

/// Describes an Identity and Access Management (IAM) role that is associated
/// with a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatusAssociatedRoles {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatusConditions {
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

/// Contains information about an instance that is part of a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatusDbClusterMembers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterParameterGroupStatus")]
    pub db_cluster_parameter_group_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbInstanceIdentifier")]
    pub db_instance_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isClusterWriter")]
    pub is_cluster_writer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "promotionTier")]
    pub promotion_tier: Option<i64>,
}

/// Used as a response element for queries on virtual private cloud (VPC) security
/// group membership.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBClusterStatusVpcSecurityGroups {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroupID")]
    pub vpc_security_group_id: Option<String>,
}

