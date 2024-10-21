output "discord_bot_url" {
  value = aws_api_gateway_deployment.example.invoke_url
}

output "dnl_api_url" {
  value = aws_api_gateway_deployment.dnl_api.invoke_url
}
