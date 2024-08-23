// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/karpenter-provider-aws/karpenter.k8s.aws/v1/ec2nodeclasses.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// EC2NodeClassSpec is the top level specification for the AWS Karpenter Provider.
/// This will contain configuration necessary to launch instances in AWS.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "karpenter.k8s.aws", version = "v1", kind = "EC2NodeClass", plural = "ec2nodeclasses")]
#[kube(status = "EC2NodeClassStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EC2NodeClassSpec {
    /// AMIFamily dictates the UserData format and default BlockDeviceMappings used when generating launch templates.
    /// This field is optional when using an alias amiSelectorTerm, and the value will be inferred from the alias'
    /// family. When an alias is specified, this field may only be set to its corresponding family or 'Custom'. If no
    /// alias is specified, this field is required.
    /// NOTE: We ignore the AMIFamily for hashing here because we hash the AMIFamily dynamically by using the alias using
    /// the AMIFamily() helper function
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amiFamily")]
    pub ami_family: Option<EC2NodeClassAmiFamily>,
    /// AMISelectorTerms is a list of or ami selector terms. The terms are ORed.
    #[serde(rename = "amiSelectorTerms")]
    pub ami_selector_terms: Vec<EC2NodeClassAmiSelectorTerms>,
    /// AssociatePublicIPAddress controls if public IP addresses are assigned to instances that are launched with the nodeclass.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatePublicIPAddress")]
    pub associate_public_ip_address: Option<bool>,
    /// BlockDeviceMappings to be applied to provisioned nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blockDeviceMappings")]
    pub block_device_mappings: Option<Vec<EC2NodeClassBlockDeviceMappings>>,
    /// Context is a Reserved field in EC2 APIs
    /// https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateFleet.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// DetailedMonitoring controls if detailed monitoring is enabled for instances that are launched
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detailedMonitoring")]
    pub detailed_monitoring: Option<bool>,
    /// InstanceProfile is the AWS entity that instances use.
    /// This field is mutually exclusive from role.
    /// The instance profile should already have a role assigned to it that Karpenter
    ///  has PassRole permission on for instance launch using this instanceProfile to succeed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceProfile")]
    pub instance_profile: Option<String>,
    /// InstanceStorePolicy specifies how to handle instance-store disks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceStorePolicy")]
    pub instance_store_policy: Option<EC2NodeClassInstanceStorePolicy>,
    /// Kubelet defines args to be used when configuring kubelet on provisioned nodes.
    /// They are a subset of the upstream types, recognizing not all options may be supported.
    /// Wherever possible, the types and names should reflect the upstream kubelet types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet: Option<EC2NodeClassKubelet>,
    /// MetadataOptions for the generated launch template of provisioned nodes.
    /// 
    /// This specifies the exposure of the Instance Metadata Service to
    /// provisioned EC2 nodes. For more information,
    /// see Instance Metadata and User Data
    /// (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
    /// in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Refer to recommended, security best practices
    /// (https://aws.github.io/aws-eks-best-practices/security/docs/iam/#restrict-access-to-the-instance-profile-assigned-to-the-worker-node)
    /// for limiting exposure of Instance Metadata and User Data to pods.
    /// If omitted, defaults to httpEndpoint enabled, with httpProtocolIPv6
    /// disabled, with httpPutResponseLimit of 1, and with httpTokens
    /// required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataOptions")]
    pub metadata_options: Option<EC2NodeClassMetadataOptions>,
    /// Role is the AWS identity that nodes use. This field is immutable.
    /// This field is mutually exclusive from instanceProfile.
    /// Marking this field as immutable avoids concerns around terminating managed instance profiles from running instances.
    /// This field may be made mutable in the future, assuming the correct garbage collection and drift handling is implemented
    /// for the old instance profiles on an update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// SecurityGroupSelectorTerms is a list of or security group selector terms. The terms are ORed.
    #[serde(rename = "securityGroupSelectorTerms")]
    pub security_group_selector_terms: Vec<EC2NodeClassSecurityGroupSelectorTerms>,
    /// SubnetSelectorTerms is a list of or subnet selector terms. The terms are ORed.
    #[serde(rename = "subnetSelectorTerms")]
    pub subnet_selector_terms: Vec<EC2NodeClassSubnetSelectorTerms>,
    /// Tags to be applied on ec2 resources like instances and launch templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    /// UserData to be applied to the provisioned nodes.
    /// It must be in the appropriate format based on the AMIFamily in use. Karpenter will merge certain fields into
    /// this UserData to ensure nodes are being provisioned with the correct configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userData")]
    pub user_data: Option<String>,
}

