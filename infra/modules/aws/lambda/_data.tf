data "aws_region" "current" {}
data "aws_caller_identity" "current" {}

data "aws_lambda_function" "discord_bot_exists" {
  count         = fileexists("${path.module}/bin/discord-bot/bootstrap.zip") ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-discord-bot"
}

data "aws_lambda_function" "discord_command_manager_exists" {
  count         = fileexists("${path.module}/bin/discord-command-manager/bootstrap.zip") ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-discord-command-manager"
}

data "aws_lambda_function" "dnl_api_exists" {
  count         = fileexists("${path.module}/bin/api/bootstrap.zip") ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-api"
}
