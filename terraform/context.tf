module "this" {
  source  = "app.terraform.io/wallet-connect/label/null"
  version = "0.3.2"

  namespace = var.name
  region    = var.region
  stage     = local.stage
  #  name      = var.name

  tags = {
    Application = var.name
  }
}
