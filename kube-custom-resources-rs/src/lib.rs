/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# Available Features

Every group has its own feature in this crate. The available features are as follows:

## about_k8s_io

apiVersion `about.k8s.io/v1alpha1`:
- `ClusterProperty`

## acid_zalan_do

apiVersion `acid.zalan.do/v1`:
- `PostgresTeam`

## acme_cert_manager_io

apiVersion `acme.cert-manager.io/v1`:
- `Challenge`
- `Order`

## acmpca_services_k8s_aws

apiVersion `acmpca.services.k8s.aws/v1alpha1`:
- `CertificateAuthority`
- `CertificateAuthorityActivation`
- `Certificate`

## actions_github_com

apiVersion `actions.github.com/v1alpha1`:
- `AutoscalingListener`
- `AutoscalingRunnerSet`
- `EphemeralRunnerSet`

## actions_summerwind_dev

apiVersion `actions.summerwind.dev/v1alpha1`:
- `HorizontalRunnerAutoscaler`
- `RunnerDeployment`
- `RunnerReplicaSet`
- `Runner`
- `RunnerSet`

## addons_cluster_x_k8s_io

apiVersion `addons.cluster.x-k8s.io/v1alpha3`:
- `ClusterResourceSetBinding`
- `ClusterResourceSet`

apiVersion `addons.cluster.x-k8s.io/v1alpha4`:
- `ClusterResourceSetBinding`
- `ClusterResourceSet`

apiVersion `addons.cluster.x-k8s.io/v1beta1`:
- `ClusterResourceSetBinding`
- `ClusterResourceSet`

## agent_k8s_elastic_co

apiVersion `agent.k8s.elastic.co/v1alpha1`:
- `Agent`

## anywhere_eks_amazonaws_com

apiVersion `anywhere.eks.amazonaws.com/v1alpha1`:
- `AWSDatacenterConfig`
- `AWSIamConfig`
- `Bundles`
- `CloudStackDatacenterConfig`
- `CloudStackMachineConfig`
- `Cluster`
- `ControlPlaneUpgrade`
- `DockerDatacenterConfig`
- `EKSARelease`
- `FluxConfig`
- `GitOpsConfig`
- `MachineDeploymentUpgrade`
- `NodeUpgrade`
- `NutanixDatacenterConfig`
- `NutanixMachineConfig`
- `OIDCConfig`
- `SnowDatacenterConfig`
- `SnowIPPool`
- `SnowMachineConfig`
- `TinkerbellDatacenterConfig`
- `TinkerbellMachineConfig`
- `TinkerbellTemplateConfig`
- `VSphereDatacenterConfig`
- `VSphereMachineConfig`

## apacheweb_arsenal_dev

apiVersion `apacheweb.arsenal.dev/v1alpha1`:
- `Apacheweb`

## api_clever_cloud_com

apiVersion `api.clever-cloud.com/v1`:
- `ConfigProvider`
- `ElasticSearch`
- `MongoDb`
- `MySql`
- `PostgreSql`
- `Redis`

apiVersion `api.clever-cloud.com/v1beta1`:
- `Pulsar`

## api_kubemod_io

apiVersion `api.kubemod.io/v1beta1`:
- `ModRule`

## apicodegen_apimatic_io

apiVersion `apicodegen.apimatic.io/v1beta1`:
- `APIMatic`

## apiextensions_crossplane_io

apiVersion `apiextensions.crossplane.io/v1`:
- `CompositeResourceDefinition`

## apigatewayv2_services_k8s_aws

apiVersion `apigatewayv2.services.k8s.aws/v1alpha1`:
- `API`
- `Authorizer`
- `Deployment`
- `Route`
- `Stage`
- `VPCLink`

## apisix_apache_org

apiVersion `apisix.apache.org/v2`:
- `ApisixClusterConfig`
- `ApisixConsumer`
- `ApisixGlobalRule`
- `ApisixPluginConfig`
- `ApisixRoute`
- `ApisixTls`
- `ApisixUpstream`

## apm_k8s_elastic_co

apiVersion `apm.k8s.elastic.co/v1`:
- `ApmServer`

apiVersion `apm.k8s.elastic.co/v1beta1`:
- `ApmServer`

## app_kiegroup_org

apiVersion `app.kiegroup.org/v1beta1`:
- `KogitoBuild`
- `KogitoInfra`
- `KogitoRuntime`
- `KogitoSupportingService`

## app_lightbend_com

apiVersion `app.lightbend.com/v1alpha1`:
- `AkkaCluster`

## app_redislabs_com

apiVersion `app.redislabs.com/v1`:
- `RedisEnterpriseCluster`

apiVersion `app.redislabs.com/v1alpha1`:
- `RedisEnterpriseActiveActiveDatabase`
- `RedisEnterpriseCluster`
- `RedisEnterpriseDatabase`
- `RedisEnterpriseRemoteCluster`

## app_terraform_io

apiVersion `app.terraform.io/v1alpha2`:
- `AgentPool`
- `Module`
- `Workspace`

## application_networking_k8s_aws

apiVersion `application-networking.k8s.aws/v1alpha1`:
- `AccessLogPolicy`
- `IAMAuthPolicy`
- `ServiceExport`
- `ServiceImport`
- `TargetGroupPolicy`
- `VpcAssociationPolicy`

## applicationautoscaling_services_k8s_aws

apiVersion `applicationautoscaling.services.k8s.aws/v1alpha1`:
- `ScalableTarget`
- `ScalingPolicy`

## appmesh_k8s_aws

apiVersion `appmesh.k8s.aws/v1beta2`:
- `BackendGroup`
- `GatewayRoute`
- `Mesh`
- `VirtualGateway`
- `VirtualNode`
- `VirtualRouter`
- `VirtualService`

## appprotect_f5_com

apiVersion `appprotect.f5.com/v1beta1`:
- `APLogConf`
- `APUserSig`

## appprotectdos_f5_com

apiVersion `appprotectdos.f5.com/v1beta1`:
- `APDosLogConf`
- `APDosPolicy`
- `DosProtectedResource`

## apps_3scale_net

apiVersion `apps.3scale.net/v1alpha1`:
- `APIManagerBackup`
- `APIManagerRestore`
- `APIManager`
- `APIcast`

## apps_clusternet_io

apiVersion `apps.clusternet.io/v1alpha1`:
- `Base`
- `Description`
- `FeedInventory`
- `Globalization`
- `HelmChart`
- `HelmRelease`
- `Localization`
- `Manifest`
- `Subscription`

## apps_emqx_io

apiVersion `apps.emqx.io/v1beta3`:
- `EmqxBroker`
- `EmqxEnterprise`
- `EmqxPlugin`

apiVersion `apps.emqx.io/v1beta4`:
- `EmqxBroker`
- `EmqxEnterprise`
- `EmqxPlugin`
- `Rebalance`

apiVersion `apps.emqx.io/v2alpha1`:
- `EMQX`

apiVersion `apps.emqx.io/v2beta1`:
- `EMQX`
- `Rebalance`

## apps_gitlab_com

apiVersion `apps.gitlab.com/v1beta1`:
- `GitLab`

apiVersion `apps.gitlab.com/v1beta2`:
- `Runner`

## apps_kubeblocks_io

apiVersion `apps.kubeblocks.io/v1alpha1`:
- `BackupPolicyTemplate`
- `ClusterDefinition`
- `Cluster`
- `ClusterVersion`
- `ComponentClassDefinition`
- `ComponentDefinition`
- `Component`
- `ComponentVersion`
- `ConfigConstraint`
- `Configuration`
- `OpsDefinition`
- `OpsRequest`
- `ServiceDescriptor`

apiVersion `apps.kubeblocks.io/v1beta1`:
- `ConfigConstraint`

## apps_kubedl_io

apiVersion `apps.kubedl.io/v1alpha1`:
- `Cron`

## apps_kubeedge_io

apiVersion `apps.kubeedge.io/v1alpha1`:
- `NodeGroup`

## apps_m88i_io

apiVersion `apps.m88i.io/v1alpha1`:
- `Nexus`

## apps_redhat_com

apiVersion `apps.redhat.com/v1alpha1`:
- `ClusterImpairment`

## aquasecurity_github_io

apiVersion `aquasecurity.github.io/v1alpha1`:
- `AquaStarboard`
- `ClusterConfigAuditReport`
- `ConfigAuditReport`

## argoproj_io

apiVersion `argoproj.io/v1alpha1`:
- `Application`
- `AppProject`
- `ArgoCDExport`
- `ArgoCD`

apiVersion `argoproj.io/v1beta1`:
- `ArgoCD`

## asdb_aerospike_com

apiVersion `asdb.aerospike.com/v1`:
- `AerospikeCluster`

apiVersion `asdb.aerospike.com/v1beta1`:
- `AerospikeCluster`

## atlasmap_io

apiVersion `atlasmap.io/v1alpha1`:
- `AtlasMap`

## auth_ops42_org

apiVersion `auth.ops42.org/v1alpha1`:
- `AwsAuthSyncConfig`

## authzed_com

apiVersion `authzed.com/v1alpha1`:
- `SpiceDBCluster`

## automation_kubensync_com

apiVersion `automation.kubensync.com/v1alpha1`:
- `ManagedResource`

## autoscaling_k8s_io

apiVersion `autoscaling.k8s.io/v1`:
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`

apiVersion `autoscaling.k8s.io/v1beta2`:
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`

## autoscaling_karmada_io

apiVersion `autoscaling.karmada.io/v1alpha1`:
- `CronFederatedHPA`
- `FederatedHPA`

## awx_ansible_com

apiVersion `awx.ansible.com/v1beta1`:
- `AWXBackup`
- `AWXRestore`
- `AWX`

## azure_microsoft_com

apiVersion `azure.microsoft.com/v1alpha1`:
- `APIMgmtAPI`
- `ApimService`
- `AppInsights`
- `AppInsightsApiKey`
- `AzureLoadBalancer`
- `AzureNetworkInterface`
- `AzurePublicIPAddress`
- `AzureSqlAction`
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSQLManagedUser`
- `AzureSqlServer`
- `AzureSQLUser`
- `AzureSQLVNetRule`
- `AzureVirtualMachineExtension`
- `AzureVirtualMachine`
- `AzureVMScaleSet`
- `BlobContainer`
- `ConsumerGroup`
- `CosmosDB`
- `EventhubNamespace`
- `Eventhub`
- `KeyVaultKey`
- `KeyVault`
- `MySQLAADUser`
- `MySQLDatabase`
- `MySQLFirewallRule`
- `MySQLServerAdministrator`
- `MySQLServer`
- `MySQLUser`
- `MySQLVNetRule`
- `PostgreSQLDatabase`
- `PostgreSQLFirewallRule`
- `PostgreSQLServer`
- `PostgreSQLUser`
- `PostgreSQLVNetRule`
- `RedisCacheAction`
- `RedisCacheFirewallRule`
- `ResourceGroup`
- `StorageAccount`
- `VirtualNetwork`

apiVersion `azure.microsoft.com/v1alpha2`:
- `BlobContainer`
- `MySQLAADUser`
- `MySQLServer`
- `MySQLUser`
- `PostgreSQLServer`

apiVersion `azure.microsoft.com/v1beta1`:
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSqlServer`

## b3scale_infra_run

apiVersion `b3scale.infra.run/v1`:
- `BBBFrontend`

## b3scale_io

apiVersion `b3scale.io/v1`:
- `BBBFrontend`

## batch_volcano_sh

apiVersion `batch.volcano.sh/v1alpha1`:
- `Job`

## beat_k8s_elastic_co

apiVersion `beat.k8s.elastic.co/v1beta1`:
- `Beat`

## beegfs_csi_netapp_com

apiVersion `beegfs.csi.netapp.com/v1`:
- `BeegfsDriver`

## binding_operators_coreos_com

apiVersion `binding.operators.coreos.com/v1alpha1`:
- `BindableKinds`
- `ServiceBinding`

## bitnami_com

apiVersion `bitnami.com/v1alpha1`:
- `SealedSecret`

## bmc_tinkerbell_org

apiVersion `bmc.tinkerbell.org/v1alpha1`:
- `Job`
- `Machine`
- `Task`

## boskos_k8s_io

apiVersion `boskos.k8s.io/v1`:
- `DRLCObject`
- `ResourceObject`

## bpfman_io

apiVersion `bpfman.io/v1alpha1`:
- `BpfProgram`
- `FentryProgram`
- `FexitProgram`
- `KprobeProgram`
- `TcProgram`
- `TracepointProgram`
- `UprobeProgram`
- `XdpProgram`

## bus_volcano_sh

apiVersion `bus.volcano.sh/v1alpha1`:
- `Command`

## cache_kubedl_io

apiVersion `cache.kubedl.io/v1alpha1`:
- `CacheBackend`

## caching_ibm_com

apiVersion `caching.ibm.com/v1alpha1`:
- `VarnishCluster`

## camel_apache_org

apiVersion `camel.apache.org/v1`:
- `Build`
- `CamelCatalog`
- `Kamelet`

apiVersion `camel.apache.org/v1alpha1`:
- `Kamelet`

## capabilities_3scale_net

apiVersion `capabilities.3scale.net/v1alpha1`:
- `Tenant`

apiVersion `capabilities.3scale.net/v1beta1`:
- `ActiveDoc`
- `Application`
- `Backend`
- `CustomPolicyDefinition`
- `DeveloperAccount`
- `DeveloperUser`
- `OpenAPI`
- `Product`
- `ProxyConfigPromote`

## capsule_clastix_io

apiVersion `capsule.clastix.io/v1alpha1`:
- `CapsuleConfiguration`
- `Tenant`

apiVersion `capsule.clastix.io/v1beta1`:
- `Tenant`

apiVersion `capsule.clastix.io/v1beta2`:
- `CapsuleConfiguration`
- `Tenant`

## cassandra_datastax_com

apiVersion `cassandra.datastax.com/v1beta1`:
- `CassandraDatacenter`

## ceph_rook_io

apiVersion `ceph.rook.io/v1`:
- `CephBlockPoolRadosNamespace`
- `CephBlockPool`
- `CephBucketNotification`
- `CephBucketTopic`
- `CephClient`
- `CephCOSIDriver`
- `CephFilesystemMirror`
- `CephFilesystem`
- `CephFilesystemSubVolumeGroup`
- `CephNFS`
- `CephObjectRealm`
- `CephObjectStore`
- `CephObjectStoreUser`
- `CephObjectZoneGroup`
- `CephObjectZone`
- `CephRBDMirror`

