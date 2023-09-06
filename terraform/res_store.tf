locals {
  db_instances = module.this.stage == "prod" ? 2 : 1
}

module "database_cluster" {
  source  = "./rds"
  context = module.this.context

  name      = "main"
  instances = local.db_instances

  vpc_id               = module.vpc.vpc_id
  db_subnet_group_name = module.vpc.database_subnet_group_name
  ingress_cidr_blocks  = module.vpc.private_subnets_cidr_blocks

  iam_db_role = aws_iam_role.application_role.name
}

#module "database_cluster_tenant" {
#  source  = "./rds"
#  context = module.this.context
#
#  name         = "tenant"
#  db_instances = local.db_instances
#
#  vpc_id               = module.vpc.vpc_id
#  db_subnet_group_name = module.vpc.database_subnet_group_name
#  ingress_cidr_blocks  = module.vpc.private_subnets_cidr_blocks
#
#  iam_db_role = aws_iam_role.application_role.name
#}
