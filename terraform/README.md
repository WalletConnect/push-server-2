# Terraform Infrastructure

You need to be authenticated to Terraform Cloud to manage the infrastructure
from your computer.

To authenticate, run `terraform login` and follow the instructions.

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | >= 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | >= 5.7 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | >= 2.1 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.5.1 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_random"></a> [random](#provider\_random) | 3.5.1 |
| <a name="provider_terraform"></a> [terraform](#provider\_terraform) | n/a |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |

## Resources

| Name | Type |
|------|------|
| [random_pet.this](https://registry.terraform.io/providers/hashicorp/random/3.5.1/docs/resources/pet) | resource |
| [terraform_remote_state.dns](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.monitoring](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.org](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_grafana_auth"></a> [grafana\_auth](#input\_grafana\_auth) | The API Token for the Grafana instance | `string` | `""` | no |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the image to deploy | `string` | n/a | yes |
| <a name="input_log_level"></a> [log\_level](#input\_log\_level) | Defines logging level for the application | `string` | n/a | yes |
| <a name="input_name"></a> [name](#input\_name) | The name of the application | `string` | `"push-server"` | no |
| <a name="input_notification_channels"></a> [notification\_channels](#input\_notification\_channels) | The notification channels to send alerts to | `list(any)` | `[]` | no |
| <a name="input_region"></a> [region](#input\_region) | AWS region to deploy to | `string` | n/a | yes |

## Outputs

No outputs.
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
