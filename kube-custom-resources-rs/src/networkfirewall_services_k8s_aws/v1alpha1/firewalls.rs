// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/networkfirewall-controller/networkfirewall.services.k8s.aws/v1alpha1/firewalls.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// FirewallSpec defines the desired state of Firewall.
/// 
/// 
/// The firewall defines the configuration settings for an Network Firewall firewall.
/// These settings include the firewall policy, the subnets in your VPC to use
/// for the firewall endpoints, and any tags that are attached to the firewall
/// Amazon Web Services resource.
/// 
/// 
/// The status of the firewall, for example whether it's ready to filter network
/// traffic, is provided in the corresponding FirewallStatus. You can retrieve
/// both objects by calling DescribeFirewall.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networkfirewall.services.k8s.aws", version = "v1alpha1", kind = "Firewall", plural = "firewalls")]
#[kube(namespaced)]
#[kube(status = "FirewallStatus")]
#[kube(schema = "disabled")]
pub struct FirewallSpec {
    /// A flag indicating whether it is possible to delete the firewall. A setting
    /// of TRUE indicates that the firewall is protected against deletion. Use this
    /// setting to protect against accidentally deleting a firewall that is in use.
    /// When you create a firewall, the operation initializes this flag to TRUE.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteProtection")]
    pub delete_protection: Option<bool>,
    /// A description of the firewall.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A complex type that contains settings for encryption of your firewall resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<FirewallEncryptionConfiguration>,
    /// The descriptive name of the firewall. You can't change the name of a firewall
    /// after you create it.
    #[serde(rename = "firewallName")]
    pub firewall_name: String,
    /// The Amazon Resource Name (ARN) of the FirewallPolicy that you want to use
    /// for the firewall.
    #[serde(rename = "firewallPolicyARN")]
    pub firewall_policy_arn: String,
    /// A setting indicating whether the firewall is protected against a change to
    /// the firewall policy association. Use this setting to protect against accidentally
    /// modifying the firewall policy for a firewall that is in use. When you create
    /// a firewall, the operation initializes this setting to TRUE.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyChangeProtection")]
    pub firewall_policy_change_protection: Option<bool>,
    /// A setting indicating whether the firewall is protected against changes to
    /// the subnet associations. Use this setting to protect against accidentally
    /// modifying the subnet associations for a firewall that is in use. When you
    /// create a firewall, the operation initializes this setting to TRUE.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetChangeProtection")]
    pub subnet_change_protection: Option<bool>,
    /// The public subnets to use for your Network Firewall firewalls. Each subnet
    /// must belong to a different Availability Zone in the VPC. Network Firewall
    /// creates a firewall endpoint in each subnet.
    #[serde(rename = "subnetMappings")]
    pub subnet_mappings: Vec<FirewallSubnetMappings>,
    /// The key:value pairs to associate with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FirewallTags>>,
    /// The unique identifier of the VPC where Network Firewall should create the
    /// firewall.
    /// 
    /// 
    /// You can't change this setting after you create the firewall.
    #[serde(rename = "vpcID")]
    pub vpc_id: String,
}

