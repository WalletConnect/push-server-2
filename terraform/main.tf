locals {
  app_name    = "push-server"
  environment = terraform.workspace
}

data "assert_test" "workspace" {
  test  = local.environment != "default"
  throw = "default workspace is not valid for this project"
}

module "tags" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/tags"

  application = local.app_name
  env         = local.environment
}
