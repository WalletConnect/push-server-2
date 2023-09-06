#-------------------------------------------------------------------------------
# Application

variable "cloud_api_key" {
  description = "The authentication key for the cloud API"
  type        = string
  sensitive   = true
}

variable "jwt_secret" {
  description = "The secret used to sign JWT tokens"
  type        = string
  sensitive   = true
}

#-------------------------------------------------------------------------------
# ECS

variable "name" {
  description = "The name of the application"
  type        = string
  default     = "push-server"
}

variable "region" {
  description = "AWS region to deploy to"
  type        = string
}

variable "image_version" {
  description = "The version of the image to deploy"
  type        = string
}

variable "log_level" {
  description = "Defines logging level for the application"
  type        = string
}

#-------------------------------------------------------------------------------
# Analytics

variable "geoip_db_key" {
  description = "The name to the GeoIP database"
  type        = string
  default     = "GeoLite2-City.mmdb"
}

#-------------------------------------------------------------------------------
# Monitoring

variable "grafana_auth" {
  description = "The API Token for the Grafana instance"
  type        = string
  default     = ""
}

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
  default     = []
}