## cert_manager_io

apiVersion `cert-manager.io/v1`:
- `CertificateRequest`
- `Certificate`
- `ClusterIssuer`
- `Issuer`

## chainsaw_kyverno_io

apiVersion `chainsaw.kyverno.io/v1alpha1`:
- `Configuration`
- `Test`

apiVersion `chainsaw.kyverno.io/v1alpha2`:
- `Configuration`
- `Test`

## chaos_mesh_org

apiVersion `chaos-mesh.org/v1alpha1`:
- `AWSChaos`
- `AzureChaos`
- `BlockChaos`
- `DNSChaos`
- `GCPChaos`
- `HTTPChaos`
- `IOChaos`
- `JVMChaos`
- `KernelChaos`
- `NetworkChaos`
- `PhysicalMachineChaos`
- `PhysicalMachine`
- `PodChaos`
- `PodHttpChaos`
- `PodIOChaos`
- `PodNetworkChaos`
- `RemoteCluster`
- `Schedule`
- `StatusCheck`
- `StressChaos`
- `TimeChaos`
- `WorkflowNode`
- `Workflow`

## chaosblade_io

apiVersion `chaosblade.io/v1alpha1`:
- `ChaosBlade`

## charts_amd_com

apiVersion `charts.amd.com/v1alpha1`:
- `AMDGPU`

## charts_flagsmith_com

apiVersion `charts.flagsmith.com/v1alpha1`:
- `Flagsmith`

## charts_helm_k8s_io

apiVersion `charts.helm.k8s.io/v1alpha1`:
- `SnykMonitor`

## charts_opdev_io

apiVersion `charts.opdev.io/v1alpha1`:
- `Synapse`

## charts_operatorhub_io

apiVersion `charts.operatorhub.io/v1alpha1`:
- `Cockroachdb`

## che_eclipse_org

apiVersion `che.eclipse.org/v1alpha1`:
- `KubernetesImagePuller`

## chisel_operator_io

apiVersion `chisel-operator.io/v1`:
- `ExitNodeProvisioner`
- `ExitNode`

apiVersion `chisel-operator.io/v2`:
- `ExitNode`

## cilium_io

apiVersion `cilium.io/v2`:
- `CiliumClusterwideNetworkPolicy`
- `CiliumEgressGatewayPolicy`
- `CiliumEndpoint`
- `CiliumExternalWorkload`
- `CiliumIdentity`
- `CiliumLocalRedirectPolicy`
- `CiliumNetworkPolicy`
- `CiliumNode`

apiVersion `cilium.io/v2alpha1`:
- `CiliumBGPPeeringPolicy`
- `CiliumCIDRGroup`
- `CiliumEndpointSlice`
- `CiliumL2AnnouncementPolicy`
- `CiliumLoadBalancerIPPool`
- `CiliumNodeConfig`
- `CiliumPodIPPool`

## claudie_io

apiVersion `claudie.io/v1beta1`:
- `InputManifest`

## cloudformation_linki_space

apiVersion `cloudformation.linki.space/v1alpha1`:
- `Stack`

## cloudfront_services_k8s_aws

apiVersion `cloudfront.services.k8s.aws/v1alpha1`:
- `CachePolicy`
- `Distribution`
- `Function`
- `OriginRequestPolicy`
- `ResponseHeadersPolicy`

## cloudtrail_services_k8s_aws

apiVersion `cloudtrail.services.k8s.aws/v1alpha1`:
- `EventDataStore`
- `Trail`

## cloudwatch_aws_amazon_com

apiVersion `cloudwatch.aws.amazon.com/v1alpha1`:
- `AmazonCloudWatchAgent`
- `Instrumentation`

## cloudwatch_services_k8s_aws

apiVersion `cloudwatch.services.k8s.aws/v1alpha1`:
- `MetricAlarm`

## cloudwatchlogs_services_k8s_aws

apiVersion `cloudwatchlogs.services.k8s.aws/v1alpha1`:
- `LogGroup`

## cluster_clusterpedia_io

apiVersion `cluster.clusterpedia.io/v1alpha2`:
- `ClusterSyncResources`
- `PediaCluster`

## cluster_ipfs_io

apiVersion `cluster.ipfs.io/v1alpha1`:
- `CircuitRelay`
- `IpfsCluster`

## cluster_x_k8s_io

apiVersion `cluster.x-k8s.io/v1alpha3`:
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`

apiVersion `cluster.x-k8s.io/v1alpha4`:
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`

apiVersion `cluster.x-k8s.io/v1beta1`:
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`

## clusters_clusternet_io

apiVersion `clusters.clusternet.io/v1beta1`:
- `ClusterRegistrationRequest`
- `ManagedCluster`

## clustertemplate_openshift_io

apiVersion `clustertemplate.openshift.io/v1alpha1`:
- `ClusterTemplateInstance`
- `ClusterTemplateQuota`
- `ClusterTemplate`
- `ClusterTemplateSetup`
- `Config`

## confidentialcontainers_org

apiVersion `confidentialcontainers.org/v1beta1`:
- `CcRuntime`

## config_gatekeeper_sh

apiVersion `config.gatekeeper.sh/v1alpha1`:
- `Config`

## config_grafana_com

apiVersion `config.grafana.com/v1`:
- `ProjectConfig`

## config_karmada_io

apiVersion `config.karmada.io/v1alpha1`:
- `ResourceInterpreterCustomization`
- `ResourceInterpreterWebhookConfiguration`

## config_koordinator_sh

apiVersion `config.koordinator.sh/v1alpha1`:
- `ClusterColocationProfile`

## config_storageos_com

apiVersion `config.storageos.com/v1`:
- `OperatorConfig`

## control_k8ssandra_io

apiVersion `control.k8ssandra.io/v1alpha1`:
- `CassandraTask`

## core_kubeadmiral_io

apiVersion `core.kubeadmiral.io/v1alpha1`:
- `ClusterCollectedStatus`
- `ClusterFederatedObject`
- `ClusterOverridePolicy`
- `ClusterPropagatedVersion`
- `ClusterPropagationPolicy`
- `CollectedStatus`
- `FederatedCluster`
- `FederatedObject`
- `FederatedTypeConfig`
- `OverridePolicy`
- `PropagatedVersion`
- `PropagationPolicy`
- `SchedulerPluginWebhookConfiguration`
- `SchedulingProfile`

## core_linuxsuren_github_com

apiVersion `core.linuxsuren.github.com/v1alpha1`:
- `ATest`

## core_openfeature_dev

apiVersion `core.openfeature.dev/v1alpha1`:
- `FeatureFlagConfiguration`

apiVersion `core.openfeature.dev/v1alpha2`:
- `FeatureFlagConfiguration`

## couchbase_com

apiVersion `couchbase.com/v2`:
- `CouchbaseAutoscaler`
- `CouchbaseBackupRestore`
- `CouchbaseBackup`
- `CouchbaseBucket`
- `CouchbaseCluster`
- `CouchbaseCollectionGroup`
- `CouchbaseCollection`
- `CouchbaseEphemeralBucket`
- `CouchbaseGroup`
- `CouchbaseMemcachedBucket`
- `CouchbaseMigrationReplication`
- `CouchbaseReplication`
- `CouchbaseRoleBinding`
- `CouchbaseScopeGroup`
- `CouchbaseScope`
- `CouchbaseUser`

## craftypath_github_io

apiVersion `craftypath.github.io/v1alpha1`:
- `SopsSecret`

## crane_konveyor_io

apiVersion `crane.konveyor.io/v1alpha1`:
- `OperatorConfig`

## crd_projectcalico_org

apiVersion `crd.projectcalico.org/v1`:
- `BGPConfiguration`
- `BGPFilter`
- `BGPPeer`
- `BlockAffinity`
- `CalicoNodeStatus`
- `ClusterInformation`
- `GlobalNetworkSet`
- `HostEndpoint`
- `IPAMBlock`
- `IPAMConfig`
- `IPAMHandle`
- `IPPool`
- `IPReservation`
- `KubeControllersConfiguration`
- `NetworkSet`
- `Tier`

## data_fluid_io

apiVersion `data.fluid.io/v1alpha1`:
- `AlluxioRuntime`
- `DataBackup`
- `DataLoad`
- `Dataset`
- `GooseFSRuntime`
- `JindoRuntime`
- `JuiceFSRuntime`
- `ThinRuntimeProfile`
- `ThinRuntime`

## databases_schemahero_io

apiVersion `databases.schemahero.io/v1alpha4`:
- `Database`

## databases_spotahome_com

apiVersion `databases.spotahome.com/v1`:
- `RedisFailover`

## datadoghq_com

apiVersion `datadoghq.com/v1alpha1`:
- `DatadogAgent`
- `DatadogMetric`
- `DatadogMonitor`
- `DatadogSLO`

apiVersion `datadoghq.com/v2alpha1`:
- `DatadogAgent`

## dataprotection_kubeblocks_io

apiVersion `dataprotection.kubeblocks.io/v1alpha1`:
- `ActionSet`
- `BackupPolicy`
- `BackupRepo`
- `Backup`
- `BackupSchedule`
- `Restore`

## designer_kaoto_io

apiVersion `designer.kaoto.io/v1alpha1`:
- `Kaoto`

## devices_kubeedge_io

apiVersion `devices.kubeedge.io/v1alpha2`:
- `DeviceModel`
- `Device`

apiVersion `devices.kubeedge.io/v1beta1`:
- `DeviceModel`
- `Device`

## devops_kubesphere_io

apiVersion `devops.kubesphere.io/v1alpha1`:
- `ReleaserController`
- `Releaser`

## dex_coreos_com

apiVersion `dex.coreos.com/v1`:
- `AuthCode`
- `AuthRequest`
- `Connector`
- `DeviceRequest`
- `DeviceToken`
- `OAuth2Client`
- `OfflineSessions`
- `Password`
- `RefreshToken`
- `SigningKey`

## dex_gpu_ninja_com

apiVersion `dex.gpu-ninja.com/v1alpha1`:
- `DexIdentityProvider`
- `DexOAuth2Client`
- `DexUser`

## digitalis_io

apiVersion `digitalis.io/v1`:
- `ValsSecret`

apiVersion `digitalis.io/v1beta1`:
- `DbSecret`

## documentdb_services_k8s_aws

apiVersion `documentdb.services.k8s.aws/v1alpha1`:
- `DBCluster`
- `DBInstance`
- `DBSubnetGroup`

## druid_apache_org

apiVersion `druid.apache.org/v1alpha1`:
- `Druid`

## dynamodb_services_k8s_aws

apiVersion `dynamodb.services.k8s.aws/v1alpha1`:
- `Backup`
- `GlobalTable`
- `Table`

## ec2_services_k8s_aws

apiVersion `ec2.services.k8s.aws/v1alpha1`:
- `DHCPOptions`
- `ElasticIPAddress`
- `Instance`
- `InternetGateway`
- `NATGateway`
- `RouteTable`
- `SecurityGroup`
- `Subnet`
- `TransitGateway`
- `VPCEndpoint`
- `VPC`

## ecr_services_k8s_aws

apiVersion `ecr.services.k8s.aws/v1alpha1`:
- `PullThroughCacheRule`
- `Repository`

## efs_services_k8s_aws

apiVersion `efs.services.k8s.aws/v1alpha1`:
- `AccessPoint`
- `FileSystem`
- `MountTarget`

## eks_services_k8s_aws

apiVersion `eks.services.k8s.aws/v1alpha1`:
- `Addon`
- `Cluster`
- `FargateProfile`
- `Nodegroup`

## elasticache_services_k8s_aws

apiVersion `elasticache.services.k8s.aws/v1alpha1`:
- `CacheParameterGroup`
- `CacheSubnetGroup`
- `ReplicationGroup`
- `Snapshot`
- `UserGroup`
- `User`

## elasticsearch_k8s_elastic_co

apiVersion `elasticsearch.k8s.elastic.co/v1`:
- `Elasticsearch`

apiVersion `elasticsearch.k8s.elastic.co/v1beta1`:
- `Elasticsearch`

## elbv2_k8s_aws

apiVersion `elbv2.k8s.aws/v1alpha1`:
- `TargetGroupBinding`

apiVersion `elbv2.k8s.aws/v1beta1`:
- `IngressClassParams`
- `TargetGroupBinding`

## emrcontainers_services_k8s_aws

apiVersion `emrcontainers.services.k8s.aws/v1alpha1`:
- `JobRun`
- `VirtualCluster`

## ensembleoss_io

apiVersion `ensembleoss.io/v1`:
- `Cluster`
- `Resource`

## enterprisesearch_k8s_elastic_co

apiVersion `enterprisesearch.k8s.elastic.co/v1`:
- `EnterpriseSearch`

apiVersion `enterprisesearch.k8s.elastic.co/v1beta1`:
- `EnterpriseSearch`

## everest_percona_com

apiVersion `everest.percona.com/v1alpha1`:
- `BackupStorage`
- `DatabaseClusterBackup`
- `DatabaseClusterRestore`
- `DatabaseCluster`
- `MonitoringConfig`

## execution_furiko_io

apiVersion `execution.furiko.io/v1alpha1`:
- `JobConfig`
- `Job`

## executor_testkube_io

apiVersion `executor.testkube.io/v1`:
- `Executor`
- `Webhook`

## expansion_gatekeeper_sh

apiVersion `expansion.gatekeeper.sh/v1alpha1`:
- `ExpansionTemplate`

apiVersion `expansion.gatekeeper.sh/v1beta1`:
- `ExpansionTemplate`

## extensions_istio_io

apiVersion `extensions.istio.io/v1alpha1`:
- `WasmPlugin`

## extensions_kubeblocks_io

apiVersion `extensions.kubeblocks.io/v1alpha1`:
- `Addon`

## external_secrets_io

apiVersion `external-secrets.io/v1alpha1`:
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`