/// EC2NodeClassSpec is the top level specification for the AWS Karpenter Provider.
/// This will contain configuration necessary to launch instances in AWS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassAmiFamily {
    #[serde(rename = "AL2")]
    Al2,
    #[serde(rename = "AL2023")]
    Al2023,
    Bottlerocket,
    Custom,
    Windows2019,
    Windows2022,
}

/// AMISelectorTerm defines selection logic for an ami used by Karpenter to launch nodes.
/// If multiple fields are used for selection, the requirements are ANDed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassAmiSelectorTerms {
    /// Alias specifies which EKS optimized AMI to select.
    /// Each alias consists of a family and an AMI version, specified as "family@version".
    /// Valid families include: al2, al2023, bottlerocket, windows2019, and windows2022.
    /// The version can either be pinned to a specific AMI release, with that AMIs version format (ex: "al2023@v20240625" or "bottlerocket@v1.10.0").
    /// The version can also be set to "latest" for any family. Setting the version to latest will result in drift when a new AMI is released. This is **not** recommended for production environments.
    /// Note: The Windows families do **not** support version pinning, and only latest may be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// ID is the ami id in EC2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name is the ami name in EC2.
    /// This value is the name field, which is different from the name tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner is the owner for the ami.
    /// You can specify a combination of AWS account IDs, "self", "amazon", and "aws-marketplace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Tags is a map of key/value tags used to select subnets
    /// Specifying '*' for a value selects all values for a given tag key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassBlockDeviceMappings {
    /// The device name (for example, /dev/sdh or xvdh).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// EBS contains parameters used to automatically set up EBS volumes when an instance is launched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EC2NodeClassBlockDeviceMappingsEbs>,
    /// RootVolume is a flag indicating if this device is mounted as kubelet root dir. You can
    /// configure at most one root volume in BlockDeviceMappings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootVolume")]
    pub root_volume: Option<bool>,
}

/// EBS contains parameters used to automatically set up EBS volumes when an instance is launched.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassBlockDeviceMappingsEbs {
    /// DeleteOnTermination indicates whether the EBS volume is deleted on instance termination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    /// Encrypted indicates whether the EBS volume is encrypted. Encrypted volumes can only
    /// be attached to instances that support Amazon EBS encryption. If you are creating
    /// a volume from a snapshot, you can't specify an encryption value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// IOPS is the number of I/O operations per second (IOPS). For gp3, io1, and io2 volumes,
    /// this represents the number of IOPS that are provisioned for the volume. For
    /// gp2 volumes, this represents the baseline performance of the volume and the
    /// rate at which the volume accumulates I/O credits for bursting.
    /// 
    /// The following are the supported values for each volume type:
    /// 
    ///    * gp3: 3,000-16,000 IOPS
    /// 
    ///    * io1: 100-64,000 IOPS
    /// 
    ///    * io2: 100-64,000 IOPS
    /// 
    /// For io1 and io2 volumes, we guarantee 64,000 IOPS only for Instances built
    /// on the Nitro System (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances).
    /// Other instance families guarantee performance up to 32,000 IOPS.
    /// 
    /// This parameter is supported for io1, io2, and gp3 volumes only. This parameter
    /// is not supported for gp2, st1, sc1, or standard volumes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// KMSKeyID (ARN) of the symmetric Key Management Service (KMS) CMK used for encryption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// SnapshotID is the ID of an EBS snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotID")]
    pub snapshot_id: Option<String>,
    /// Throughput to provision for a gp3 volume, with a maximum of 1,000 MiB/s.
    /// Valid Range: Minimum value of 125. Maximum value of 1000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    /// VolumeSize in `Gi`, `G`, `Ti`, or `T`. You must specify either a snapshot ID or
    /// a volume size. The following are the supported volumes sizes for each volume
    /// type:
    /// 
    ///    * gp2 and gp3: 1-16,384
    /// 
    ///    * io1 and io2: 4-16,384
    /// 
    ///    * st1 and sc1: 125-16,384
    /// 
    ///    * standard: 1-1,024
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<String>,
    /// VolumeType of the block device.
    /// For more information, see Amazon EBS volume types (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html)
    /// in the Amazon Elastic Compute Cloud User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeType")]
    pub volume_type: Option<EC2NodeClassBlockDeviceMappingsEbsVolumeType>,
}

