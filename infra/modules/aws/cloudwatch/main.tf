resource "aws_cloudwatch_log_group" "dnl_discord_bot" {
  name              = "/aws/lambda/${var.lambda_name_hello}"
  retention_in_days = 7
}

resource "aws_cloudwatch_log_group" "command_manager" {
  name              = "/aws/lambda/${var.lambda_name_command_manager}"
  retention_in_days = 7
}
