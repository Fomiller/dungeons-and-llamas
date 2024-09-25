locals {
    namespace = "fomiller"
    project_name = "{name}"
}

generate provider {
  path      = "_.provider.gen.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      email = "forrestmillerj@gmail.com"
      managedWith = "terraform"
    }
  }
}
EOF
}

generate versions {
  path      = "_.versions.gen.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
terraform {
  required_version = ">=1.3.0"
  required_providers {
      aws = {
          source = "hashicorp/aws"
          version = ">=5.0.0"
      }
  }
}
EOF
}

generate variables {
  path      = "_.variables.gen.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
variable "environment" {
    type = string
}

variable "app_prefix" {
    type = string
    default = "${local.project_name}"
}

variable "asset_name" {
    type = string
}

variable "namespace" {
    type = string
    default = "${local.namespace}"
}
EOF
}

remote_state {
  backend = "s3"
  config = {
    encrypt        = true
    disable_bucket_update= true
    bucket         = "${local.namespace}-terraform-state-${get_env("TF_VAR_environment", "dev")}"
    key            = "${local.project_name}/${path_relative_to_include()}/terraform.tfstate"
    region         = "${get_env("TF_VAR_region", "us-east-1")}"
    dynamodb_table = "${local.namespace}-terraform-state-lock"
  }
  generate = {
    path      = "_.backend.gen.tf"
    if_exists = "overwrite_terragrunt"
  }
}

terraform {
  # Force Terraform to keep trying to acquire a lock for
  # up to 20 minutes if someone else already has the lock
  extra_arguments "var_files" {
    commands = [
      "apply",
      "plan",
      "import",
      "push",
      "refresh",
      "output",
      "init",
      "destroy",
      "validate-inputs",
    ]
    # env_vars = {
    #   TF_VAR_var_from_environment = "value"
    # }
    required_var_files = [
      "${get_parent_terragrunt_dir()}/common.tfvars",
      "${get_terragrunt_dir()}/env-config/common.tfvars",
      "${get_terragrunt_dir()}/env-config/${get_env("TF_VAR_region", "us-east-1")}/common.tfvars",
      "${get_terragrunt_dir()}/env-config/${get_env("TF_VAR_region", "us-east-1")}/${get_env("TF_VAR_environment", "dev")}.tfvars",
    ]
  }
}
