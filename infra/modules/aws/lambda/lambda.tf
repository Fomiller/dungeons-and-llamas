resource "aws_lambda_function" "dnd_bot" {
  function_name    = "${var.namespace}-${var.app_prefix}-bot"
  role             = var.iam_role_arn_lambda_dnd_bot
  handler          = "bootstrap"
  filename         = "${path.module}/bin/hello/bootstrap.zip"
  source_code_hash = local.source_code_hash["dnd_bot"]
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      REGION             = data.aws_region.current.name
      DISCORD_PUBLIC_KEY = var.discord_public_key
      ACCOUNT            = data.aws_caller_identity.current.account_id
    }
  }
}


resource "aws_lambda_function" "command_manager" {
  function_name    = "${var.namespace}-${var.app_prefix}-command-manager"
  role             = var.iam_role_arn_lambda_dnd_bot
  handler          = "bootstrap"
  filename         = "${path.module}/bin/commandmanager/bootstrap.zip"
  source_code_hash = local.source_code_hash["command_manager"]
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  memory_size      = 128
  timeout          = 10
  environment {
    variables = {
      REGION            = data.aws_region.current.name
      DISCORD_APP_ID    = var.discord_application_id
      DISCORD_BOT_TOKEN = var.discord_token
      ACCOUNT           = data.aws_caller_identity.current.account_id
    }
  }
}
