# output "discord_bot_url" {
#   value = aws_api_gateway_deployment.example.invoke_url
# }

# output "dnl_api_url" {
#   value = aws_api_gateway_deployment.dnl_api.invoke_url
# }

output "aws_api_gateway_domain_name_regional_zone_id_dnl_api" {
  value = aws_apigatewayv2_domain_name.dnl_api.domain_name_configuration[0].hosted_zone_id
}

output "aws_api_gateway_domain_name_regional_domain_name_dnl_api" {
  value = aws_apigatewayv2_domain_name.dnl_api.domain_name_configuration[0].target_domain_name
}

output "aws_api_gateway_domain_name_regional_zone_id_discord_bot" {
  value = aws_apigatewayv2_domain_name.discord_bot.domain_name_configuration[0].hosted_zone_id
}

output "aws_api_gateway_domain_name_regional_domain_name_discord_bot" {
  value = aws_apigatewayv2_domain_name.discord_bot.domain_name_configuration[0].target_domain_name
}

#
# output "dnl_api_stage" {
#   value = aws_api_gateway_deployment.dnl_api.stage_name
# }
