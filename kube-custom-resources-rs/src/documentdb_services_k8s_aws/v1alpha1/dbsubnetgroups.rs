// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/documentdb-controller/documentdb.services.k8s.aws/v1alpha1/dbsubnetgroups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DBSubnetGroupSpec defines the desired state of DBSubnetGroup.
/// 
/// 
/// Detailed information about a subnet group.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "documentdb.services.k8s.aws", version = "v1alpha1", kind = "DBSubnetGroup", plural = "dbsubnetgroups")]
#[kube(namespaced)]
#[kube(status = "DBSubnetGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DBSubnetGroupSpec {
    /// The description for the subnet group.
    pub description: String,
    /// The name for the subnet group. This value is stored as a lowercase string.
    /// 
    /// 
    /// Constraints: Must contain no more than 255 letters, numbers, periods, underscores,
    /// spaces, or hyphens. Must not be default.
    /// 
    /// 
    /// Example: mySubnetgroup
    pub name: String,
    /// The Amazon EC2 subnet IDs for the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRefs")]
    pub subnet_refs: Option<Vec<DBSubnetGroupSubnetRefs>>,
    /// The tags to be assigned to the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DBSubnetGroupTags>>,
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
pub struct DBSubnetGroupSubnetRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DBSubnetGroupSubnetRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupSubnetRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Metadata assigned to an Amazon DocumentDB resource consisting of a key-value
/// pair.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DBSubnetGroupStatus defines the observed state of DBSubnetGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DBSubnetGroupStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Provides the status of the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetGroupStatus")]
    pub subnet_group_status: Option<String>,
    /// Detailed information about one or more subnets within a subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<DBSubnetGroupStatusSubnets>>,
    /// Provides the virtual private cloud (VPC) ID of the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupStatusAckResourceMetadata {
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

/// Detailed information about a subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupStatusSubnets {
    /// Information about an Availability Zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetAvailabilityZone")]
    pub subnet_availability_zone: Option<DBSubnetGroupStatusSubnetsSubnetAvailabilityZone>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIdentifier")]
    pub subnet_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetStatus")]
    pub subnet_status: Option<String>,
}

/// Information about an Availability Zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBSubnetGroupStatusSubnetsSubnetAvailabilityZone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

