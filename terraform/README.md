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
| <a name="provider_aws"></a> [aws](#provider\_aws) | >= 5.7 |
| <a name="provider_random"></a> [random](#provider\_random) | 3.5.1 |
| <a name="provider_terraform"></a> [terraform](#provider\_terraform) | n/a |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_database_cluster"></a> [database\_cluster](#module\_database\_cluster) | ./rds | n/a |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |
| <a name="module_vpc"></a> [vpc](#module\_vpc) | terraform-aws-modules/vpc/aws | ~> 5.0 |
| <a name="module_vpc_endpoints"></a> [vpc\_endpoints](#module\_vpc\_endpoints) | terraform-aws-modules/vpc/aws//modules/vpc-endpoints | 5.1 |
| <a name="module_vpc_flow_s3_bucket"></a> [vpc\_flow\_s3\_bucket](#module\_vpc\_flow\_s3\_bucket) | terraform-aws-modules/s3-bucket/aws | ~> 3.14 |

## Resources

| Name | Type |
|------|------|
| [aws_iam_role.application_role](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/iam_role) | resource |
| [random_pet.this](https://registry.terraform.io/providers/hashicorp/random/3.5.1/docs/resources/pet) | resource |
| [aws_availability_zones.available](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/availability_zones) | data source |
| [terraform_remote_state.dns](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.monitoring](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.org](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_cloud_api_key"></a> [cloud\_api\_key](#input\_cloud\_api\_key) | The authentication key for the cloud API | `string` | n/a | yes |
| <a name="input_geoip_db_key"></a> [geoip\_db\_key](#input\_geoip\_db\_key) | The name to the GeoIP database | `string` | `"GeoLite2-City.mmdb"` | no |
| <a name="input_grafana_auth"></a> [grafana\_auth](#input\_grafana\_auth) | The API Token for the Grafana instance | `string` | `""` | no |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the image to deploy | `string` | n/a | yes |
| <a name="input_jwt_secret"></a> [jwt\_secret](#input\_jwt\_secret) | The secret used to sign JWT tokens | `string` | n/a | yes |
| <a name="input_log_level"></a> [log\_level](#input\_log\_level) | Defines logging level for the application | `string` | n/a | yes |
| <a name="input_name"></a> [name](#input\_name) | The name of the application | `string` | `"push-server"` | no |
| <a name="input_notification_channels"></a> [notification\_channels](#input\_notification\_channels) | The notification channels to send alerts to | `list(any)` | `[]` | no |
| <a name="input_region"></a> [region](#input\_region) | AWS region to deploy to | `string` | n/a | yes |

## Outputs

No outputs.
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
