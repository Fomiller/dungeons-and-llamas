output "lambda_name_discord_bot" {
  value = aws_lambda_function.discord_bot.function_name
}

output "lambda_arn_discord_bot" {
  value = aws_lambda_function.discord_bot.arn
}

output "lambda_invoke_arn_discord_bot" {
  value = aws_lambda_function.discord_bot.invoke_arn
}

output "lambda_name_discord_command_manager" {
  value = aws_lambda_function.discord_command_manager.function_name
}

output "lambda_arn_discord_command_manager" {
  value = aws_lambda_function.discord_command_manager.arn
}

output "lambda_invoke_arn_discord_command_manager" {
  value = aws_lambda_function.discord_command_manager.invoke_arn
}
