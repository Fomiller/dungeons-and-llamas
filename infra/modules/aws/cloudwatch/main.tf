resource "aws_cloudwatch_log_group" "event_sub" {
  name              = "/aws/lambda/${var.lambda_name_hello}"
  retention_in_days = 7
}

resource "aws_cloudwatch_log_group" "event_sub" {
  name              = "/aws/lambda/${var.lambda_name_command_manager}"
  retention_in_days = 7
}
