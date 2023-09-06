resource "aws_iam_role" "application_role" {
  name = "${module.this.id}-ecs-task-execution"
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      }
    ]
  })
}

#locals {
#  app_settings = {
#    "staging" : {
#      allowed_origins        = "*",
#      telemetry_sample_ratio = 1.0,
#      autoscaling : {
#        desired_count = 1,
#        min_capacity  = 1,
#        max_capacity  = 1,
#      }
#    },
#    "prod" : {
#      allowed_origins        = "https://cloud.walletconnect.com",
#      telemetry_sample_ratio = 0.25,
#      autoscaling : {
#        desired_count = 2,
#        min_capacity  = 2,
#        max_capacity  = 4,
#      }
#    }
#  }
#
#  geoip_db_bucket_name            = "${local.stage}.relay.geo.ip.database.private.${local.stage}.walletconnect"
#  analytics_data_lake_bucket_name = "walletconnect.data-lake.${local.stage}"
#}
#
##TODO: Migrate the data project and export the ARNs with proper IAM configuration for the KMS keys
#data "aws_kms_key" "analytics_key" {
#  key_id = "alias/analytics/data_lake/${local.stage}"
#}
#
#resource "aws_prometheus_workspace" "prometheus" {
#  alias = "prometheus-${module.this.id}"
#}
#
## ECS Cluster, Task, Service, and Load Balancer for our app
#module "ecs" {
#  source  = "./ecs"
#  context = module.this
#
#  # Cluster
#  ecr_repository_url        = local.ecr_repository_url
#  image_version             = var.image_version
#  task_cpu                  = 512
#  task_memory               = 1024
#  autoscaling_desired_count = local.app_settings[local.stage].autoscaling.desired_count
#  autoscaling_min_capacity  = local.app_settings[local.stage].autoscaling.min_capacity
#  autoscaling_max_capacity  = local.app_settings[local.stage].autoscaling.max_capacity
#
#  # DNS
#  route53_zones              = local.zones
#  route53_zones_certificates = local.zones_certificates
#
#  # Network
#  vpc_id                          = module.vpc.vpc_id
#  public_subnets                  = module.vpc.public_subnets
#  private_subnets                 = module.vpc.private_subnets
#  database_subnets                = module.vpc.database_subnets
#  allowed_app_ingress_cidr_blocks = module.vpc.vpc_cidr_block
#  allowed_lb_ingress_cidr_blocks  = module.vpc.vpc_cidr_block
#
#  # Application
#  port      = 8080
#  log_level = var.log_level
#
#  database_url           = local.database_url
#  tenant_database_url    = local.tenant_database_url
#  telemetry_sample_ratio = local.app_settings[local.stage].telemetry_sample_ratio
#  allowed_origins        = local.app_settings[local.stage].allowed_origins
#
#  analytics_datalake_bucket_name = local.analytics_data_lake_bucket_name
#  analytics_key_arn              = data.aws_kms_key.analytics_key.arn
#  analytics_geoip_db_bucket_name = local.geoip_db_bucket_name
#  analytics_geoip_db_key         = var.geoip_db_key
#
#  cloud_api_key = var.cloud_api_key
#  cloud_api_url = "https://registry.walletconnect.com/"
#
#  jwt_secret = var.jwt_secret
#
#  # Monitoring
#  prometheus_endpoint = aws_prometheus_workspace.prometheus.prometheus_endpoint
#}
