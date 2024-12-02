resource "aws_lambda_function" "discord_bot" {
  function_name    = "${var.namespace}-${var.app_prefix}-discord-bot"
  role             = var.iam_role_arn_lambda_discord_bot
  handler          = "bootstrap"
  filename         = local.filename["discord_bot"]
  source_code_hash = local.source_code_hash["discord_bot"]
  runtime          = local.runtime
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
  filename         = local.filename["discord_command_manager"]
  source_code_hash = local.source_code_hash["discord_command_manager"]
  runtime          = local.runtime
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
  filename         = local.filename["dnl_api"]
  source_code_hash = local.source_code_hash["dnl_api"]
  runtime          = local.runtime
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

resource "aws_lambda_function" "llm_handler" {
  function_name    = "${var.namespace}-${var.app_prefix}-llm-handler"
  role             = var.iam_role_arn_lambda_llm_handler
  handler          = "bootstrap"
  filename         = local.filename["llm_handler"]
  source_code_hash = local.source_code_hash["llm_handler"]
  runtime          = local.runtime
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      ACCOUNT               = data.aws_caller_identity.current.account_id
      AWS_LAMBDA_LOG_LEVEL  = "INFO"
      AWS_LAMBDA_LOG_FORMAT = "JSON"
      ENVIRONMENT           = var.environment
      REGION                = data.aws_region.current.name
    }
  }
}