apiVersion `external-secrets.io/v1beta1`:
- `ClusterExternalSecret`
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`

## externaldata_gatekeeper_sh

apiVersion `externaldata.gatekeeper.sh/v1alpha1`:
- `Provider`

apiVersion `externaldata.gatekeeper.sh/v1beta1`:
- `Provider`

## externaldns_k8s_io

apiVersion `externaldns.k8s.io/v1alpha1`:
- `DNSEndpoint`

## externaldns_nginx_org

apiVersion `externaldns.nginx.org/v1`:
- `DNSEndpoint`

## flagger_app

apiVersion `flagger.app/v1beta1`:
- `AlertProvider`
- `Canary`
- `MetricTemplate`

## flink_apache_org

apiVersion `flink.apache.org/v1beta1`:
- `FlinkDeployment`
- `FlinkSessionJob`

## flow_volcano_sh

apiVersion `flow.volcano.sh/v1alpha1`:
- `JobFlow`
- `JobTemplate`

## flows_netobserv_io

apiVersion `flows.netobserv.io/v1alpha1`:
- `FlowCollector`

apiVersion `flows.netobserv.io/v1beta1`:
- `FlowCollector`

apiVersion `flows.netobserv.io/v1beta2`:
- `FlowCollector`

## fluentbit_fluent_io

apiVersion `fluentbit.fluent.io/v1alpha2`:
- `ClusterFilter`
- `ClusterFluentBitConfig`
- `ClusterInput`
- `ClusterOutput`
- `ClusterParser`
- `Collector`
- `Filter`
- `FluentBitConfig`
- `FluentBit`
- `Output`
- `Parser`

## fluentd_fluent_io

apiVersion `fluentd.fluent.io/v1alpha1`:
- `ClusterFilter`
- `ClusterFluentdConfig`
- `ClusterInput`
- `ClusterOutput`
- `Filter`
- `FluentdConfig`
- `Fluentd`
- `Input`
- `Output`

## flux_framework_org

apiVersion `flux-framework.org/v1alpha1`:
- `MiniCluster`

apiVersion `flux-framework.org/v1alpha2`:
- `MiniCluster`

## forklift_konveyor_io

apiVersion `forklift.konveyor.io/v1beta1`:
- `ForkliftController`
- `Hook`
- `Host`
- `Migration`
- `NetworkMap`
- `OpenstackVolumePopulator`
- `OvirtVolumePopulator`
- `Plan`
- `Provider`
- `StorageMap`

## fossul_io

apiVersion `fossul.io/v1`:
- `BackupConfig`
- `Backup`
- `BackupSchedule`
- `Fossul`
- `Restore`

## gateway_networking_k8s_io

apiVersion `gateway.networking.k8s.io/v1`:
- `GatewayClass`
- `Gateway`
- `GRPCRoute`
- `HTTPRoute`

apiVersion `gateway.networking.k8s.io/v1alpha2`:
- `GRPCRoute`
- `ReferenceGrant`
- `TCPRoute`
- `TLSRoute`
- `UDPRoute`

apiVersion `gateway.networking.k8s.io/v1beta1`:
- `GatewayClass`
- `Gateway`
- `HTTPRoute`
- `ReferenceGrant`

## gateway_nginx_org

apiVersion `gateway.nginx.org/v1alpha1`:
- `ClientSettingsPolicy`
- `NginxGateway`
- `NginxProxy`
- `ObservabilityPolicy`

## getambassador_io

apiVersion `getambassador.io/v3alpha1`:
- `AuthService`
- `ConsulResolver`
- `DevPortal`
- `Host`
- `KubernetesEndpointResolver`
- `KubernetesServiceResolver`
- `Listener`
- `LogService`
- `Module`
- `RateLimitService`
- `TCPMapping`
- `TLSContext`
- `TracingService`

## gitops_hybrid_cloud_patterns_io

apiVersion `gitops.hybrid-cloud-patterns.io/v1alpha1`:
- `Pattern`

## grafana_integreatly_org

apiVersion `grafana.integreatly.org/v1beta1`:
- `GrafanaDashboard`
- `GrafanaDatasource`
- `GrafanaFolder`

## groupsnapshot_storage_k8s_io

apiVersion `groupsnapshot.storage.k8s.io/v1alpha1`:
- `VolumeGroupSnapshotClass`
- `VolumeGroupSnapshotContent`
- `VolumeGroupSnapshot`

## hazelcast_com

apiVersion `hazelcast.com/v1alpha1`:
- `CronHotBackup`
- `Hazelcast`
- `HotBackup`
- `ManagementCenter`
- `Map`
- `WanReplication`

## helm_sigstore_dev

apiVersion `helm.sigstore.dev/v1alpha1`:
- `Rekor`

## helm_toolkit_fluxcd_io

apiVersion `helm.toolkit.fluxcd.io/v2`:
- `HelmRelease`

apiVersion `helm.toolkit.fluxcd.io/v2beta1`:
- `HelmRelease`

apiVersion `helm.toolkit.fluxcd.io/v2beta2`:
- `HelmRelease`

## hive_openshift_io

apiVersion `hive.openshift.io/v1`:
- `Checkpoint`
- `ClusterClaim`
- `ClusterDeploymentCustomization`
- `ClusterDeployment`
- `ClusterDeprovision`
- `ClusterImageSet`
- `ClusterPool`
- `ClusterProvision`
- `ClusterRelocate`
- `ClusterState`
- `DNSZone`
- `HiveConfig`
- `MachinePoolNameLease`
- `MachinePool`
- `SelectorSyncIdentityProvider`
- `SyncIdentityProvider`

## hiveinternal_openshift_io

apiVersion `hiveinternal.openshift.io/v1alpha1`:
- `ClusterSyncLease`
- `ClusterSync`
- `FakeClusterInstall`

## hnc_x_k8s_io

apiVersion `hnc.x-k8s.io/v1alpha2`:
- `HierarchicalResourceQuota`
- `HierarchyConfiguration`
- `HNCConfiguration`
- `SubnamespaceAnchor`

## hyperfoil_io

apiVersion `hyperfoil.io/v1alpha1`:
- `Horreum`

apiVersion `hyperfoil.io/v1alpha2`:
- `Hyperfoil`

## iam_services_k8s_aws

apiVersion `iam.services.k8s.aws/v1alpha1`:
- `Group`
- `InstanceProfile`
- `OpenIDConnectProvider`
- `Policy`
- `Role`
- `User`

## ibmcloud_ibm_com

apiVersion `ibmcloud.ibm.com/v1alpha1`:
- `Composable`

## image_toolkit_fluxcd_io

apiVersion `image.toolkit.fluxcd.io/v1beta1`:
- `ImageUpdateAutomation`
- `ImagePolicy`
- `ImageRepository`

apiVersion `image.toolkit.fluxcd.io/v1beta2`:
- `ImageUpdateAutomation`
- `ImagePolicy`
- `ImageRepository`

## imaging_ingestion_alvearie_org

apiVersion `imaging-ingestion.alvearie.org/v1alpha1`:
- `DicomEventBridge`
- `DicomEventDrivenIngestion`
- `DicomInstanceBinding`
- `DicomStudyBinding`
- `DicomwebIngestionService`
- `DimseIngestionService`
- `DimseProxy`

## inference_kubedl_io

apiVersion `inference.kubedl.io/v1alpha1`:
- `ElasticBatchJob`

## infinispan_org

apiVersion `infinispan.org/v1`:
- `Infinispan`

apiVersion `infinispan.org/v2alpha1`:
- `Backup`
- `Batch`
- `Cache`
- `Restore`

## infra_contrib_fluxcd_io

apiVersion `infra.contrib.fluxcd.io/v1alpha1`:
- `Terraform`

apiVersion `infra.contrib.fluxcd.io/v1alpha2`:
- `Terraform`

## infrastructure_cluster_x_k8s_io

apiVersion `infrastructure.cluster.x-k8s.io/v1alpha1`:
- `KubevirtCluster`
- `KubevirtClusterTemplate`
- `KubevirtMachine`
- `KubevirtMachineTemplate`

apiVersion `infrastructure.cluster.x-k8s.io/v1alpha3`:
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`

apiVersion `infrastructure.cluster.x-k8s.io/v1alpha4`:
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`

apiVersion `infrastructure.cluster.x-k8s.io/v1beta1`:
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
- `TinkerbellCluster`
- `TinkerbellMachine`
- `TinkerbellMachineTemplate`

apiVersion `infrastructure.cluster.x-k8s.io/v1beta2`:
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`

## install_istio_io

apiVersion `install.istio.io/v1alpha1`:
- `IstioOperator`

## installation_mattermost_com

apiVersion `installation.mattermost.com/v1beta1`:
- `Mattermost`

## instana_io

apiVersion `instana.io/v1`:
- `InstanaAgent`

## integration_rock8s_com

apiVersion `integration.rock8s.com/v1beta1`:
- `DeferredResource`
- `Plug`
- `Socket`

## iot_eclipse_org

apiVersion `iot.eclipse.org/v1alpha1`:
- `Ditto`
- `Hawkbit`

## ipam_cluster_x_k8s_io

apiVersion `ipam.cluster.x-k8s.io/v1alpha1`:
- `IPAddressClaim`
- `IPAddress`

apiVersion `ipam.cluster.x-k8s.io/v1beta1`:
- `IPAddressClaim`
- `IPAddress`

## isindir_github_com

apiVersion `isindir.github.com/v1alpha1`:
- `SopsSecret`

apiVersion `isindir.github.com/v1alpha2`:
- `SopsSecret`

apiVersion `isindir.github.com/v1alpha3`:
- `SopsSecret`

## jaegertracing_io

apiVersion `jaegertracing.io/v1`:
- `Jaeger`

## jobset_x_k8s_io

apiVersion `jobset.x-k8s.io/v1alpha2`:
- `JobSet`

## jobsmanager_raczylo_com

apiVersion `jobsmanager.raczylo.com/v1beta1`:
- `ManagedJob`

## k6_io

apiVersion `k6.io/v1alpha1`:
- `K6`
- `PrivateLoadZone`
- `TestRun`

## k8gb_absa_oss

apiVersion `k8gb.absa.oss/v1beta1`:
- `Gslb`

## k8s_keycloak_org

apiVersion `k8s.keycloak.org/v2alpha1`:
- `KeycloakRealmImport`
- `Keycloak`

## k8s_mariadb_com

apiVersion `k8s.mariadb.com/v1alpha1`:
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `MaxScale`
- `Restore`
- `SqlJob`
- `User`

## k8s_nginx_org

apiVersion `k8s.nginx.org/v1`:
- `GlobalConfiguration`
- `Policy`
- `TransportServer`
- `VirtualServerRoute`
- `VirtualServer`

apiVersion `k8s.nginx.org/v1alpha1`:
- `GlobalConfiguration`
- `Policy`
- `TransportServer`

## k8s_otterize_com

apiVersion `k8s.otterize.com/v1alpha2`:
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`

apiVersion `k8s.otterize.com/v1alpha3`:
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`

## k8up_io

apiVersion `k8up.io/v1`:
- `Archive`
- `Backup`
- `Check`
- `PreBackupPod`
- `Prune`
- `Restore`
- `Schedule`
- `Snapshot`

## kafka_banzaicloud_io

apiVersion `kafka.banzaicloud.io/v1alpha1`:
- `CruiseControlOperation`
- `KafkaTopic`
- `KafkaUser`

apiVersion `kafka.banzaicloud.io/v1beta1`:
- `KafkaCluster`

## kafka_services_k8s_aws

apiVersion `kafka.services.k8s.aws/v1alpha1`:
- `Cluster`

## kafka_strimzi_io

apiVersion `kafka.strimzi.io/v1alpha1`:
- `KafkaTopic`
- `KafkaUser`

apiVersion `kafka.strimzi.io/v1beta1`:
- `KafkaTopic`
- `KafkaUser`

apiVersion `kafka.strimzi.io/v1beta2`:
- `KafkaBridge`
- `KafkaConnector`
- `KafkaConnect`
- `KafkaMirrorMaker`
- `KafkaRebalance`
- `Kafka`
- `KafkaTopic`
- `KafkaUser`

## kamaji_clastix_io

apiVersion `kamaji.clastix.io/v1alpha1`:
- `DataStore`
- `TenantControlPlane`

## karpenter_k8s_aws

apiVersion `karpenter.k8s.aws/v1`:
- `EC2NodeClass`

apiVersion `karpenter.k8s.aws/v1beta1`:
- `EC2NodeClass`

## karpenter_sh

apiVersion `karpenter.sh/v1`:
- `NodeClaim`
- `NodePool`

apiVersion `karpenter.sh/v1beta1`:
- `NodeClaim`
- `NodePool`

## keda_sh

apiVersion `keda.sh/v1alpha1`:
- `ClusterTriggerAuthentication`
- `ScaledJob`
- `ScaledObject`
- `TriggerAuthentication`

## keycloak_k8s_reddec_net

apiVersion `keycloak.k8s.reddec.net/v1alpha1`:
- `KeycloakClient`

## keycloak_org

apiVersion `keycloak.org/v1alpha1`:
- `KeycloakBackup`
- `KeycloakClient`
- `KeycloakRealm`
- `Keycloak`
- `KeycloakUser`

## keyspaces_services_k8s_aws

apiVersion `keyspaces.services.k8s.aws/v1alpha1`:
- `Keyspace`
- `Table`

## kibana_k8s_elastic_co

apiVersion `kibana.k8s.elastic.co/v1`:
- `Kibana`

apiVersion `kibana.k8s.elastic.co/v1beta1`:
- `Kibana`

## kinesis_services_k8s_aws

apiVersion `kinesis.services.k8s.aws/v1alpha1`:
- `Stream`

## kmm_sigs_x_k8s_io

apiVersion `kmm.sigs.x-k8s.io/v1beta1`:
- `Module`
- `NodeModulesConfig`
- `PreflightValidation`

apiVersion `kmm.sigs.x-k8s.io/v1beta2`:
- `PreflightValidation`

## kms_services_k8s_aws

apiVersion `kms.services.k8s.aws/v1alpha1`:
- `Alias`
- `Grant`
- `Key`

## kuadrant_io

apiVersion `kuadrant.io/v1alpha1`:
- `DNSRecord`
- `ManagedZone`

apiVersion `kuadrant.io/v1beta1`:
- `Kuadrant`