/// EBS contains parameters used to automatically set up EBS volumes when an instance is launched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassBlockDeviceMappingsEbsVolumeType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "io1")]
    Io1,
    #[serde(rename = "io2")]
    Io2,
    #[serde(rename = "gp2")]
    Gp2,
    #[serde(rename = "sc1")]
    Sc1,
    #[serde(rename = "st1")]
    St1,
    #[serde(rename = "gp3")]
    Gp3,
}

/// EC2NodeClassSpec is the top level specification for the AWS Karpenter Provider.
/// This will contain configuration necessary to launch instances in AWS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassInstanceStorePolicy {
    #[serde(rename = "RAID0")]
    Raid0,
}

/// Kubelet defines args to be used when configuring kubelet on provisioned nodes.
/// They are a subset of the upstream types, recognizing not all options may be supported.
/// Wherever possible, the types and names should reflect the upstream kubelet types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassKubelet {
    /// clusterDNS is a list of IP addresses for the cluster DNS server.
    /// Note that not all providers may use all addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDNS")]
    pub cluster_dns: Option<Vec<String>>,
    /// CPUCFSQuota enables CPU CFS quota enforcement for containers that specify CPU limits.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuCFSQuota")]
    pub cpu_cfs_quota: Option<bool>,
    /// EvictionHard is the map of signal names to quantities that define hard eviction thresholds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionHard")]
    pub eviction_hard: Option<BTreeMap<String, String>>,
    /// EvictionMaxPodGracePeriod is the maximum allowed grace period (in seconds) to use when terminating pods in
    /// response to soft eviction thresholds being met.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionMaxPodGracePeriod")]
    pub eviction_max_pod_grace_period: Option<i32>,
    /// EvictionSoft is the map of signal names to quantities that define soft eviction thresholds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoft")]
    pub eviction_soft: Option<BTreeMap<String, String>>,
    /// EvictionSoftGracePeriod is the map of signal names to quantities that define grace periods for each eviction signal
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoftGracePeriod")]
    pub eviction_soft_grace_period: Option<BTreeMap<String, String>>,
    /// ImageGCHighThresholdPercent is the percent of disk usage after which image
    /// garbage collection is always run. The percent is calculated by dividing this
    /// field value by 100, so this field must be between 0 and 100, inclusive.
    /// When specified, the value must be greater than ImageGCLowThresholdPercent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCHighThresholdPercent")]
    pub image_gc_high_threshold_percent: Option<i32>,
    /// ImageGCLowThresholdPercent is the percent of disk usage before which image
    /// garbage collection is never run. Lowest disk usage to garbage collect to.
    /// The percent is calculated by dividing this field value by 100,
    /// so the field value must be between 0 and 100, inclusive.
    /// When specified, the value must be less than imageGCHighThresholdPercent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCLowThresholdPercent")]
    pub image_gc_low_threshold_percent: Option<i32>,
    /// KubeReserved contains resources reserved for Kubernetes system components.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeReserved")]
    pub kube_reserved: Option<BTreeMap<String, String>>,
    /// MaxPods is an override for the maximum number of pods that can run on
    /// a worker node instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPods")]
    pub max_pods: Option<i32>,
    /// PodsPerCore is an override for the number of pods that can run on a worker node
    /// instance based on the number of cpu cores. This value cannot exceed MaxPods, so, if
    /// MaxPods is a lower value, that value will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podsPerCore")]
    pub pods_per_core: Option<i32>,
    /// SystemReserved contains resources reserved for OS system daemons and kernel memory.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemReserved")]
    pub system_reserved: Option<BTreeMap<String, String>>,
}

