resource "aws_lambda_function" "discord_bot" {
  function_name    = "${var.namespace}-${var.app_prefix}-discord-bot"
  role             = var.iam_role_arn_lambda_discord_bot
  handler          = "bootstrap"
  filename         = "${path.module}/bin/discord-bot/bootstrap.zip"
  source_code_hash = local.source_code_hash["discord_bot"]
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ENVIRONMENT        = var.environment
      ACCOUNT            = data.aws_caller_identity.current.account_id
      REGION             = data.aws_region.current.name
      DISCORD_PUBLIC_KEY = var.discord_public_key
    }
  }
}


resource "aws_lambda_function" "discord_command_manager" {
  function_name    = "${var.namespace}-${var.app_prefix}-discord-command-manager"
  role             = var.iam_role_arn_lambda_discord_command_manager
  handler          = "bootstrap"
  filename         = "${path.module}/bin/discord-command-manager/bootstrap.zip"
  source_code_hash = local.source_code_hash["discord_command_manager"]
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ENVIRONMENT       = var.environment
      ACCOUNT           = data.aws_caller_identity.current.account_id
      REGION            = data.aws_region.current.name
      DISCORD_APP_ID    = var.discord_application_id
      DISCORD_BOT_TOKEN = var.discord_token
    }
  }
}
