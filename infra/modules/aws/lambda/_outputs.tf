output "lambda_name_hello" {
  value = aws_lambda_function.dnd_bot.function_name
}

output "lambda_arn_hello" {
  value = aws_lambda_function.dnd_bot.arn
}

output "lambda_invoke_arn_hello" {
  value = aws_lambda_function.dnd_bot.invoke_arn
}

output "lambda_name_command_manager" {
  value = aws_lambda_function.command_manager.function_name
}

output "lambda_arn_command_manager" {
  value = aws_lambda_function.command_manager.arn
}

output "lambda_invoke_arn_command_manager" {
  value = aws_lambda_function.command_manager.invoke_arn
}