/// MetadataOptions for the generated launch template of provisioned nodes.
/// 
/// This specifies the exposure of the Instance Metadata Service to
/// provisioned EC2 nodes. For more information,
/// see Instance Metadata and User Data
/// (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
/// in the Amazon Elastic Compute Cloud User Guide.
/// 
/// Refer to recommended, security best practices
/// (https://aws.github.io/aws-eks-best-practices/security/docs/iam/#restrict-access-to-the-instance-profile-assigned-to-the-worker-node)
/// for limiting exposure of Instance Metadata and User Data to pods.
/// If omitted, defaults to httpEndpoint enabled, with httpProtocolIPv6
/// disabled, with httpPutResponseLimit of 1, and with httpTokens
/// required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassMetadataOptions {
    /// HTTPEndpoint enables or disables the HTTP metadata endpoint on provisioned
    /// nodes. If metadata options is non-nil, but this parameter is not specified,
    /// the default state is "enabled".
    /// 
    /// If you specify a value of "disabled", instance metadata will not be accessible
    /// on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpEndpoint")]
    pub http_endpoint: Option<EC2NodeClassMetadataOptionsHttpEndpoint>,
    /// HTTPProtocolIPv6 enables or disables the IPv6 endpoint for the instance metadata
    /// service on provisioned nodes. If metadata options is non-nil, but this parameter
    /// is not specified, the default state is "disabled".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpProtocolIPv6")]
    pub http_protocol_i_pv6: Option<EC2NodeClassMetadataOptionsHttpProtocolIPv6>,
    /// HTTPPutResponseHopLimit is the desired HTTP PUT response hop limit for
    /// instance metadata requests. The larger the number, the further instance
    /// metadata requests can travel. Possible values are integers from 1 to 64.
    /// If metadata options is non-nil, but this parameter is not specified, the
    /// default value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<i64>,
    /// HTTPTokens determines the state of token usage for instance metadata
    /// requests. If metadata options is non-nil, but this parameter is not
    /// specified, the default state is "required".
    /// 
    /// If the state is optional, one can choose to retrieve instance metadata with
    /// or without a signed token header on the request. If one retrieves the IAM
    /// role credentials without a token, the version 1.0 role credentials are
    /// returned. If one retrieves the IAM role credentials using a valid signed
    /// token, the version 2.0 role credentials are returned.
    /// 
    /// If the state is "required", one must send a signed token header with any
    /// instance metadata retrieval requests. In this state, retrieving the IAM
    /// role credentials always returns the version 2.0 credentials; the version
    /// 1.0 credentials are not available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpTokens")]
    pub http_tokens: Option<EC2NodeClassMetadataOptionsHttpTokens>,
}

