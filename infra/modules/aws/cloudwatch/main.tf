resource "aws_cloudwatch_log_group" "discord_bot" {
  name              = "/aws/lambda/${var.lambda_name_discord_bot}"
  retention_in_days = 7
}

resource "aws_cloudwatch_log_group" "discord_command_manager" {
  name              = "/aws/lambda/${var.lambda_name_discord_command_manager}"
  retention_in_days = 7
}
