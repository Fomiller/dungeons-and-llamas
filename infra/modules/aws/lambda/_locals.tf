locals {
  source_code_hash = {
    dnl_discord_bot = fileexists("${path.module}/bin/hello/bootstrap.zip") ? filebase64sha256("${path.module}/bin/hello/bootstrap.zip") : data.aws_lambda_function.dnl_discord_bot_exists[0].source_code_hash
    command_manager = fileexists("${path.module}/bin/command-manager/bootstrap.zip") ? filebase64sha256("${path.module}/bin/command-manager/bootstrap.zip") : data.aws_lambda_function.command_manager_exists[0].source_code_hash
  }
}
