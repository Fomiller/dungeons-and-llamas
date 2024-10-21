resource "aws_lambda_function" "discord_bot" {
  function_name    = "${var.namespace}-${var.app_prefix}-discord-bot"
  role             = var.iam_role_arn_lambda_discord_bot
  handler          = "bootstrap"
  filename         = local.lambda["discord_bot"].filename
  source_code_hash = local.lambda["discord_bot"].source_code_hash
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ACCOUNT               = data.aws_caller_identity.current.account_id
      AWS_LAMBDA_LOG_LEVEL  = "INFO"
      AWS_LAMBDA_LOG_FORMAT = "JSON"
      DISCORD_PUBLIC_KEY    = var.discord_public_key
      DISCORD_APP_ID        = var.discord_application_id
      DISCORD_BOT_TOKEN     = var.discord_token
      ENVIRONMENT           = var.environment
      REGION                = data.aws_region.current.name
    }
  }
}


resource "aws_lambda_function" "discord_command_manager" {
  function_name    = "${var.namespace}-${var.app_prefix}-discord-command-manager"
  role             = var.iam_role_arn_lambda_discord_command_manager
  handler          = "bootstrap"
  filename         = local.lambda["discord_command_manager"].filename
  source_code_hash = local.lambda["discord_command_manager"].source_code_hash
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ACCOUNT               = data.aws_caller_identity.current.account_id
      AWS_LAMBDA_LOG_LEVEL  = "INFO"
      AWS_LAMBDA_LOG_FORMAT = "JSON"
      DISCORD_APP_ID        = var.discord_application_id
      DISCORD_BOT_TOKEN     = var.discord_token
      ENVIRONMENT           = var.environment
      REGION                = data.aws_region.current.name
    }
  }
}

resource "aws_lambda_function" "dnl_api" {
  function_name    = "${var.namespace}-${var.app_prefix}-api"
  role             = var.iam_role_arn_lambda_dnl_api
  handler          = "bootstrap"
  filename         = local.lambda["dnl_api"].filename
  source_code_hash = local.lambda["dnl_api"].source_code_hash
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ACCOUNT               = data.aws_caller_identity.current.account_id
      AWS_LAMBDA_LOG_LEVEL  = "INFO"
      AWS_LAMBDA_LOG_FORMAT = "JSON"
      DISCORD_APP_ID        = var.discord_application_id
      DISCORD_BOT_TOKEN     = var.discord_token
      ENVIRONMENT           = var.environment
      REGION                = data.aws_region.current.name
    }
  }
}