/// MetadataOptions for the generated launch template of provisioned nodes.
/// 
/// This specifies the exposure of the Instance Metadata Service to
/// provisioned EC2 nodes. For more information,
/// see Instance Metadata and User Data
/// (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
/// in the Amazon Elastic Compute Cloud User Guide.
/// 
/// Refer to recommended, security best practices
/// (https://aws.github.io/aws-eks-best-practices/security/docs/iam/#restrict-access-to-the-instance-profile-assigned-to-the-worker-node)
/// for limiting exposure of Instance Metadata and User Data to pods.
/// If omitted, defaults to httpEndpoint enabled, with httpProtocolIPv6
/// disabled, with httpPutResponseLimit of 1, and with httpTokens
/// required.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassMetadataOptionsHttpEndpoint {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// MetadataOptions for the generated launch template of provisioned nodes.
/// 
/// This specifies the exposure of the Instance Metadata Service to
/// provisioned EC2 nodes. For more information,
/// see Instance Metadata and User Data
/// (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
/// in the Amazon Elastic Compute Cloud User Guide.
/// 
/// Refer to recommended, security best practices
/// (https://aws.github.io/aws-eks-best-practices/security/docs/iam/#restrict-access-to-the-instance-profile-assigned-to-the-worker-node)
/// for limiting exposure of Instance Metadata and User Data to pods.
/// If omitted, defaults to httpEndpoint enabled, with httpProtocolIPv6
/// disabled, with httpPutResponseLimit of 1, and with httpTokens
/// required.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassMetadataOptionsHttpProtocolIPv6 {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// MetadataOptions for the generated launch template of provisioned nodes.
/// 
/// This specifies the exposure of the Instance Metadata Service to
/// provisioned EC2 nodes. For more information,
/// see Instance Metadata and User Data
/// (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
/// in the Amazon Elastic Compute Cloud User Guide.
/// 
/// Refer to recommended, security best practices
/// (https://aws.github.io/aws-eks-best-practices/security/docs/iam/#restrict-access-to-the-instance-profile-assigned-to-the-worker-node)
/// for limiting exposure of Instance Metadata and User Data to pods.
/// If omitted, defaults to httpEndpoint enabled, with httpProtocolIPv6
/// disabled, with httpPutResponseLimit of 1, and with httpTokens
/// required.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EC2NodeClassMetadataOptionsHttpTokens {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
}

/// SecurityGroupSelectorTerm defines selection logic for a security group used by Karpenter to launch nodes.
/// If multiple fields are used for selection, the requirements are ANDed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassSecurityGroupSelectorTerms {
    /// ID is the security group id in EC2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name is the security group name in EC2.
    /// This value is the name field, which is different from the name tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags is a map of key/value tags used to select subnets
    /// Specifying '*' for a value selects all values for a given tag key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// SubnetSelectorTerm defines selection logic for a subnet used by Karpenter to launch nodes.
/// If multiple fields are used for selection, the requirements are ANDed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassSubnetSelectorTerms {
    /// ID is the subnet id in EC2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tags is a map of key/value tags used to select subnets
    /// Specifying '*' for a value selects all values for a given tag key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// EC2NodeClassStatus contains the resolved state of the EC2NodeClass
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassStatus {
    /// AMI contains the current AMI values that are available to the
    /// cluster under the AMI selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amis: Option<Vec<EC2NodeClassStatusAmis>>,
    /// Conditions contains signals for health and readiness
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// InstanceProfile contains the resolved instance profile for the role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceProfile")]
    pub instance_profile: Option<String>,
    /// SecurityGroups contains the current Security Groups values that are available to the
    /// cluster under the SecurityGroups selectors.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<Vec<EC2NodeClassStatusSecurityGroups>>,
    /// Subnets contains the current Subnet values that are available to the
    /// cluster under the subnet selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<EC2NodeClassStatusSubnets>>,
}

/// AMI contains resolved AMI selector values utilized for node launch
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassStatusAmis {
    /// ID of the AMI
    pub id: String,
    /// Name of the AMI
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Requirements of the AMI to be utilized on an instance type
    pub requirements: Vec<EC2NodeClassStatusAmisRequirements>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassStatusAmisRequirements {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// SecurityGroup contains resolved SecurityGroup selector values utilized for node launch
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassStatusSecurityGroups {
    /// ID of the security group
    pub id: String,
    /// Name of the security group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Subnet contains resolved Subnet selector values utilized for node launch
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EC2NodeClassStatusSubnets {
    /// ID of the subnet
    pub id: String,
    /// The associated availability zone
    pub zone: String,
    /// The associated availability zone ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneID")]
    pub zone_id: Option<String>,
}

