// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta2/ibmpowervsmachines.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// IBMPowerVSMachineSpec defines the desired state of IBMPowerVSMachine.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta2", kind = "IBMPowerVSMachine", plural = "ibmpowervsmachines")]
#[kube(namespaced)]
#[kube(status = "IBMPowerVSMachineStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMPowerVSMachineSpec {
    /// Image the reference to the image which is used to create the instance.
    /// supported image identifier in IBMPowerVSResourceReference are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<IBMPowerVSMachineImage>,
    /// ImageRef is an optional reference to a provider-specific resource that holds
    /// the details for provisioning the Image for a Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageRef")]
    pub image_ref: Option<IBMPowerVSMachineImageRef>,
    /// memoryGiB is the size of a virtual machine's memory, in GiB.
    /// maximum value for the MemoryGiB depends on the selected SystemType.
    /// when SystemType is set to e880 maximum MemoryGiB value is 7463 GiB.
    /// when SystemType is set to e980 maximum MemoryGiB value is 15307 GiB.
    /// when SystemType is set to s922 maximum MemoryGiB value is 942 GiB.
    /// The minimum memory is 2 GiB.
    /// When omitted, this means the user has no opinion and the platform is left to choose a reasonable
    /// default, which is subject to change over time. The current default is 2.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryGiB")]
    pub memory_gi_b: Option<i32>,
    /// Network is the reference to the Network to use for this instance.
    /// supported network identifier in IBMPowerVSResourceReference are Name, ID and RegEx and that can be obtained from IBM Cloud UI or IBM Cloud cli.
    pub network: IBMPowerVSMachineNetwork,
    /// processorType is the VM instance processor type.
    /// It must be set to one of the following values: Dedicated, Capped or Shared.
    /// Dedicated: resources are allocated for a specific client, The hypervisor makes a 1:1 binding of a partition’s processor to a physical processor core.
    /// Shared: Shared among other clients.
    /// Capped: Shared, but resources do not expand beyond those that are requested, the amount of CPU time is Capped to the value specified for the entitlement.
    /// if the processorType is selected as Dedicated, then processors value cannot be fractional.
    /// When omitted, this means that the user has no opinion and the platform is left to choose a
    /// reasonable default, which is subject to change over time. The current default is Shared.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processorType")]
    pub processor_type: Option<IBMPowerVSMachineProcessorType>,
    /// processors is the number of virtual processors in a virtual machine.
    /// when the processorType is selected as Dedicated the processors value cannot be fractional.
    /// maximum value for the Processors depends on the selected SystemType.
    /// when SystemType is set to e880 or e980 maximum Processors value is 143.
    /// when SystemType is set to s922 maximum Processors value is 15.
    /// minimum value for Processors depends on the selected ProcessorType.
    /// when ProcessorType is set as Shared or Capped, The minimum processors is 0.25.
    /// when ProcessorType is set as Dedicated, The minimum processors is 1.
    /// When omitted, this means that the user has no opinion and the platform is left to choose a
    /// reasonable default, which is subject to change over time. The default is set based on the selected ProcessorType.
    /// when ProcessorType selected as Dedicated, the default is set to 1.
    /// when ProcessorType selected as Shared or Capped, the default is set to 0.25.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<IntOrString>,
    /// ProviderID is the unique identifier as specified by the cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// serviceInstance is the reference to the Power VS workspace on which the server instance(VM) will be created.
    /// Power VS workspace is a container for all Power VS instances at a specific geographic region.
    /// serviceInstance can be created via IBM Cloud catalog or CLI.
    /// supported serviceInstance identifier in PowerVSResource are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
    /// More detail about Power VS service instance.
    /// https://cloud.ibm.com/docs/power-iaas?topic=power-iaas-creating-power-virtual-server
    /// when omitted system will dynamically create the service instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceInstance")]
    pub service_instance: Option<IBMPowerVSMachineServiceInstance>,
    /// ServiceInstanceID is the id of the power cloud instance where the vsi instance will get deployed.
    /// Deprecated: use ServiceInstance instead
    #[serde(rename = "serviceInstanceID")]
    pub service_instance_id: String,
    /// SSHKey is the name of the SSH key pair provided to the vsi for authenticating users.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKey")]
    pub ssh_key: Option<String>,
    /// systemType is the System type used to host the instance.
    /// systemType determines the number of cores and memory that is available.
    /// Few of the supported SystemTypes are s922,e880,e980.
    /// e880 systemType available only in Dallas Datacenters.
    /// e980 systemType available in Datacenters except Dallas and Washington.
    /// When omitted, this means that the user has no opinion and the platform is left to choose a
    /// reasonable default, which is subject to change over time. The current default is s922 which is generally available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemType")]
    pub system_type: Option<IBMPowerVSMachineSystemType>,
}

