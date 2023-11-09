// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/subnets.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SubnetSpec defines the desired state of Subnet. 
///  Describes a subnet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "Subnet", plural = "subnets")]
#[kube(namespaced)]
#[kube(status = "SubnetStatus")]
#[kube(schema = "disabled")]
pub struct SubnetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "assignIPv6AddressOnCreation")]
    pub assign_i_pv6_address_on_creation: Option<bool>,
    /// The Availability Zone or Local Zone for the subnet. 
    ///  Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet. 
    ///  To create a subnet in a Local Zone, set this value to the Local Zone ID, for example us-west-2-lax-1a. For information about the Regions that support Local Zones, see Available Regions (https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) in the Amazon Elastic Compute Cloud User Guide. 
    ///  To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZone")]
    pub availability_zone: Option<String>,
    /// The AZ ID or the Local Zone ID of the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZoneID")]
    pub availability_zone_id: Option<String>,
    /// The IPv4 network range for the subnet, in CIDR notation. For example, 10.0.0.0/24. We modify the specified CIDR block to its canonical form; for example, if you specify 100.68.0.18/18, we modify it to 100.68.0.0/18. 
    ///  This parameter is not supported for an IPv6 only subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlock")]
    pub cidr_block: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customerOwnedIPv4Pool")]
    pub customer_owned_i_pv4_pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableDNS64")]
    pub enable_dns64: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableResourceNameDNSAAAARecord")]
    pub enable_resource_name_dnsaaaa_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableResourceNameDNSARecord")]
    pub enable_resource_name_dnsa_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostnameType")]
    pub hostname_type: Option<String>,
    /// The IPv6 network range for the subnet, in CIDR notation. The subnet size must use a /64 prefix length. 
    ///  This parameter is required for an IPv6 only subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// Indicates whether to create an IPv6 only subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6Native")]
    pub ipv6_native: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapPublicIPOnLaunch")]
    pub map_public_ip_on_launch: Option<bool>,
    /// The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outpostARN")]
    pub outpost_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTableRefs")]
    pub route_table_refs: Option<Vec<SubnetRouteTableRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTables")]
    pub route_tables: Option<Vec<String>>,
    /// The tags. The value parameter is required, but if you don't want the tag to have a value, specify the parameter with no value, and we set the value to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SubnetTags>>,
    /// The ID of the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
    ///  from: name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcRef")]
    pub vpc_ref: Option<SubnetVpcRef>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetRouteTableRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<SubnetRouteTableRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetRouteTableRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetVpcRef {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<SubnetVpcRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetVpcRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SubnetStatus defines the observed state of Subnet
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<SubnetStatusAckResourceMetadata>,
    /// The number of unused private IPv4 addresses in the subnet. The IPv4 addresses for any stopped instances are considered unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableIPAddressCount")]
    pub available_ip_address_count: Option<i64>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SubnetStatusConditions>>,
    /// Indicates whether this is the default subnet for the Availability Zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultForAZ")]
    pub default_for_az: Option<bool>,
    /// Indicates the device position for local network interfaces in this subnet. For example, 1 indicates local network interfaces in this subnet are the secondary network interface (eth1).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableLniAtDeviceIndex")]
    pub enable_lni_at_device_index: Option<i64>,
    /// Information about the IPv6 CIDR blocks associated with the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlockAssociationSet")]
    pub ipv6_cidr_block_association_set: Option<Vec<SubnetStatusIpv6CidrBlockAssociationSet>>,
    /// Indicates whether a network interface created in this subnet (including a network interface created by RunInstances) receives a customer-owned IPv4 address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapCustomerOwnedIPOnLaunch")]
    pub map_customer_owned_ip_on_launch: Option<bool>,
    /// The ID of the Amazon Web Services account that owns the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The type of hostnames to assign to instances in the subnet at launch. An instance hostname is based on the IPv4 address or ID of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateDNSNameOptionsOnLaunch")]
    pub private_dns_name_options_on_launch: Option<SubnetStatusPrivateDnsNameOptionsOnLaunch>,
    /// The current state of the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatusAckResourceMetadata {
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatusConditions {
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

/// Describes an association between a subnet and an IPv6 CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatusIpv6CidrBlockAssociationSet {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associationID")]
    pub association_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// Describes the state of a CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlockState")]
    pub ipv6_cidr_block_state: Option<SubnetStatusIpv6CidrBlockAssociationSetIpv6CidrBlockState>,
}

/// Describes the state of a CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatusIpv6CidrBlockAssociationSetIpv6CidrBlockState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

/// The type of hostnames to assign to instances in the subnet at launch. An instance hostname is based on the IPv4 address or ID of the instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnetStatusPrivateDnsNameOptionsOnLaunch {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableResourceNameDNSAAAARecord")]
    pub enable_resource_name_dnsaaaa_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableResourceNameDNSARecord")]
    pub enable_resource_name_dnsa_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostnameType")]
    pub hostname_type: Option<String>,
}

