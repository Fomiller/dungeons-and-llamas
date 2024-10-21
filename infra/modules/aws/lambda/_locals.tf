locals {
  source_code_hash = {
    discord_bot             = fileexists("${path.module}/bin/discord-bot/bootstrap.zip") ? filebase64sha256("${path.module}/bin/discord-bot/bootstrap.zip") : data.aws_lambda_function.discord_bot_exists[0].source_code_hash
    discord_command_manager = fileexists("${path.module}/bin/discord-command-manager/bootstrap.zip") ? filebase64sha256("${path.module}/bin/discord-command-manager/bootstrap.zip") : data.aws_lambda_function.discord_command_manager_exists[0].source_code_hash
    dnl_api                 = fileexists("${path.module}/bin/dnl-api/bootstrap.zip") ? filebase64sha256("${path.module}/bin/dnl-api/bootstrap.zip") : data.aws_lambda_function.dnl_api_exists[0].source_code_hash
  }
}
