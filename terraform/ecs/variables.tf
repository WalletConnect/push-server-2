#---------------------------------------
# Cluster

variable "ecr_repository_url" {
  description = "The URL of the ECR repository where the app image is stored"
  type        = string
}

variable "image_version" {
  description = "The version of the app image to deploy"
  type        = string
}

variable "task_cpu" {
  description = "The number of CPU units to reserve for the container."
  type        = number
}

variable "task_memory" {
  description = "The amount of memory (in MiB) to reserve for the container."
  type        = number
}

variable "autoscaling_desired_count" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_min_capacity" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_max_capacity" {
  description = "Maximum number of instances in the autoscaling group"
  type        = number
  default     = 8
}

#---------------------------------------
# DNS

variable "route53_zones" {
  description = "The FQDNs to use for the app"
  type        = map(string)
}

variable "route53_zones_certificates" {
  description = "The ARNs of the ACM certificates to use for HTTPS"
  type        = map(string)
}

#---------------------------------------
# Network

variable "vpc_id" {
  description = "The ID of the VPC to deploy to"
  type        = string
}

variable "public_subnets" {
  description = "The IDs of the public subnets"
  type        = list(string)
}

variable "private_subnets" {
  description = "The IDs of the private subnets"
  type        = list(string)
}

variable "database_subnets" {
  description = "The IDs of the database subnets"
  type        = list(string)
}

variable "allowed_app_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the application."
  type        = string
}

variable "allowed_lb_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the load-balancer."
  type        = string
}
#---------------------------------------
# Application

variable "port" {
  description = "The port the app listens on"
  type        = number
}

variable "log_level" {
  description = "Defines logging level for the application"
  type        = string
}

variable "database_url" {}
variable "tenant_database_url" {}
variable "telemetry_sample_ratio" {}
variable "allowed_origins" {}
variable "analytics_datalake_bucket_name" {}
variable "analytics_key_arn" {}
variable "analytics_geoip_db_bucket_name" {}
variable "analytics_geoip_db_key" {}
variable "cloud_api_key" {}
variable "cloud_api_url" {}
variable "jwt_secret" {}

#---------------------------------------
# Monitoring

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus server to use for monitoring"
  type        = string
}