apiVersion `kuadrant.io/v1beta2`:
- `RateLimitPolicy`

## kube_green_com

apiVersion `kube-green.com/v1alpha1`:
- `SleepInfo`

## kubean_io

apiVersion `kubean.io/v1alpha1`:
- `ClusterOperation`
- `Cluster`
- `LocalArtifactSet`
- `Manifest`

## kubecost_com

apiVersion `kubecost.com/v1alpha1`:
- `TurndownSchedule`

## kubevious_io

apiVersion `kubevious.io/v1alpha1`:
- `WorkloadProfile`
- `Workload`

## kueue_x_k8s_io

apiVersion `kueue.x-k8s.io/v1beta1`:
- `AdmissionCheck`
- `ClusterQueue`
- `LocalQueue`
- `ResourceFlavor`
- `Workload`

## kuma_io

apiVersion `kuma.io/v1alpha1`:
- `CircuitBreaker`
- `ContainerPatch`
- `DataplaneInsight`
- `Dataplane`
- `ExternalService`
- `FaultInjection`
- `HealthCheck`
- `MeshAccessLog`
- `MeshCircuitBreaker`
- `Mesh`
- `MeshFaultInjection`
- `MeshGatewayConfig`
- `MeshGatewayInstance`
- `MeshGatewayRoute`
- `MeshGateway`
- `MeshHealthCheck`
- `MeshHTTPRoute`
- `MeshInsight`
- `MeshLoadBalancingStrategy`
- `MeshProxyPatch`
- `MeshRateLimit`
- `MeshRetry`
- `MeshTCPRoute`
- `MeshTimeout`
- `MeshTrace`
- `MeshTrafficPermission`
- `ProxyTemplate`
- `RateLimit`
- `Retry`
- `ServiceInsight`
- `Timeout`
- `TrafficLog`
- `TrafficPermission`
- `TrafficRoute`
- `TrafficTrace`
- `VirtualOutbound`
- `ZoneEgress`
- `ZoneEgressInsight`
- `ZoneIngress`
- `ZoneIngressInsight`
- `ZoneInsight`
- `Zone`

## kustomize_toolkit_fluxcd_io

apiVersion `kustomize.toolkit.fluxcd.io/v1`:
- `Kustomization`

apiVersion `kustomize.toolkit.fluxcd.io/v1beta1`:
- `Kustomization`

apiVersion `kustomize.toolkit.fluxcd.io/v1beta2`:
- `Kustomization`

## kyverno_io

apiVersion `kyverno.io/v1`:
- `ClusterPolicy`
- `Policy`

apiVersion `kyverno.io/v1alpha2`:
- `AdmissionReport`
- `BackgroundScanReport`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`

apiVersion `kyverno.io/v1beta1`:
- `UpdateRequest`

apiVersion `kyverno.io/v2`:
- `AdmissionReport`
- `BackgroundScanReport`
- `CleanupPolicy`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`
- `ClusterCleanupPolicy`
- `PolicyException`
- `UpdateRequest`

apiVersion `kyverno.io/v2alpha1`:
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `GlobalContextEntry`
- `PolicyException`

apiVersion `kyverno.io/v2beta1`:
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `ClusterPolicy`
- `Policy`
- `PolicyException`

## lambda_services_k8s_aws

apiVersion `lambda.services.k8s.aws/v1alpha1`:
- `CodeSigningConfig`
- `EventSourceMapping`
- `Function`
- `FunctionURLConfig`
- `LayerVersion`
- `Version`

## leaksignal_com

apiVersion `leaksignal.com/v1`:
- `ClusterLeaksignalIstio`
- `LeaksignalIstio`

## lerentis_uploadfilter24_eu

apiVersion `lerentis.uploadfilter24.eu/v1beta4`:
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`

apiVersion `lerentis.uploadfilter24.eu/v1beta5`:
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`

apiVersion `lerentis.uploadfilter24.eu/v1beta6`:
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`

apiVersion `lerentis.uploadfilter24.eu/v1beta7`:
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`

## limitador_kuadrant_io

apiVersion `limitador.kuadrant.io/v1alpha1`:
- `Limitador`

## litmuschaos_io

apiVersion `litmuschaos.io/v1alpha1`:
- `ChaosEngine`
- `ChaosExperiment`
- `ChaosResult`

## logging_extensions_banzaicloud_io

apiVersion `logging-extensions.banzaicloud.io/v1alpha1`:
- `HostTailer`

## logging_banzaicloud_io

apiVersion `logging.banzaicloud.io/v1alpha1`:
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Logging`
- `Output`

apiVersion `logging.banzaicloud.io/v1beta1`:
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Output`
- `SyslogNGClusterFlow`
- `SyslogNGClusterOutput`
- `SyslogNGFlow`
- `SyslogNGOutput`

## loki_grafana_com

apiVersion `loki.grafana.com/v1`:
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`

apiVersion `loki.grafana.com/v1beta1`:
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`

## longhorn_io

apiVersion `longhorn.io/v1beta1`:
- `BackingImageDataSource`
- `BackingImageManager`
- `BackingImage`
- `Backup`
- `BackupTarget`
- `BackupVolume`
- `EngineImage`
- `Engine`
- `InstanceManager`
- `Node`
- `RecurringJob`
- `Replica`
- `Setting`
- `ShareManager`
- `Volume`

apiVersion `longhorn.io/v1beta2`:
- `BackingImageDataSource`
- `BackingImageManager`
- `BackingImage`
- `BackupBackingImage`
- `Backup`
- `BackupTarget`
- `BackupVolume`
- `EngineImage`
- `Engine`
- `InstanceManager`
- `Node`
- `Orphan`
- `RecurringJob`
- `Replica`
- `Setting`
- `ShareManager`
- `Snapshot`
- `SupportBundle`
- `SystemBackup`
- `SystemRestore`
- `VolumeAttachment`
- `Volume`

## m4e_krestomat_io

apiVersion `m4e.krestomat.io/v1alpha1`:
- `Moodle`
- `Nginx`
- `Phpfpm`
- `Routine`

## machine_deletion_remediation_medik8s_io

apiVersion `machine-deletion-remediation.medik8s.io/v1alpha1`:
- `MachineDeletionRemediation`
- `MachineDeletionRemediationTemplate`

## maps_k8s_elastic_co

apiVersion `maps.k8s.elastic.co/v1alpha1`:
- `ElasticMapsServer`

## mariadb_mmontes_io

apiVersion `mariadb.mmontes.io/v1alpha1`:
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `Restore`
- `SqlJob`
- `User`

## marin3r_3scale_net

apiVersion `marin3r.3scale.net/v1alpha1`:
- `EnvoyConfigRevision`
- `EnvoyConfig`

## mattermost_com

apiVersion `mattermost.com/v1alpha1`:
- `ClusterInstallation`
- `MattermostRestoreDB`

## memorydb_services_k8s_aws

apiVersion `memorydb.services.k8s.aws/v1alpha1`:
- `ACL`
- `Cluster`
- `ParameterGroup`
- `Snapshot`
- `SubnetGroup`
- `User`

## metacontroller_k8s_io

apiVersion `metacontroller.k8s.io/v1alpha1`:
- `CompositeController`
- `ControllerRevision`
- `DecoratorController`

## metal3_io

apiVersion `metal3.io/v1alpha1`:
- `BMCEventSubscription`
- `DataImage`
- `FirmwareSchema`
- `HardwareData`
- `HostFirmwareComponents`
- `HostFirmwareSettings`
- `PreprovisioningImage`

## microcks_github_io

apiVersion `microcks.github.io/v1alpha1`:
- `MicrocksInstall`

## minio_min_io

apiVersion `minio.min.io/v2`:
- `Tenant`

## mirrors_kts_studio

apiVersion `mirrors.kts.studio/v1alpha1`:
- `SecretMirror`

apiVersion `mirrors.kts.studio/v1alpha2`:
- `SecretMirror`

## model_kubedl_io

apiVersion `model.kubedl.io/v1alpha1`:
- `Model`
- `ModelVersion`

## monitoring_coreos_com

apiVersion `monitoring.coreos.com/v1`:
- `Alertmanager`
- `PodMonitor`
- `Probe`
- `Prometheus`
- `PrometheusRule`
- `ServiceMonitor`
- `ThanosRuler`

apiVersion `monitoring.coreos.com/v1alpha1`:
- `AlertmanagerConfig`
- `PrometheusAgent`
- `ScrapeConfig`

apiVersion `monitoring.coreos.com/v1beta1`:
- `AlertmanagerConfig`

## monocle_monocle_change_metrics_io

apiVersion `monocle.monocle.change-metrics.io/v1alpha1`:
- `Monocle`

## mq_services_k8s_aws

apiVersion `mq.services.k8s.aws/v1alpha1`:
- `Broker`

## multicluster_crd_antrea_io

apiVersion `multicluster.crd.antrea.io/v1alpha1`:
- `ClusterInfoImport`
- `ClusterSet`
- `Gateway`
- `LabelIdentity`
- `MemberClusterAnnounce`
- `MultiClusterConfig`
- `ResourceExport`
- `ResourceImport`

apiVersion `multicluster.crd.antrea.io/v1alpha2`:
- `ClusterClaim`
- `ClusterSet`

## multicluster_x_k8s_io

apiVersion `multicluster.x-k8s.io/v1alpha1`:
- `ServiceExport`
- `ServiceImport`
- `AppliedWork`

## mutations_gatekeeper_sh

apiVersion `mutations.gatekeeper.sh/v1`:
- `Assign`
- `AssignMetadata`
- `ModifySet`

apiVersion `mutations.gatekeeper.sh/v1alpha1`:
- `Assign`
- `AssignImage`
- `AssignMetadata`
- `ModifySet`

apiVersion `mutations.gatekeeper.sh/v1beta1`:
- `Assign`
- `AssignMetadata`
- `ModifySet`

## nativestor_alauda_io

apiVersion `nativestor.alauda.io/v1`:
- `RawDevice`

## netchecks_io

apiVersion `netchecks.io/v1`:
- `NetworkAssertion`

## networkfirewall_services_k8s_aws

apiVersion `networkfirewall.services.k8s.aws/v1alpha1`:
- `FirewallPolicy`
- `Firewall`
- `RuleGroup`

## networking_gke_io

apiVersion `networking.gke.io/v1`:
- `ManagedCertificate`
- `GCPBackendPolicy`
- `GCPGatewayPolicy`
- `HealthCheckPolicy`
- `LBPolicy`

## networking_istio_io

apiVersion `networking.istio.io/v1`:
- `DestinationRule`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`

apiVersion `networking.istio.io/v1alpha3`:
- `DestinationRule`
- `EnvoyFilter`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`

apiVersion `networking.istio.io/v1beta1`:
- `DestinationRule`
- `Gateway`
- `ProxyConfig`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`

## networking_k8s_aws

apiVersion `networking.k8s.aws/v1alpha1`:
- `PolicyEndpoint`

## networking_karmada_io

apiVersion `networking.karmada.io/v1alpha1`:
- `MultiClusterIngress`
- `MultiClusterService`

## nfd_k8s_sigs_io

apiVersion `nfd.k8s-sigs.io/v1alpha1`:
- `NodeFeatureRule`

## nfd_kubernetes_io

apiVersion `nfd.kubernetes.io/v1`:
- `NodeFeatureDiscovery`

apiVersion `nfd.kubernetes.io/v1alpha1`:
- `NodeFeatureRule`

## nodeinfo_volcano_sh

apiVersion `nodeinfo.volcano.sh/v1alpha1`:
- `Numatopology`

## notebook_kubedl_io

apiVersion `notebook.kubedl.io/v1alpha1`:
- `Notebook`

## notification_toolkit_fluxcd_io

apiVersion `notification.toolkit.fluxcd.io/v1`:
- `Receiver`

apiVersion `notification.toolkit.fluxcd.io/v1beta1`:
- `Alert`
- `Provider`
- `Receiver`

apiVersion `notification.toolkit.fluxcd.io/v1beta2`:
- `Alert`
- `Provider`
- `Receiver`

apiVersion `notification.toolkit.fluxcd.io/v1beta3`:
- `Alert`
- `Provider`

## objectbucket_io

apiVersion `objectbucket.io/v1alpha1`:
- `ObjectBucketClaim`
- `ObjectBucket`

## onepassword_com

apiVersion `onepassword.com/v1`:
- `OnePasswordItem`

## opensearchservice_services_k8s_aws

apiVersion `opensearchservice.services.k8s.aws/v1alpha1`:
- `Domain`

## opentelemetry_io

apiVersion `opentelemetry.io/v1alpha1`:
- `Instrumentation`
- `OpAMPBridge`
- `OpenTelemetryCollector`

apiVersion `opentelemetry.io/v1beta1`:
- `OpenTelemetryCollector`

## operations_kubeedge_io

apiVersion `operations.kubeedge.io/v1alpha1`:
- `NodeUpgradeJob`

## operator_aquasec_com

apiVersion `operator.aquasec.com/v1alpha1`:
- `AquaCsp`
- `AquaDatabase`
- `AquaEnforcer`
- `AquaGateway`
- `AquaKubeEnforcer`
- `AquaScanner`
- `AquaServer`

## operator_authorino_kuadrant_io

apiVersion `operator.authorino.kuadrant.io/v1beta1`:
- `Authorino`

## operator_cluster_x_k8s_io

apiVersion `operator.cluster.x-k8s.io/v1alpha1`:
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`

