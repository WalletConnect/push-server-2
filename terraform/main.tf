resource "random_pet" "this" {
  length = 2
}

locals {
  ecr_repository_url = data.terraform_remote_state.org.outputs.accounts.wl.push[local.stage].ecr-url

  stage = lookup({
    "push-server-wl-staging" = "staging",
    "push-server-wl-prod"    = "prod",
    "push-server-staging"    = "staging",
    "push-server-prod"       = "prod",
    "wl-staging"             = "staging",
    "wl-prod"                = "prod",
    "staging"                = "staging",
    "prod"                   = "prod",
  }, terraform.workspace, terraform.workspace)
}
