output "iam_role_arn_lambda_dnl_discord_bot" {
  value = aws_iam_role.lambda_dnl_discord_bot.arn
}

output "iam_role_name_lambda_dnl_discord_bot" {
  value = aws_iam_role.lambda_dnl_discord_bot.name
}

output "iam_role_arn_lambda_command_manager" {
  value = aws_iam_role.lambda_command_manager.arn
}

output "iam_role_name_lambda_command_manager" {
  value = aws_iam_role.lambda_command_manager.name
}
