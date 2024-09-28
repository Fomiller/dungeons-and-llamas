output "lambda_name_hello" {
  value = aws_lambda_function.dnd_bot.name
}

output "lambda_arn_hello" {
  value = aws_lambda_function.dnd_bot.arn
}