/// A complex type that contains settings for encryption of your firewall resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// The ID for a subnet that you want to associate with the firewall. This is
/// used with CreateFirewall and AssociateSubnets. Network Firewall creates an
/// instance of the associated firewall in each subnet that you specify, to filter
/// traffic in the subnet's Availability Zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallSubnetMappings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iPAddressType")]
    pub i_p_address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// FirewallStatus defines the observed state of Firewall
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FirewallStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FirewallStatusConditions>>,
    /// The configuration settings for the firewall. These settings include the firewall
    /// policy and the subnets in your VPC to use for the firewall endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firewall: Option<FirewallStatusFirewall>,
    /// Detailed information about the current status of a Firewall. You can retrieve
    /// this for a firewall by calling DescribeFirewall and providing the firewall
    /// name and ARN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallStatus")]
    pub firewall_status: Option<FirewallStatusFirewallStatus>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusAckResourceMetadata {
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

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusConditions {
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

/// The configuration settings for the firewall. These settings include the firewall
/// policy and the subnets in your VPC to use for the firewall endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewall {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteProtection")]
    pub delete_protection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A complex type that contains optional Amazon Web Services Key Management
    /// Service (KMS) encryption settings for your Network Firewall resources. Your
    /// data is encrypted by default with an Amazon Web Services owned key that Amazon
    /// Web Services owns and manages for you. You can use either the Amazon Web
    /// Services owned key, or provide your own customer managed key. To learn more
    /// about KMS encryption of your Network Firewall resources, see Encryption at
    /// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
    /// in the Network Firewall Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<FirewallStatusFirewallEncryptionConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallARN")]
    pub firewall_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallID")]
    pub firewall_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallName")]
    pub firewall_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyARN")]
    pub firewall_policy_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyChangeProtection")]
    pub firewall_policy_change_protection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetChangeProtection")]
    pub subnet_change_protection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetMappings")]
    pub subnet_mappings: Option<Vec<FirewallStatusFirewallSubnetMappings>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FirewallStatusFirewallTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// A complex type that contains optional Amazon Web Services Key Management
/// Service (KMS) encryption settings for your Network Firewall resources. Your
/// data is encrypted by default with an Amazon Web Services owned key that Amazon
/// Web Services owns and manages for you. You can use either the Amazon Web
/// Services owned key, or provide your own customer managed key. To learn more
/// about KMS encryption of your Network Firewall resources, see Encryption at
/// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
/// in the Network Firewall Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// The ID for a subnet that you want to associate with the firewall. This is
/// used with CreateFirewall and AssociateSubnets. Network Firewall creates an
/// instance of the associated firewall in each subnet that you specify, to filter
/// traffic in the subnet's Availability Zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallSubnetMappings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iPAddressType")]
    pub i_p_address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Detailed information about the current status of a Firewall. You can retrieve
/// this for a firewall by calling DescribeFirewall and providing the firewall
/// name and ARN.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatus {
    /// The capacity usage summary of the resources used by the ReferenceSets in
    /// a firewall.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "capacityUsageSummary")]
    pub capacity_usage_summary: Option<FirewallStatusFirewallStatusCapacityUsageSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationSyncStateSummary")]
    pub configuration_sync_state_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncStates")]
    pub sync_states: Option<BTreeMap<String, FirewallStatusFirewallStatusSyncStates>>,
}

/// The capacity usage summary of the resources used by the ReferenceSets in
/// a firewall.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusCapacityUsageSummary {
    /// Summarizes the CIDR blocks used by the IP set references in a firewall. Network
    /// Firewall calculates the number of CIDRs by taking an aggregated count of
    /// all CIDRs used by the IP sets you are referencing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<FirewallStatusFirewallStatusCapacityUsageSummaryCidrs>,
}

/// Summarizes the CIDR blocks used by the IP set references in a firewall. Network
/// Firewall calculates the number of CIDRs by taking an aggregated count of
/// all CIDRs used by the IP sets you are referencing.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusCapacityUsageSummaryCidrs {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableCIDRCount")]
    pub available_cidr_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iPSetReferences")]
    pub i_p_set_references: Option<BTreeMap<String, FirewallStatusFirewallStatusCapacityUsageSummaryCidrsIPSetReferences>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "utilizedCIDRCount")]
    pub utilized_cidr_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusCapacityUsageSummaryCidrsIPSetReferences {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedCIDRCount")]
    pub resolved_cidr_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusSyncStates {
    /// The configuration and status for a single subnet that you've specified for
    /// use by the Network Firewall firewall. This is part of the FirewallStatus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<FirewallStatusFirewallStatusSyncStatesAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, FirewallStatusFirewallStatusSyncStatesConfig>>,
}

/// The configuration and status for a single subnet that you've specified for
/// use by the Network Firewall firewall. This is part of the FirewallStatus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusSyncStatesAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointID")]
    pub endpoint_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallStatusFirewallStatusSyncStatesConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncStatus")]
    pub sync_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateToken")]
    pub update_token: Option<String>,
}