apiVersion `operator.cluster.x-k8s.io/v1alpha2`:
- `AddonProvider`
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`

## operator_cryostat_io

apiVersion `operator.cryostat.io/v1beta1`:
- `Cryostat`

apiVersion `operator.cryostat.io/v1beta2`:
- `Cryostat`

## operator_marin3r_3scale_net

apiVersion `operator.marin3r.3scale.net/v1alpha1`:
- `DiscoveryServiceCertificate`
- `DiscoveryService`
- `EnvoyDeployment`

## operator_open_cluster_management_io

apiVersion `operator.open-cluster-management.io/v1`:
- `ClusterManager`
- `Klusterlet`

## operator_shipwright_io

apiVersion `operator.shipwright.io/v1alpha1`:
- `ShipwrightBuild`

## operator_tekton_dev

apiVersion `operator.tekton.dev/v1alpha1`:
- `TektonChain`
- `TektonConfig`
- `TektonHub`
- `TektonInstallerSet`
- `TektonPipeline`
- `TektonTrigger`

## operator_tigera_io

apiVersion `operator.tigera.io/v1`:
- `AmazonCloudIntegration`
- `APIServer`
- `ApplicationLayer`
- `Authentication`
- `Compliance`
- `EgressGateway`
- `ImageSet`
- `Installation`
- `IntrusionDetection`
- `LogCollector`
- `LogStorage`
- `ManagementClusterConnection`
- `ManagementCluster`
- `Manager`
- `Monitor`
- `PacketCapture`
- `PolicyRecommendation`
- `Tenant`
- `TigeraStatus`
- `TLSPassThroughRoute`
- `TLSTerminatedRoute`

apiVersion `operator.tigera.io/v1beta1`:
- `AmazonCloudIntegration`

## operator_victoriametrics_com

apiVersion `operator.victoriametrics.com/v1beta1`:
- `VMAlertmanagerConfig`
- `VMNodeScrape`
- `VMPodScrape`
- `VMProbe`
- `VMRule`
- `VMServiceScrape`
- `VMStaticScrape`
- `VMUser`

## oracle_db_anthosapis_com

apiVersion `oracle.db.anthosapis.com/v1alpha1`:
- `Backup`
- `BackupSchedule`
- `Config`
- `CronAnything`
- `Database`
- `Export`
- `Import`
- `PITR`
- `Release`

## org_eclipse_che

apiVersion `org.eclipse.che/v1`:
- `CheCluster`

apiVersion `org.eclipse.che/v2`:
- `CheCluster`

## organizations_services_k8s_aws

apiVersion `organizations.services.k8s.aws/v1alpha1`:
- `OrganizationalUnit`

## pgv2_percona_com

apiVersion `pgv2.percona.com/v2`:
- `PerconaPGBackup`
- `PerconaPGCluster`
- `PerconaPGRestore`
- `PerconaPGUpgrade`

## pipes_services_k8s_aws

apiVersion `pipes.services.k8s.aws/v1alpha1`:
- `Pipe`

## pkg_crossplane_io

apiVersion `pkg.crossplane.io/v1`:
- `ConfigurationRevision`
- `Configuration`
- `ProviderRevision`
- `Provider`

apiVersion `pkg.crossplane.io/v1alpha1`:
- `ControllerConfig`

apiVersion `pkg.crossplane.io/v1beta1`:
- `Lock`

## policy_clusterpedia_io

apiVersion `policy.clusterpedia.io/v1alpha1`:
- `ClusterImportPolicy`
- `PediaClusterLifecycle`

## policy_karmada_io

apiVersion `policy.karmada.io/v1alpha1`:
- `ClusterOverridePolicy`
- `ClusterPropagationPolicy`
- `FederatedResourceQuota`
- `OverridePolicy`
- `PropagationPolicy`

## policy_kubeedge_io

apiVersion `policy.kubeedge.io/v1alpha1`:
- `ServiceAccountAccess`

## policy_networking_k8s_io

apiVersion `policy.networking.k8s.io/v1alpha1`:
- `AdminNetworkPolicy`
- `BaselineAdminNetworkPolicy`

## postgres_operator_crunchydata_com

apiVersion `postgres-operator.crunchydata.com/v1beta1`:
- `PGAdmin`
- `PGUpgrade`
- `PostgresCluster`

## postgresql_cnpg_io

apiVersion `postgresql.cnpg.io/v1`:
- `Backup`
- `Pooler`
- `ScheduledBackup`

## projectcontour_io

apiVersion `projectcontour.io/v1`:
- `HTTPProxy`
- `TLSCertificateDelegation`

apiVersion `projectcontour.io/v1alpha1`:
- `ContourConfiguration`
- `ContourDeployment`
- `ExtensionService`

## prometheusservice_services_k8s_aws

apiVersion `prometheusservice.services.k8s.aws/v1alpha1`:
- `AlertManagerDefinition`
- `LoggingConfiguration`
- `RuleGroupsNamespace`
- `Workspace`

## ps_percona_com

apiVersion `ps.percona.com/v1alpha1`:
- `PerconaServerMySQLBackup`
- `PerconaServerMySQLRestore`
- `PerconaServerMySQL`

## psmdb_percona_com

apiVersion `psmdb.percona.com/v1`:
- `PerconaServerMongoDBBackup`
- `PerconaServerMongoDBRestore`

## ptp_openshift_io

apiVersion `ptp.openshift.io/v1`:
- `NodePtpDevice`

## pxc_percona_com

apiVersion `pxc.percona.com/v1`:
- `PerconaXtraDBClusterBackup`
- `PerconaXtraDBClusterRestore`
- `PerconaXtraDBCluster`

## quay_redhat_com

apiVersion `quay.redhat.com/v1`:
- `QuayRegistry`

## quota_codeflare_dev

apiVersion `quota.codeflare.dev/v1alpha1`:
- `QuotaSubtree`

## ray_io

apiVersion `ray.io/v1`:
- `RayCluster`
- `RayJob`
- `RayService`

apiVersion `ray.io/v1alpha1`:
- `RayCluster`
- `RayJob`
- `RayService`

## rbacmanager_reactiveops_io

apiVersion `rbacmanager.reactiveops.io/v1beta1`:
- `RBACDefinition`

## rc_app_stacks

apiVersion `rc.app.stacks/v1`:
- `RuntimeComponent`
- `RuntimeOperation`

apiVersion `rc.app.stacks/v1beta2`:
- `RuntimeComponent`
- `RuntimeOperation`

## rds_services_k8s_aws

apiVersion `rds.services.k8s.aws/v1alpha1`:
- `DBClusterParameterGroup`
- `DBCluster`
- `DBInstance`
- `DBParameterGroup`
- `DBProxy`
- `DBSubnetGroup`
- `GlobalCluster`

## redhatcop_redhat_io

apiVersion `redhatcop.redhat.io/v1alpha1`:
- `KeepalivedGroup`

## registry_apicur_io

apiVersion `registry.apicur.io/v1`:
- `ApicurioRegistry`

## registry_devfile_io

apiVersion `registry.devfile.io/v1alpha1`:
- `ClusterDevfileRegistriesList`
- `DevfileRegistry`
- `DevfileRegistriesList`

## reliablesyncs_kubeedge_io

apiVersion `reliablesyncs.kubeedge.io/v1alpha1`:
- `ClusterObjectSync`
- `ObjectSync`

## remediation_medik8s_io

apiVersion `remediation.medik8s.io/v1alpha1`:
- `NodeHealthCheck`

## repo_manager_pulpproject_org

apiVersion `repo-manager.pulpproject.org/v1beta2`:
- `PulpBackup`
- `PulpRestore`
- `Pulp`

## reports_kyverno_io

apiVersion `reports.kyverno.io/v1`:
- `ClusterEphemeralReport`
- `EphemeralReport`

## resources_teleport_dev

apiVersion `resources.teleport.dev/v1`:
- `TeleportLoginRule`
- `TeleportOktaImportRule`

apiVersion `resources.teleport.dev/v2`:
- `TeleportSAMLConnector`
- `TeleportUser`

apiVersion `resources.teleport.dev/v3`:
- `TeleportGithubConnector`
- `TeleportOIDCConnector`

## rocketmq_apache_org

apiVersion `rocketmq.apache.org/v1alpha1`:
- `Broker`
- `Console`
- `NameService`
- `TopicTransfer`

## route53_services_k8s_aws

apiVersion `route53.services.k8s.aws/v1alpha1`:
- `HostedZone`
- `RecordSet`

## route53resolver_services_k8s_aws

apiVersion `route53resolver.services.k8s.aws/v1alpha1`:
- `ResolverEndpoint`
- `ResolverRule`

## rules_kubeedge_io

apiVersion `rules.kubeedge.io/v1`:
- `RuleEndpoint`
- `Rule`

## runtime_cluster_x_k8s_io

apiVersion `runtime.cluster.x-k8s.io/v1alpha1`:
- `ExtensionConfig`

## s3_services_k8s_aws

apiVersion `s3.services.k8s.aws/v1alpha1`:
- `Bucket`

## s3_snappcloud_io

apiVersion `s3.snappcloud.io/v1alpha1`:
- `S3Bucket`
- `S3UserClaim`
- `S3User`

## sagemaker_services_k8s_aws

apiVersion `sagemaker.services.k8s.aws/v1alpha1`:
- `App`
- `DataQualityJobDefinition`
- `Domain`
- `EndpointConfig`
- `Endpoint`
- `FeatureGroup`
- `HyperParameterTuningJob`
- `ModelBiasJobDefinition`
- `ModelExplainabilityJobDefinition`
- `ModelPackageGroup`
- `ModelPackage`
- `ModelQualityJobDefinition`
- `Model`
- `MonitoringSchedule`
- `NotebookInstanceLifecycleConfig`
- `NotebookInstance`
- `ProcessingJob`
- `TrainingJob`
- `TransformJob`
- `UserProfile`

## scheduling_koordinator_sh

apiVersion `scheduling.koordinator.sh/v1alpha1`:
- `Device`
- `PodMigrationJob`
- `Reservation`

## scheduling_sigs_k8s_io

apiVersion `scheduling.sigs.k8s.io/v1alpha1`:
- `ElasticQuota`
- `PodGroup`

## scheduling_volcano_sh

apiVersion `scheduling.volcano.sh/v1beta1`:
- `PodGroup`
- `Queue`

## schemas_schemahero_io

apiVersion `schemas.schemahero.io/v1alpha4`:
- `DataType`
- `Migration`
- `Table`

## scylla_scylladb_com

apiVersion `scylla.scylladb.com/v1`:
- `ScyllaCluster`

apiVersion `scylla.scylladb.com/v1alpha1`:
- `NodeConfig`
- `ScyllaOperatorConfig`

## secretgenerator_mittwald_de

apiVersion `secretgenerator.mittwald.de/v1alpha1`:
- `BasicAuth`
- `SSHKeyPair`
- `StringSecret`

## secrets_store_csi_x_k8s_io

apiVersion `secrets-store.csi.x-k8s.io/v1`:
- `SecretProviderClass`
- `SecretProviderClassPodStatus`

apiVersion `secrets-store.csi.x-k8s.io/v1alpha1`:
- `SecretProviderClass`
- `SecretProviderClassPodStatus`

## secrets_crossplane_io

apiVersion `secrets.crossplane.io/v1alpha1`:
- `StoreConfig`

## secrets_doppler_com

apiVersion `secrets.doppler.com/v1alpha1`:
- `DopplerSecret`

## secrets_hashicorp_com

apiVersion `secrets.hashicorp.com/v1beta1`:
- `HCPAuth`
- `HCPVaultSecretsApp`
- `VaultAuth`
- `VaultConnection`
- `VaultDynamicSecret`
- `VaultPKISecret`
- `VaultStaticSecret`

## secretsmanager_services_k8s_aws

apiVersion `secretsmanager.services.k8s.aws/v1alpha1`:
- `Secret`

## secscan_quay_redhat_com

apiVersion `secscan.quay.redhat.com/v1alpha1`:
- `ImageManifestVuln`

## security_profiles_operator_x_k8s_io

apiVersion `security-profiles-operator.x-k8s.io/v1alpha1`:
- `AppArmorProfile`
- `ProfileBinding`
- `ProfileRecording`
- `SecurityProfileNodeStatus`
- `SecurityProfilesOperatorDaemon`

apiVersion `security-profiles-operator.x-k8s.io/v1alpha2`:
- `RawSelinuxProfile`

apiVersion `security-profiles-operator.x-k8s.io/v1beta1`:
- `SeccompProfile`

## security_istio_io

apiVersion `security.istio.io/v1`:
- `AuthorizationPolicy`
- `PeerAuthentication`
- `RequestAuthentication`

apiVersion `security.istio.io/v1beta1`:
- `AuthorizationPolicy`
- `PeerAuthentication`
- `RequestAuthentication`

## self_node_remediation_medik8s_io

apiVersion `self-node-remediation.medik8s.io/v1alpha1`:
- `SelfNodeRemediationConfig`
- `SelfNodeRemediation`
- `SelfNodeRemediationTemplate`

## sematext_com

apiVersion `sematext.com/v1`:
- `SematextAgent`

## servicebinding_io

apiVersion `servicebinding.io/v1alpha3`:
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`

apiVersion `servicebinding.io/v1beta1`:
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`

## servicemesh_cisco_com

apiVersion `servicemesh.cisco.com/v1alpha1`:
- `IstioControlPlane`
- `IstioMesh`
- `IstioMeshGateway`
- `PeerIstioControlPlane`

## services_k8s_aws

apiVersion `services.k8s.aws/v1alpha1`:
- `AdoptedResource`
- `FieldExport`

## serving_kubedl_io

apiVersion `serving.kubedl.io/v1alpha1`:
- `Inference`

## sfn_services_k8s_aws

apiVersion `sfn.services.k8s.aws/v1alpha1`:
- `Activity`
- `StateMachine`

## site_superedge_io

apiVersion `site.superedge.io/v1alpha1`:
- `NodeGroup`
- `NodeUnit`

## slo_koordinator_sh

apiVersion `slo.koordinator.sh/v1alpha1`:
- `NodeMetric`
- `NodeSLO`

## sloth_slok_dev

apiVersion `sloth.slok.dev/v1`:
- `PrometheusServiceLevel`

## snapscheduler_backube

apiVersion `snapscheduler.backube/v1`:
- `SnapshotSchedule`

## snapshot_storage_k8s_io

apiVersion `snapshot.storage.k8s.io/v1`:
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`

apiVersion `snapshot.storage.k8s.io/v1beta1`:
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`

## sns_services_k8s_aws

apiVersion `sns.services.k8s.aws/v1alpha1`:
- `PlatformApplication`
- `PlatformEndpoint`
- `Subscription`
- `Topic`

## sonataflow_org

apiVersion `sonataflow.org/v1alpha08`:
- `SonataFlowBuild`
- `SonataFlowPlatform`

## source_toolkit_fluxcd_io

apiVersion `source.toolkit.fluxcd.io/v1`:
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`