/// Image the reference to the image which is used to create the instance.
/// supported image identifier in IBMPowerVSResourceReference are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineImage {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// ImageRef is an optional reference to a provider-specific resource that holds
/// the details for provisioning the Image for a Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineImageRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Drop `kubebuilder:default` when controller-gen doesn't need it https://github.com/kubernetes-sigs/kubebuilder/issues/3896.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Network is the reference to the Network to use for this instance.
/// supported network identifier in IBMPowerVSResourceReference are Name, ID and RegEx and that can be obtained from IBM Cloud UI or IBM Cloud cli.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineNetwork {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// IBMPowerVSMachineSpec defines the desired state of IBMPowerVSMachine.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IBMPowerVSMachineProcessorType {
    Dedicated,
    Shared,
    Capped,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// serviceInstance is the reference to the Power VS workspace on which the server instance(VM) will be created.
/// Power VS workspace is a container for all Power VS instances at a specific geographic region.
/// serviceInstance can be created via IBM Cloud catalog or CLI.
/// supported serviceInstance identifier in PowerVSResource are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
/// More detail about Power VS service instance.
/// https://cloud.ibm.com/docs/power-iaas?topic=power-iaas-creating-power-virtual-server
/// when omitted system will dynamically create the service instance
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineServiceInstance {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// IBMPowerVSMachineSpec defines the desired state of IBMPowerVSMachine.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IBMPowerVSMachineSystemType {
    #[serde(rename = "s922")]
    S922,
    #[serde(rename = "e880")]
    E880,
    #[serde(rename = "e980")]
    E980,
    #[serde(rename = "s1022")]
    S1022,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// IBMPowerVSMachineStatus defines the observed state of IBMPowerVSMachine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineStatus {
    /// Addresses contains the vsi associated addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<IBMPowerVSMachineStatusAddresses>>,
    /// Conditions defines current service state of the IBMPowerVSMachine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureMessage will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a more verbose string suitable
    /// for logging and human consumption.
    /// 
    /// 
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the Machine's spec or the configuration of
    /// the controller, and that manual intervention is required. Examples
    /// of terminal errors would be invalid combinations of settings in the
    /// spec, values that are unsupported by the controller, or the
    /// responsible controller itself being critically misconfigured.
    /// 
    /// 
    /// Any transient errors that occur during the reconciliation of Machines
    /// can be added as events to the Machine object and/or logged in the
    /// controller's output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// FailureReason will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a succinct value suitable
    /// for machine interpretation.
    /// 
    /// 
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the Machine's spec or the configuration of
    /// the controller, and that manual intervention is required. Examples
    /// of terminal errors would be invalid combinations of settings in the
    /// spec, values that are unsupported by the controller, or the
    /// responsible controller itself being critically misconfigured.
    /// 
    /// 
    /// Any transient errors that occur during the reconciliation of Machines
    /// can be added as events to the Machine object and/or logged in the
    /// controller's output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Fault will report if any fault messages for the vsi.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    /// Health is the health of the vsi.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceID")]
    pub instance_id: Option<String>,
    /// InstanceState is the status of the vsi.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceState")]
    pub instance_state: Option<String>,
    /// Ready is true when the provider resource is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// Region specifies the Power VS Service instance region.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Zone specifies the Power VS Service instance zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// NodeAddress contains information for the node's address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineStatusAddresses {
    /// The node address.
    pub address: String,
    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[serde(rename = "type")]
    pub r#type: String,
}

