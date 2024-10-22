# HTTP API Gateway
resource "aws_apigatewayv2_api" "discord_bot" {
  name          = "${var.namespace}-${var.app_prefix}-discord-bot"
  protocol_type = "HTTP"
}

# Create a Custom Domain Name for API Gateway
resource "aws_apigatewayv2_domain_name" "discord_bot" {
  domain_name = "discord-bot.dev.aws.fomillercloud.com" # Your custom domain
  domain_name_configuration {
    certificate_arn = data.aws_acm_certificate.fomiller_cloud.arn # ACM certificate for HTTPS
    endpoint_type   = "REGIONAL"                                  # Use REGIONAL for API Gateway
    security_policy = "TLS_1_2"
  }
}

# API Gateway Custom Domain Mapping to Stage
resource "aws_apigatewayv2_api_mapping" "discord_bot" {
  api_id      = aws_apigatewayv2_api.discord_bot.id
  domain_name = aws_apigatewayv2_domain_name.discord_bot.domain_name
  stage       = aws_apigatewayv2_stage.discord_bot.name # $default stage
}

# Lambda Integration for HTTP API
resource "aws_apigatewayv2_integration" "discord_bot" {
  api_id             = aws_apigatewayv2_api.discord_bot.id
  integration_type   = "AWS_PROXY"
  integration_uri    = var.lambda_invoke_arn_discord_bot
  integration_method = "POST"

  # Integration Credentials: Attach API Gateway permission to invoke Lambda
  payload_format_version = "2.0"
}

# Default route ($default) - Catch-all
resource "aws_apigatewayv2_route" "discord_bot" {
  api_id    = aws_apigatewayv2_api.discord_bot.id
  route_key = "$default"

  target = "integrations/${aws_apigatewayv2_integration.discord_bot.id}"
}

# Deploy API Gateway
resource "aws_apigatewayv2_stage" "discord_bot" {
  api_id      = aws_apigatewayv2_api.discord_bot.id
  name        = "$default"
  auto_deploy = true
}

# Permission for API Gateway to invoke Lambda
resource "aws_lambda_permission" "discord_bot" {
  statement_id  = "AllowAPIGatewayInvokeDnlDiscordBot"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_name_discord_bot
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.discord_bot.execution_arn}/*"
}