apiVersion `source.toolkit.fluxcd.io/v1beta1`:
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`

apiVersion `source.toolkit.fluxcd.io/v1beta2`:
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`
- `OCIRepository`

## sparkoperator_k8s_io

apiVersion `sparkoperator.k8s.io/v1beta2`:
- `ScheduledSparkApplication`
- `SparkApplication`

## spv_no

apiVersion `spv.no/v1`:
- `AzureKeyVaultSecret`

apiVersion `spv.no/v1alpha1`:
- `AzureKeyVaultIdentity`
- `AzureKeyVaultSecret`
- `AzureManagedIdentity`

apiVersion `spv.no/v2alpha1`:
- `AzureKeyVaultSecret`

apiVersion `spv.no/v2beta1`:
- `AzureKeyVaultSecret`

## sqs_services_k8s_aws

apiVersion `sqs.services.k8s.aws/v1alpha1`:
- `Queue`

## status_gatekeeper_sh

apiVersion `status.gatekeeper.sh/v1beta1`:
- `ConstraintPodStatus`
- `ConstraintTemplatePodStatus`
- `ExpansionTemplatePodStatus`
- `MutatorPodStatus`

## storage_kubeblocks_io

apiVersion `storage.kubeblocks.io/v1alpha1`:
- `StorageProvider`

## storageos_com

apiVersion `storageos.com/v1`:
- `StorageOSCluster`

## sts_min_io

apiVersion `sts.min.io/v1alpha1`:
- `PolicyBinding`

apiVersion `sts.min.io/v1beta1`:
- `PolicyBinding`

## stunner_l7mp_io

apiVersion `stunner.l7mp.io/v1`:
- `Dataplane`
- `GatewayConfig`
- `StaticService`
- `UDPRoute`

apiVersion `stunner.l7mp.io/v1alpha1`:
- `Dataplane`
- `GatewayConfig`
- `StaticService`

## submariner_io

apiVersion `submariner.io/v1alpha1`:
- `Broker`
- `ServiceDiscovery`
- `Submariner`

## telemetry_istio_io

apiVersion `telemetry.istio.io/v1`:
- `Telemetry`

apiVersion `telemetry.istio.io/v1alpha1`:
- `Telemetry`

## templates_gatekeeper_sh

apiVersion `templates.gatekeeper.sh/v1`:
- `ConstraintTemplate`

apiVersion `templates.gatekeeper.sh/v1alpha1`:
- `ConstraintTemplate`

apiVersion `templates.gatekeeper.sh/v1beta1`:
- `ConstraintTemplate`

## tempo_grafana_com

apiVersion `tempo.grafana.com/v1alpha1`:
- `TempoMonolithic`
- `TempoStack`

## temporal_io

apiVersion `temporal.io/v1beta1`:
- `TemporalClusterClient`
- `TemporalNamespace`
- `TemporalWorkerProcess`

## tests_testkube_io

apiVersion `tests.testkube.io/v1`:
- `Script`
- `TestExecution`
- `Test`
- `TestSource`
- `TestSuiteExecution`
- `TestSuite`
- `TestTrigger`

apiVersion `tests.testkube.io/v2`:
- `Script`
- `Test`
- `TestSuite`

apiVersion `tests.testkube.io/v3`:
- `Test`
- `TestSuite`

## tf_tungsten_io

apiVersion `tf.tungsten.io/v1alpha1`:
- `Analytics`
- `AnalyticsAlarm`
- `AnalyticsSnmp`
- `Cassandra`
- `Config`
- `Control`
- `Kubemanager`
- `Manager`
- `QueryEngine`
- `Rabbitmq`
- `Redis`
- `Vrouter`
- `Zookeeper`

## theketch_io

apiVersion `theketch.io/v1beta1`:
- `App`
- `Job`

## tinkerbell_org

apiVersion `tinkerbell.org/v1alpha1`:
- `Stack`
- `Hardware`
- `OSIE`
- `Template`
- `Workflow`

apiVersion `tinkerbell.org/v1alpha2`:
- `Hardware`
- `OSIE`
- `Template`
- `Workflow`

## topology_node_k8s_io

apiVersion `topology.node.k8s.io/v1alpha1`:
- `NodeResourceTopology`

## topolvm_cybozu_com

apiVersion `topolvm.cybozu.com/v1`:
- `LogicalVolume`

apiVersion `topolvm.cybozu.com/v2`:
- `TopolvmCluster`

## traefik_io

apiVersion `traefik.io/v1alpha1`:
- `IngressRoute`
- `IngressRouteTCP`
- `IngressRouteUDP`
- `MiddlewareTCP`
- `ServersTransport`
- `ServersTransportTCP`
- `TLSOption`
- `TLSStore`
- `TraefikService`

## training_kubedl_io

apiVersion `training.kubedl.io/v1alpha1`:
- `ElasticDLJob`
- `MarsJob`
- `MPIJob`
- `PyTorchJob`
- `TFJob`
- `XDLJob`
- `XGBoostJob`

## trident_netapp_io

apiVersion `trident.netapp.io/v1`:
- `TridentOrchestrator`

## trust_cert_manager_io

apiVersion `trust.cert-manager.io/v1alpha1`:
- `Bundle`

## upgrade_cattle_io

apiVersion `upgrade.cattle.io/v1`:
- `Plan`

## upgrade_managed_openshift_io

apiVersion `upgrade.managed.openshift.io/v1alpha1`:
- `UpgradeConfig`

## velero_io

apiVersion `velero.io/v1`:
- `BackupRepository`
- `Backup`
- `BackupStorageLocation`
- `DeleteBackupRequest`
- `DownloadRequest`
- `PodVolumeBackup`
- `PodVolumeRestore`
- `Schedule`
- `ServerStatusRequest`
- `VolumeSnapshotLocation`

apiVersion `velero.io/v2alpha1`:
- `DataDownload`
- `DataUpload`

## virt_virtink_smartx_com

apiVersion `virt.virtink.smartx.com/v1alpha1`:
- `VirtualMachineMigration`
- `VirtualMachine`

## volsync_backube

apiVersion `volsync.backube/v1alpha1`:
- `ReplicationDestination`
- `ReplicationSource`

## vpcresources_k8s_aws

apiVersion `vpcresources.k8s.aws/v1alpha1`:
- `CNINode`

apiVersion `vpcresources.k8s.aws/v1beta1`:
- `SecurityGroupPolicy`

## wgpolicyk8s_io

apiVersion `wgpolicyk8s.io/v1alpha1`:
- `ClusterPolicyReport`
- `PolicyReport`

apiVersion `wgpolicyk8s.io/v1alpha2`:
- `ClusterPolicyReport`
- `PolicyReport`

apiVersion `wgpolicyk8s.io/v1beta1`:
- `ClusterPolicyReport`
- `PolicyReport`

## wildfly_org

apiVersion `wildfly.org/v1alpha1`:
- `WildFlyServer`

## work_karmada_io

apiVersion `work.karmada.io/v1alpha1`:
- `ClusterResourceBinding`
- `ResourceBinding`

apiVersion `work.karmada.io/v1alpha2`:
- `ClusterResourceBinding`
- `ResourceBinding`

## workload_codeflare_dev

apiVersion `workload.codeflare.dev/v1beta1`:
- `AppWrapper`
- `SchedulingSpec`

apiVersion `workload.codeflare.dev/v1beta2`:
- `AppWrapper`

## workloads_kubeblocks_io

apiVersion `workloads.kubeblocks.io/v1alpha1`:
- `InstanceSet`
- `ReplicatedStateMachine`

## zonecontrol_k8s_aws

apiVersion `zonecontrol.k8s.aws/v1`:
- `ZoneAwareUpdate`
- `ZoneDisruptionBudget`

## zookeeper_pravega_io

apiVersion `zookeeper.pravega.io/v1beta1`:
- `ZookeeperCluster`
 */

