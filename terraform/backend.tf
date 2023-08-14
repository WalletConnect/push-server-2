terraform {
  required_version = "~> 1.0"

  backend "remote" {
    hostname     = "app.terraform.io"
    organization = "wallet-connect"
    workspaces {
      prefix = "push-server-"
    }
  }

  required_providers {
    assert = {
      source = "bwoxnicki/assert"
    }
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.31"
    }
  }
}