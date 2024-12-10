data "aws_region" "current" {}
data "aws_caller_identity" "current" {}

data "aws_vpc" "fomiller" {
  filter {
    name   = "tag:Name"
    values = ["${var.namespace}-vpc"]
  }
}

data "aws_subnets" "private" {
  filter {
    name   = "tag:Tier"
    values = ["private"]
  }
}

data "aws_lambda_function" "discord_bot_exists" {
  count         = fileexists(local.filename["discord_bot"]) ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-discord-bot"
}

data "aws_lambda_function" "discord_command_manager_exists" {
  count         = fileexists(local.filename["discord_command_manager"]) ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-discord-command-manager"
}

data "aws_lambda_function" "dnl_api_exists" {
  count         = fileexists(local.filename["dnl_api"]) ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-api"
}

data "aws_lambda_function" "llm_handler_exists" {
  count         = fileexists(local.filename["llm_handler"]) ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-llm-handler"
}

data "aws_lambda_function" "db_manager_exists" {
  count         = fileexists(local.filename["db_manager"]) ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-db-manager"
}

data "aws_bedrock_foundation_models" "test" {
  by_inference_type = "ON_DEMAND"
  by_provider       = "Meta"
}

data "aws_security_group" "rds" {
  name = "db-sg"
}
