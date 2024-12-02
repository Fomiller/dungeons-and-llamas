locals {
  runtime = "provided.al2023"
  filename = {
    discord_bot             = "${path.module}/bin/discord-bot/bootstrap.zip"
    discord_command_manager = "${path.module}/bin/discord-command-manager/bootstrap.zip"
    dnl_api                 = "${path.module}/bin/${var.app_prefix}-api/bootstrap.zip"
    llm_handler             = "${path.module}/bin/llm-handler/bootstrap.zip"
  }
  source_code_hash = {
    discord_bot             = fileexists(local.filename["discord_bot"]) ? filebase64sha256(local.filename["discord_bot"]) : data.aws_lambda_function.discord_bot_exists[0].code_sha256
    discord_command_manager = fileexists(local.filename["discord_command_manager"]) ? filebase64sha256(local.filename["discord_command_manager"]) : data.aws_lambda_function.discord_command_manager_exists[0].code_sha256
    dnl_api                 = fileexists(local.filename["dnl_api"]) ? filebase64sha256(local.filename["dnl_api"]) : data.aws_lambda_function.dnl_api_exists[0].code_sha256
    llm_handler             = fileexists(local.filename["llm_handler"]) ? filebase64sha256(local.filename["llm_handler"]) : data.aws_lambda_function.llm_handler_exists[0].code_sha256
  }

  # lambda = {
  #   discord_bot = {
  #     filename         = "${path.module}/bin/discord-bot/bootstrap.zip"
  #     source_code_hash = fileexists(local.lambda["discord_bot"].filename) ? filebase64sha256(local.lambda["discord_bot"].filename) : data.aws_lambda_function.discord_bot_exists[0].source_code_hash
  #   }
  #   discord_command_manager = {
  #     filename         = "${path.module}/bin/discord-command-manager/bootstrap.zip"
  #     source_code_hash = fileexists(local.lambda["discord_command_manager"].filename) ? filebase64sha256(local.lambda["discord_command_manager"].filename) : data.aws_lambda_function.discord_command_manager_exists[0].source_code_hash
  #   }
  #   dnl_api = {
  #     filename         = "${path.module}/bin/dnl-api/bootstrap.zip"
  #     source_code_hash = fileexists(local.lambda["dnl_api"].filename) ? filebase64sha256(local.lambda["dnl_api"].filename) : data.aws_lambda_function.discord_bot_exists[0].source_code_hash
  #   }
  # }
}
