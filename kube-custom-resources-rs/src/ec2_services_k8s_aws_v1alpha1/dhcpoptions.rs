// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/dhcpoptions.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// DhcpOptionsSpec defines the desired state of DhcpOptions. 
///  Describes a set of DHCP options.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "DHCPOptions", plural = "dhcpoptions")]
#[kube(namespaced)]
#[kube(status = "DHCPOptionsStatus")]
pub struct DHCPOptionsSpec {
    /// A DHCP configuration option.
    #[serde(rename = "dhcpConfigurations")]
    pub dhcp_configurations: Vec<DHCPOptionsDhcpConfigurations>,
    /// The tags. The value parameter is required, but if you don't want the tag to have a value, specify the parameter with no value, and we set the value to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DHCPOptionsTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcRefs")]
    pub vpc_refs: Option<Vec<DHCPOptionsVpcRefs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsDhcpConfigurations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsVpcRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DHCPOptionsVpcRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsVpcRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DHCPOptionsStatus defines the observed state of DHCPOptions
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DHCPOptionsStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DHCPOptionsStatusConditions>>,
    /// The ID of the set of DHCP options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcpOptionsID")]
    pub dhcp_options_id: Option<String>,
    /// The ID of the Amazon Web Services account that owns the DHCP options set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DHCPOptionsStatusAckResourceMetadata {
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
pub struct DHCPOptionsStatusConditions {
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

