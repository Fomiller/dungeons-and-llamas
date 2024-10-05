output "iam_role_arn_lambda_dnd_bot" {
  value = aws_iam_role.lambda_dnd_bot.arn
}

output "iam_role_name_lambda_dnd_bot" {
  value = aws_iam_role.lambda_dnd_bot.name
}

output "iam_role_arn_lambda_command_manager" {
  value = aws_iam_role.lambda_command_manager.arn
}

output "iam_role_name_lambda_command_manager" {
  value = aws_iam_role.lambda_command_manager.name
}