#[cfg(feature = "about_k8s_io")]
pub mod about_k8s_io;
#[cfg(feature = "acid_zalan_do")]
pub mod acid_zalan_do;
#[cfg(feature = "acme_cert_manager_io")]
pub mod acme_cert_manager_io;
#[cfg(feature = "acmpca_services_k8s_aws")]
pub mod acmpca_services_k8s_aws;
#[cfg(feature = "actions_github_com")]
pub mod actions_github_com;
#[cfg(feature = "actions_summerwind_dev")]
pub mod actions_summerwind_dev;
#[cfg(feature = "addons_cluster_x_k8s_io")]
pub mod addons_cluster_x_k8s_io;
#[cfg(feature = "agent_k8s_elastic_co")]
pub mod agent_k8s_elastic_co;
#[cfg(feature = "anywhere_eks_amazonaws_com")]
pub mod anywhere_eks_amazonaws_com;
#[cfg(feature = "apacheweb_arsenal_dev")]
pub mod apacheweb_arsenal_dev;
#[cfg(feature = "api_clever_cloud_com")]
pub mod api_clever_cloud_com;
#[cfg(feature = "api_kubemod_io")]
pub mod api_kubemod_io;
#[cfg(feature = "apicodegen_apimatic_io")]
pub mod apicodegen_apimatic_io;
#[cfg(feature = "apiextensions_crossplane_io")]
pub mod apiextensions_crossplane_io;
#[cfg(feature = "apigatewayv2_services_k8s_aws")]
pub mod apigatewayv2_services_k8s_aws;
#[cfg(feature = "apisix_apache_org")]
pub mod apisix_apache_org;
#[cfg(feature = "apm_k8s_elastic_co")]
pub mod apm_k8s_elastic_co;
#[cfg(feature = "app_kiegroup_org")]
pub mod app_kiegroup_org;
#[cfg(feature = "app_lightbend_com")]
pub mod app_lightbend_com;
#[cfg(feature = "app_redislabs_com")]
pub mod app_redislabs_com;
#[cfg(feature = "app_terraform_io")]
pub mod app_terraform_io;
#[cfg(feature = "application_networking_k8s_aws")]
pub mod application_networking_k8s_aws;
#[cfg(feature = "applicationautoscaling_services_k8s_aws")]
pub mod applicationautoscaling_services_k8s_aws;
#[cfg(feature = "appmesh_k8s_aws")]
pub mod appmesh_k8s_aws;
#[cfg(feature = "appprotect_f5_com")]
pub mod appprotect_f5_com;
#[cfg(feature = "appprotectdos_f5_com")]
pub mod appprotectdos_f5_com;
#[cfg(feature = "apps_3scale_net")]
pub mod apps_3scale_net;
#[cfg(feature = "apps_clusternet_io")]
pub mod apps_clusternet_io;
#[cfg(feature = "apps_emqx_io")]
pub mod apps_emqx_io;
#[cfg(feature = "apps_gitlab_com")]
pub mod apps_gitlab_com;
#[cfg(feature = "apps_kubeblocks_io")]
pub mod apps_kubeblocks_io;
#[cfg(feature = "apps_kubedl_io")]
pub mod apps_kubedl_io;
#[cfg(feature = "apps_kubeedge_io")]
pub mod apps_kubeedge_io;
#[cfg(feature = "apps_m88i_io")]
pub mod apps_m88i_io;
#[cfg(feature = "apps_redhat_com")]
pub mod apps_redhat_com;
#[cfg(feature = "aquasecurity_github_io")]
pub mod aquasecurity_github_io;
#[cfg(feature = "argoproj_io")]
pub mod argoproj_io;
#[cfg(feature = "asdb_aerospike_com")]
pub mod asdb_aerospike_com;
#[cfg(feature = "atlasmap_io")]
pub mod atlasmap_io;
#[cfg(feature = "auth_ops42_org")]
pub mod auth_ops42_org;
#[cfg(feature = "authzed_com")]
pub mod authzed_com;
#[cfg(feature = "automation_kubensync_com")]
pub mod automation_kubensync_com;
#[cfg(feature = "autoscaling_k8s_io")]
pub mod autoscaling_k8s_io;
#[cfg(feature = "autoscaling_karmada_io")]
pub mod autoscaling_karmada_io;
#[cfg(feature = "awx_ansible_com")]
pub mod awx_ansible_com;
#[cfg(feature = "azure_microsoft_com")]
pub mod azure_microsoft_com;
#[cfg(feature = "b3scale_infra_run")]
pub mod b3scale_infra_run;
#[cfg(feature = "b3scale_io")]
pub mod b3scale_io;
#[cfg(feature = "batch_volcano_sh")]
pub mod batch_volcano_sh;
#[cfg(feature = "beat_k8s_elastic_co")]
pub mod beat_k8s_elastic_co;
#[cfg(feature = "beegfs_csi_netapp_com")]
pub mod beegfs_csi_netapp_com;
#[cfg(feature = "binding_operators_coreos_com")]
pub mod binding_operators_coreos_com;
#[cfg(feature = "bitnami_com")]
pub mod bitnami_com;
#[cfg(feature = "bmc_tinkerbell_org")]
pub mod bmc_tinkerbell_org;
#[cfg(feature = "boskos_k8s_io")]
pub mod boskos_k8s_io;
#[cfg(feature = "bpfman_io")]
pub mod bpfman_io;
#[cfg(feature = "bus_volcano_sh")]
pub mod bus_volcano_sh;
#[cfg(feature = "cache_kubedl_io")]
pub mod cache_kubedl_io;
#[cfg(feature = "caching_ibm_com")]
pub mod caching_ibm_com;
#[cfg(feature = "camel_apache_org")]
pub mod camel_apache_org;
#[cfg(feature = "capabilities_3scale_net")]
pub mod capabilities_3scale_net;
#[cfg(feature = "capsule_clastix_io")]
pub mod capsule_clastix_io;
#[cfg(feature = "cassandra_datastax_com")]
pub mod cassandra_datastax_com;
#[cfg(feature = "ceph_rook_io")]
pub mod ceph_rook_io;
#[cfg(feature = "cert_manager_io")]
pub mod cert_manager_io;
#[cfg(feature = "chainsaw_kyverno_io")]
pub mod chainsaw_kyverno_io;
#[cfg(feature = "chaos_mesh_org")]
pub mod chaos_mesh_org;
#[cfg(feature = "chaosblade_io")]
pub mod chaosblade_io;
#[cfg(feature = "charts_amd_com")]
pub mod charts_amd_com;
#[cfg(feature = "charts_flagsmith_com")]
pub mod charts_flagsmith_com;
#[cfg(feature = "charts_helm_k8s_io")]
pub mod charts_helm_k8s_io;
#[cfg(feature = "charts_opdev_io")]
pub mod charts_opdev_io;
#[cfg(feature = "charts_operatorhub_io")]
pub mod charts_operatorhub_io;
#[cfg(feature = "che_eclipse_org")]
pub mod che_eclipse_org;
#[cfg(feature = "chisel_operator_io")]
pub mod chisel_operator_io;
#[cfg(feature = "cilium_io")]
pub mod cilium_io;
#[cfg(feature = "claudie_io")]
pub mod claudie_io;
#[cfg(feature = "cloudformation_linki_space")]
pub mod cloudformation_linki_space;
#[cfg(feature = "cloudfront_services_k8s_aws")]
pub mod cloudfront_services_k8s_aws;
#[cfg(feature = "cloudtrail_services_k8s_aws")]
pub mod cloudtrail_services_k8s_aws;
#[cfg(feature = "cloudwatch_aws_amazon_com")]
pub mod cloudwatch_aws_amazon_com;
#[cfg(feature = "cloudwatch_services_k8s_aws")]
pub mod cloudwatch_services_k8s_aws;
#[cfg(feature = "cloudwatchlogs_services_k8s_aws")]
pub mod cloudwatchlogs_services_k8s_aws;
#[cfg(feature = "cluster_clusterpedia_io")]
pub mod cluster_clusterpedia_io;
#[cfg(feature = "cluster_ipfs_io")]
pub mod cluster_ipfs_io;
#[cfg(feature = "cluster_x_k8s_io")]
pub mod cluster_x_k8s_io;
#[cfg(feature = "clusters_clusternet_io")]
pub mod clusters_clusternet_io;
#[cfg(feature = "clustertemplate_openshift_io")]
pub mod clustertemplate_openshift_io;
#[cfg(feature = "confidentialcontainers_org")]
pub mod confidentialcontainers_org;
#[cfg(feature = "config_gatekeeper_sh")]
pub mod config_gatekeeper_sh;
#[cfg(feature = "config_grafana_com")]
pub mod config_grafana_com;
#[cfg(feature = "config_karmada_io")]
pub mod config_karmada_io;
#[cfg(feature = "config_koordinator_sh")]
pub mod config_koordinator_sh;
#[cfg(feature = "config_storageos_com")]
pub mod config_storageos_com;
#[cfg(feature = "control_k8ssandra_io")]
pub mod control_k8ssandra_io;
#[cfg(feature = "core_kubeadmiral_io")]
pub mod core_kubeadmiral_io;
#[cfg(feature = "core_linuxsuren_github_com")]
pub mod core_linuxsuren_github_com;
#[cfg(feature = "core_openfeature_dev")]
pub mod core_openfeature_dev;
#[cfg(feature = "couchbase_com")]
pub mod couchbase_com;
#[cfg(feature = "craftypath_github_io")]
pub mod craftypath_github_io;
#[cfg(feature = "crane_konveyor_io")]
pub mod crane_konveyor_io;
#[cfg(feature = "crd_projectcalico_org")]
pub mod crd_projectcalico_org;
#[cfg(feature = "data_fluid_io")]
pub mod data_fluid_io;
#[cfg(feature = "databases_schemahero_io")]
pub mod databases_schemahero_io;
#[cfg(feature = "databases_spotahome_com")]
pub mod databases_spotahome_com;
#[cfg(feature = "datadoghq_com")]
pub mod datadoghq_com;
#[cfg(feature = "dataprotection_kubeblocks_io")]
pub mod dataprotection_kubeblocks_io;
#[cfg(feature = "designer_kaoto_io")]
pub mod designer_kaoto_io;
#[cfg(feature = "devices_kubeedge_io")]
pub mod devices_kubeedge_io;
#[cfg(feature = "devops_kubesphere_io")]
pub mod devops_kubesphere_io;
#[cfg(feature = "dex_coreos_com")]
pub mod dex_coreos_com;
#[cfg(feature = "dex_gpu_ninja_com")]
pub mod dex_gpu_ninja_com;
#[cfg(feature = "digitalis_io")]
pub mod digitalis_io;
#[cfg(feature = "documentdb_services_k8s_aws")]
pub mod documentdb_services_k8s_aws;
#[cfg(feature = "druid_apache_org")]
pub mod druid_apache_org;
#[cfg(feature = "dynamodb_services_k8s_aws")]
pub mod dynamodb_services_k8s_aws;
#[cfg(feature = "ec2_services_k8s_aws")]
pub mod ec2_services_k8s_aws;
#[cfg(feature = "ecr_services_k8s_aws")]
pub mod ecr_services_k8s_aws;
#[cfg(feature = "efs_services_k8s_aws")]
pub mod efs_services_k8s_aws;
#[cfg(feature = "eks_services_k8s_aws")]
pub mod eks_services_k8s_aws;
#[cfg(feature = "elasticache_services_k8s_aws")]
pub mod elasticache_services_k8s_aws;
#[cfg(feature = "elasticsearch_k8s_elastic_co")]
pub mod elasticsearch_k8s_elastic_co;
#[cfg(feature = "elbv2_k8s_aws")]
pub mod elbv2_k8s_aws;
#[cfg(feature = "emrcontainers_services_k8s_aws")]
pub mod emrcontainers_services_k8s_aws;
#[cfg(feature = "ensembleoss_io")]
pub mod ensembleoss_io;
#[cfg(feature = "enterprisesearch_k8s_elastic_co")]
pub mod enterprisesearch_k8s_elastic_co;
#[cfg(feature = "everest_percona_com")]
pub mod everest_percona_com;
#[cfg(feature = "execution_furiko_io")]
pub mod execution_furiko_io;
#[cfg(feature = "executor_testkube_io")]
pub mod executor_testkube_io;
#[cfg(feature = "expansion_gatekeeper_sh")]
pub mod expansion_gatekeeper_sh;
#[cfg(feature = "extensions_istio_io")]
pub mod extensions_istio_io;
#[cfg(feature = "extensions_kubeblocks_io")]
pub mod extensions_kubeblocks_io;
#[cfg(feature = "external_secrets_io")]
pub mod external_secrets_io;
#[cfg(feature = "externaldata_gatekeeper_sh")]
pub mod externaldata_gatekeeper_sh;
#[cfg(feature = "externaldns_k8s_io")]
pub mod externaldns_k8s_io;
#[cfg(feature = "externaldns_nginx_org")]
pub mod externaldns_nginx_org;
#[cfg(feature = "flagger_app")]
pub mod flagger_app;
#[cfg(feature = "flink_apache_org")]
pub mod flink_apache_org;
#[cfg(feature = "flow_volcano_sh")]
pub mod flow_volcano_sh;
#[cfg(feature = "flows_netobserv_io")]
pub mod flows_netobserv_io;
#[cfg(feature = "fluentbit_fluent_io")]
pub mod fluentbit_fluent_io;
#[cfg(feature = "fluentd_fluent_io")]
pub mod fluentd_fluent_io;
#[cfg(feature = "flux_framework_org")]
pub mod flux_framework_org;
#[cfg(feature = "forklift_konveyor_io")]
pub mod forklift_konveyor_io;
#[cfg(feature = "fossul_io")]
pub mod fossul_io;
#[cfg(feature = "gateway_networking_k8s_io")]
pub mod gateway_networking_k8s_io;
#[cfg(feature = "gateway_nginx_org")]
pub mod gateway_nginx_org;
#[cfg(feature = "getambassador_io")]
pub mod getambassador_io;
#[cfg(feature = "gitops_hybrid_cloud_patterns_io")]
pub mod gitops_hybrid_cloud_patterns_io;
#[cfg(feature = "grafana_integreatly_org")]
pub mod grafana_integreatly_org;
#[cfg(feature = "groupsnapshot_storage_k8s_io")]
pub mod groupsnapshot_storage_k8s_io;
#[cfg(feature = "hazelcast_com")]
pub mod hazelcast_com;
#[cfg(feature = "helm_sigstore_dev")]
pub mod helm_sigstore_dev;
#[cfg(feature = "helm_toolkit_fluxcd_io")]
pub mod helm_toolkit_fluxcd_io;
#[cfg(feature = "hive_openshift_io")]
pub mod hive_openshift_io;
#[cfg(feature = "hiveinternal_openshift_io")]
pub mod hiveinternal_openshift_io;
#[cfg(feature = "hnc_x_k8s_io")]
pub mod hnc_x_k8s_io;
#[cfg(feature = "hyperfoil_io")]
pub mod hyperfoil_io;
#[cfg(feature = "iam_services_k8s_aws")]
pub mod iam_services_k8s_aws;
#[cfg(feature = "ibmcloud_ibm_com")]
pub mod ibmcloud_ibm_com;
#[cfg(feature = "image_toolkit_fluxcd_io")]
pub mod image_toolkit_fluxcd_io;
#[cfg(feature = "imaging_ingestion_alvearie_org")]
pub mod imaging_ingestion_alvearie_org;
#[cfg(feature = "inference_kubedl_io")]
pub mod inference_kubedl_io;
#[cfg(feature = "infinispan_org")]
pub mod infinispan_org;
#[cfg(feature = "infra_contrib_fluxcd_io")]
pub mod infra_contrib_fluxcd_io;
#[cfg(feature = "infrastructure_cluster_x_k8s_io")]
pub mod infrastructure_cluster_x_k8s_io;
#[cfg(feature = "install_istio_io")]
pub mod install_istio_io;
#[cfg(feature = "installation_mattermost_com")]
pub mod installation_mattermost_com;
#[cfg(feature = "instana_io")]
pub mod instana_io;
#[cfg(feature = "integration_rock8s_com")]
pub mod integration_rock8s_com;
#[cfg(feature = "iot_eclipse_org")]
pub mod iot_eclipse_org;
#[cfg(feature = "ipam_cluster_x_k8s_io")]
pub mod ipam_cluster_x_k8s_io;
#[cfg(feature = "isindir_github_com")]
pub mod isindir_github_com;
#[cfg(feature = "jaegertracing_io")]
pub mod jaegertracing_io;
#[cfg(feature = "jobset_x_k8s_io")]
pub mod jobset_x_k8s_io;
#[cfg(feature = "jobsmanager_raczylo_com")]
pub mod jobsmanager_raczylo_com;
#[cfg(feature = "k6_io")]
pub mod k6_io;
#[cfg(feature = "k8gb_absa_oss")]
pub mod k8gb_absa_oss;
#[cfg(feature = "k8s_keycloak_org")]
pub mod k8s_keycloak_org;
#[cfg(feature = "k8s_mariadb_com")]
pub mod k8s_mariadb_com;
#[cfg(feature = "k8s_nginx_org")]
pub mod k8s_nginx_org;
#[cfg(feature = "k8s_otterize_com")]
pub mod k8s_otterize_com;
#[cfg(feature = "k8up_io")]
pub mod k8up_io;
#[cfg(feature = "kafka_banzaicloud_io")]
pub mod kafka_banzaicloud_io;
#[cfg(feature = "kafka_services_k8s_aws")]
pub mod kafka_services_k8s_aws;
#[cfg(feature = "kafka_strimzi_io")]
pub mod kafka_strimzi_io;
#[cfg(feature = "kamaji_clastix_io")]
pub mod kamaji_clastix_io;
#[cfg(feature = "karpenter_k8s_aws")]
pub mod karpenter_k8s_aws;
#[cfg(feature = "karpenter_sh")]
pub mod karpenter_sh;
#[cfg(feature = "keda_sh")]
pub mod keda_sh;
#[cfg(feature = "keycloak_k8s_reddec_net")]
pub mod keycloak_k8s_reddec_net;
#[cfg(feature = "keycloak_org")]
pub mod keycloak_org;
#[cfg(feature = "keyspaces_services_k8s_aws")]
pub mod keyspaces_services_k8s_aws;
#[cfg(feature = "kibana_k8s_elastic_co")]
pub mod kibana_k8s_elastic_co;
#[cfg(feature = "kinesis_services_k8s_aws")]
pub mod kinesis_services_k8s_aws;
#[cfg(feature = "kmm_sigs_x_k8s_io")]
pub mod kmm_sigs_x_k8s_io;
#[cfg(feature = "kms_services_k8s_aws")]
pub mod kms_services_k8s_aws;
#[cfg(feature = "kuadrant_io")]
pub mod kuadrant_io;
#[cfg(feature = "kube_green_com")]
pub mod kube_green_com;
#[cfg(feature = "kubean_io")]
pub mod kubean_io;
#[cfg(feature = "kubecost_com")]
pub mod kubecost_com;
#[cfg(feature = "kubevious_io")]
pub mod kubevious_io;
#[cfg(feature = "kueue_x_k8s_io")]
pub mod kueue_x_k8s_io;
#[cfg(feature = "kuma_io")]
pub mod kuma_io;
#[cfg(feature = "kustomize_toolkit_fluxcd_io")]
pub mod kustomize_toolkit_fluxcd_io;
#[cfg(feature = "kyverno_io")]
pub mod kyverno_io;
#[cfg(feature = "lambda_services_k8s_aws")]
pub mod lambda_services_k8s_aws;
#[cfg(feature = "leaksignal_com")]
pub mod leaksignal_com;
#[cfg(feature = "lerentis_uploadfilter24_eu")]
pub mod lerentis_uploadfilter24_eu;
#[cfg(feature = "limitador_kuadrant_io")]
pub mod limitador_kuadrant_io;
#[cfg(feature = "litmuschaos_io")]
pub mod litmuschaos_io;
#[cfg(feature = "logging_extensions_banzaicloud_io")]
pub mod logging_extensions_banzaicloud_io;
#[cfg(feature = "logging_banzaicloud_io")]
pub mod logging_banzaicloud_io;
#[cfg(feature = "loki_grafana_com")]
pub mod loki_grafana_com;
#[cfg(feature = "longhorn_io")]
pub mod longhorn_io;
#[cfg(feature = "m4e_krestomat_io")]
pub mod m4e_krestomat_io;
#[cfg(feature = "machine_deletion_remediation_medik8s_io")]
pub mod machine_deletion_remediation_medik8s_io;
#[cfg(feature = "maps_k8s_elastic_co")]
pub mod maps_k8s_elastic_co;
#[cfg(feature = "mariadb_mmontes_io")]
pub mod mariadb_mmontes_io;
#[cfg(feature = "marin3r_3scale_net")]
pub mod marin3r_3scale_net;
#[cfg(feature = "mattermost_com")]
pub mod mattermost_com;
#[cfg(feature = "memorydb_services_k8s_aws")]
pub mod memorydb_services_k8s_aws;
#[cfg(feature = "metacontroller_k8s_io")]
pub mod metacontroller_k8s_io;
#[cfg(feature = "metal3_io")]
pub mod metal3_io;
#[cfg(feature = "microcks_github_io")]
pub mod microcks_github_io;
#[cfg(feature = "minio_min_io")]
pub mod minio_min_io;
#[cfg(feature = "mirrors_kts_studio")]
pub mod mirrors_kts_studio;
#[cfg(feature = "model_kubedl_io")]
pub mod model_kubedl_io;
#[cfg(feature = "monitoring_coreos_com")]
pub mod monitoring_coreos_com;
#[cfg(feature = "monocle_monocle_change_metrics_io")]
pub mod monocle_monocle_change_metrics_io;
#[cfg(feature = "mq_services_k8s_aws")]
pub mod mq_services_k8s_aws;
#[cfg(feature = "multicluster_crd_antrea_io")]
pub mod multicluster_crd_antrea_io;
#[cfg(feature = "multicluster_x_k8s_io")]
pub mod multicluster_x_k8s_io;
#[cfg(feature = "mutations_gatekeeper_sh")]
pub mod mutations_gatekeeper_sh;
#[cfg(feature = "nativestor_alauda_io")]
pub mod nativestor_alauda_io;
#[cfg(feature = "netchecks_io")]
pub mod netchecks_io;
#[cfg(feature = "networkfirewall_services_k8s_aws")]
pub mod networkfirewall_services_k8s_aws;
#[cfg(feature = "networking_gke_io")]
pub mod networking_gke_io;
#[cfg(feature = "networking_istio_io")]
pub mod networking_istio_io;
#[cfg(feature = "networking_k8s_aws")]
pub mod networking_k8s_aws;
#[cfg(feature = "networking_karmada_io")]
pub mod networking_karmada_io;
#[cfg(feature = "nfd_k8s_sigs_io")]
pub mod nfd_k8s_sigs_io;
#[cfg(feature = "nfd_kubernetes_io")]
pub mod nfd_kubernetes_io;
#[cfg(feature = "nodeinfo_volcano_sh")]
pub mod nodeinfo_volcano_sh;
#[cfg(feature = "notebook_kubedl_io")]
pub mod notebook_kubedl_io;
#[cfg(feature = "notification_toolkit_fluxcd_io")]
pub mod notification_toolkit_fluxcd_io;
#[cfg(feature = "objectbucket_io")]
pub mod objectbucket_io;
#[cfg(feature = "onepassword_com")]
pub mod onepassword_com;
#[cfg(feature = "opensearchservice_services_k8s_aws")]
pub mod opensearchservice_services_k8s_aws;
#[cfg(feature = "opentelemetry_io")]
pub mod opentelemetry_io;
#[cfg(feature = "operations_kubeedge_io")]
pub mod operations_kubeedge_io;
#[cfg(feature = "operator_aquasec_com")]
pub mod operator_aquasec_com;
#[cfg(feature = "operator_authorino_kuadrant_io")]
pub mod operator_authorino_kuadrant_io;
#[cfg(feature = "operator_cluster_x_k8s_io")]
pub mod operator_cluster_x_k8s_io;
#[cfg(feature = "operator_cryostat_io")]
pub mod operator_cryostat_io;
#[cfg(feature = "operator_marin3r_3scale_net")]
pub mod operator_marin3r_3scale_net;
#[cfg(feature = "operator_open_cluster_management_io")]
pub mod operator_open_cluster_management_io;
#[cfg(feature = "operator_shipwright_io")]
pub mod operator_shipwright_io;
#[cfg(feature = "operator_tekton_dev")]
pub mod operator_tekton_dev;
#[cfg(feature = "operator_tigera_io")]
pub mod operator_tigera_io;
#[cfg(feature = "operator_victoriametrics_com")]
pub mod operator_victoriametrics_com;
#[cfg(feature = "oracle_db_anthosapis_com")]
pub mod oracle_db_anthosapis_com;
#[cfg(feature = "org_eclipse_che")]
pub mod org_eclipse_che;
#[cfg(feature = "organizations_services_k8s_aws")]
pub mod organizations_services_k8s_aws;
#[cfg(feature = "pgv2_percona_com")]
pub mod pgv2_percona_com;
#[cfg(feature = "pipes_services_k8s_aws")]
pub mod pipes_services_k8s_aws;
#[cfg(feature = "pkg_crossplane_io")]
pub mod pkg_crossplane_io;
#[cfg(feature = "policy_clusterpedia_io")]
pub mod policy_clusterpedia_io;
#[cfg(feature = "policy_karmada_io")]
pub mod policy_karmada_io;
#[cfg(feature = "policy_kubeedge_io")]
pub mod policy_kubeedge_io;
#[cfg(feature = "policy_networking_k8s_io")]
pub mod policy_networking_k8s_io;
#[cfg(feature = "postgres_operator_crunchydata_com")]
pub mod postgres_operator_crunchydata_com;
#[cfg(feature = "postgresql_cnpg_io")]
pub mod postgresql_cnpg_io;
#[cfg(feature = "projectcontour_io")]
pub mod projectcontour_io;
#[cfg(feature = "prometheusservice_services_k8s_aws")]
pub mod prometheusservice_services_k8s_aws;
#[cfg(feature = "ps_percona_com")]
pub mod ps_percona_com;
#[cfg(feature = "psmdb_percona_com")]
pub mod psmdb_percona_com;
#[cfg(feature = "ptp_openshift_io")]
pub mod ptp_openshift_io;
#[cfg(feature = "pxc_percona_com")]
pub mod pxc_percona_com;
#[cfg(feature = "quay_redhat_com")]
pub mod quay_redhat_com;
#[cfg(feature = "quota_codeflare_dev")]
pub mod quota_codeflare_dev;
#[cfg(feature = "ray_io")]
pub mod ray_io;
#[cfg(feature = "rbacmanager_reactiveops_io")]
pub mod rbacmanager_reactiveops_io;
#[cfg(feature = "rc_app_stacks")]
pub mod rc_app_stacks;
#[cfg(feature = "rds_services_k8s_aws")]
pub mod rds_services_k8s_aws;
#[cfg(feature = "redhatcop_redhat_io")]
pub mod redhatcop_redhat_io;
#[cfg(feature = "registry_apicur_io")]
pub mod registry_apicur_io;
#[cfg(feature = "registry_devfile_io")]
pub mod registry_devfile_io;
#[cfg(feature = "reliablesyncs_kubeedge_io")]
pub mod reliablesyncs_kubeedge_io;
#[cfg(feature = "remediation_medik8s_io")]
pub mod remediation_medik8s_io;
#[cfg(feature = "repo_manager_pulpproject_org")]
pub mod repo_manager_pulpproject_org;
#[cfg(feature = "reports_kyverno_io")]
pub mod reports_kyverno_io;
#[cfg(feature = "resources_teleport_dev")]
pub mod resources_teleport_dev;
#[cfg(feature = "rocketmq_apache_org")]
pub mod rocketmq_apache_org;
#[cfg(feature = "route53_services_k8s_aws")]
pub mod route53_services_k8s_aws;
#[cfg(feature = "route53resolver_services_k8s_aws")]
pub mod route53resolver_services_k8s_aws;
#[cfg(feature = "rules_kubeedge_io")]
pub mod rules_kubeedge_io;
#[cfg(feature = "runtime_cluster_x_k8s_io")]
pub mod runtime_cluster_x_k8s_io;
#[cfg(feature = "s3_services_k8s_aws")]
pub mod s3_services_k8s_aws;
#[cfg(feature = "s3_snappcloud_io")]
pub mod s3_snappcloud_io;
#[cfg(feature = "sagemaker_services_k8s_aws")]
pub mod sagemaker_services_k8s_aws;
#[cfg(feature = "scheduling_koordinator_sh")]
pub mod scheduling_koordinator_sh;
#[cfg(feature = "scheduling_sigs_k8s_io")]
pub mod scheduling_sigs_k8s_io;
#[cfg(feature = "scheduling_volcano_sh")]
pub mod scheduling_volcano_sh;
#[cfg(feature = "schemas_schemahero_io")]
pub mod schemas_schemahero_io;
#[cfg(feature = "scylla_scylladb_com")]
pub mod scylla_scylladb_com;
#[cfg(feature = "secretgenerator_mittwald_de")]
pub mod secretgenerator_mittwald_de;
#[cfg(feature = "secrets_store_csi_x_k8s_io")]
pub mod secrets_store_csi_x_k8s_io;
#[cfg(feature = "secrets_crossplane_io")]
pub mod secrets_crossplane_io;
#[cfg(feature = "secrets_doppler_com")]
pub mod secrets_doppler_com;
#[cfg(feature = "secrets_hashicorp_com")]
pub mod secrets_hashicorp_com;
#[cfg(feature = "secretsmanager_services_k8s_aws")]
pub mod secretsmanager_services_k8s_aws;
#[cfg(feature = "secscan_quay_redhat_com")]
pub mod secscan_quay_redhat_com;
#[cfg(feature = "security_profiles_operator_x_k8s_io")]
pub mod security_profiles_operator_x_k8s_io;
#[cfg(feature = "security_istio_io")]
pub mod security_istio_io;
#[cfg(feature = "self_node_remediation_medik8s_io")]
pub mod self_node_remediation_medik8s_io;
#[cfg(feature = "sematext_com")]
pub mod sematext_com;
#[cfg(feature = "servicebinding_io")]
pub mod servicebinding_io;
#[cfg(feature = "servicemesh_cisco_com")]
pub mod servicemesh_cisco_com;
#[cfg(feature = "services_k8s_aws")]
pub mod services_k8s_aws;
#[cfg(feature = "serving_kubedl_io")]
pub mod serving_kubedl_io;
#[cfg(feature = "sfn_services_k8s_aws")]
pub mod sfn_services_k8s_aws;
#[cfg(feature = "site_superedge_io")]
pub mod site_superedge_io;
#[cfg(feature = "slo_koordinator_sh")]
pub mod slo_koordinator_sh;
#[cfg(feature = "sloth_slok_dev")]
pub mod sloth_slok_dev;
#[cfg(feature = "snapscheduler_backube")]
pub mod snapscheduler_backube;
#[cfg(feature = "snapshot_storage_k8s_io")]
pub mod snapshot_storage_k8s_io;
#[cfg(feature = "sns_services_k8s_aws")]
pub mod sns_services_k8s_aws;
#[cfg(feature = "sonataflow_org")]
pub mod sonataflow_org;
#[cfg(feature = "source_toolkit_fluxcd_io")]
pub mod source_toolkit_fluxcd_io;
#[cfg(feature = "sparkoperator_k8s_io")]
pub mod sparkoperator_k8s_io;
#[cfg(feature = "spv_no")]
pub mod spv_no;
#[cfg(feature = "sqs_services_k8s_aws")]
pub mod sqs_services_k8s_aws;
#[cfg(feature = "status_gatekeeper_sh")]
pub mod status_gatekeeper_sh;
#[cfg(feature = "storage_kubeblocks_io")]
pub mod storage_kubeblocks_io;
#[cfg(feature = "storageos_com")]
pub mod storageos_com;
#[cfg(feature = "sts_min_io")]
pub mod sts_min_io;
#[cfg(feature = "stunner_l7mp_io")]
pub mod stunner_l7mp_io;
#[cfg(feature = "submariner_io")]
pub mod submariner_io;
#[cfg(feature = "telemetry_istio_io")]
pub mod telemetry_istio_io;
#[cfg(feature = "templates_gatekeeper_sh")]
pub mod templates_gatekeeper_sh;
#[cfg(feature = "tempo_grafana_com")]
pub mod tempo_grafana_com;
#[cfg(feature = "temporal_io")]
pub mod temporal_io;
#[cfg(feature = "tests_testkube_io")]
pub mod tests_testkube_io;
#[cfg(feature = "tf_tungsten_io")]
pub mod tf_tungsten_io;
#[cfg(feature = "theketch_io")]
pub mod theketch_io;
#[cfg(feature = "tinkerbell_org")]
pub mod tinkerbell_org;
#[cfg(feature = "topology_node_k8s_io")]
pub mod topology_node_k8s_io;
#[cfg(feature = "topolvm_cybozu_com")]
pub mod topolvm_cybozu_com;
#[cfg(feature = "traefik_io")]
pub mod traefik_io;
#[cfg(feature = "training_kubedl_io")]
pub mod training_kubedl_io;
#[cfg(feature = "trident_netapp_io")]
pub mod trident_netapp_io;
#[cfg(feature = "trust_cert_manager_io")]
pub mod trust_cert_manager_io;
#[cfg(feature = "upgrade_cattle_io")]
pub mod upgrade_cattle_io;
#[cfg(feature = "upgrade_managed_openshift_io")]
pub mod upgrade_managed_openshift_io;
#[cfg(feature = "velero_io")]
pub mod velero_io;
#[cfg(feature = "virt_virtink_smartx_com")]
pub mod virt_virtink_smartx_com;
#[cfg(feature = "volsync_backube")]
pub mod volsync_backube;
#[cfg(feature = "vpcresources_k8s_aws")]
pub mod vpcresources_k8s_aws;
#[cfg(feature = "wgpolicyk8s_io")]
pub mod wgpolicyk8s_io;
#[cfg(feature = "wildfly_org")]
pub mod wildfly_org;
#[cfg(feature = "work_karmada_io")]
pub mod work_karmada_io;
#[cfg(feature = "workload_codeflare_dev")]
pub mod workload_codeflare_dev;
#[cfg(feature = "workloads_kubeblocks_io")]
pub mod workloads_kubeblocks_io;
#[cfg(feature = "zonecontrol_k8s_aws")]
pub mod zonecontrol_k8s_aws;
#[cfg(feature = "zookeeper_pravega_io")]
pub mod zookeeper_pravega_io;
