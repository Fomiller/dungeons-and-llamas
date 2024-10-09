output "iam_role_arn_lambda_discord_bot" {
  value = aws_iam_role.lambda_discord_bot.arn
}

output "iam_role_name_lambda_discord_bot" {
  value = aws_iam_role.lambda_discord_bot.name
}

output "iam_role_arn_lambda_discord_command_manager" {
  value = aws_iam_role.lambda_discord_command_manager.arn
}

output "iam_role_name_lambda_discord_command_manager" {
  value = aws_iam_role.lambda_discord_command_manager.name
}
