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

output "lambda_name_dnl_api" {
  value = aws_lambda_function.dnl_api.function_name
}

output "lambda_arn_dnl_api" {
  value = aws_lambda_function.dnl_api.arn
}

output "lambda_invoke_arn_dnl_api" {
  value = aws_lambda_function.dnl_api.invoke_arn
}
