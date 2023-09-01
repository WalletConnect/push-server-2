#-------------------------------------------------------------------------------
# Configuration

variable "grafana_auth" {
  description = "The API Token for the Grafana instance"
  type        = string
  default     = ""
}

#-------------------------------------------------------------------------------
# Application

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
# Monitoring

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
  default     = []
}
