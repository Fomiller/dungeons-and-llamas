resource "aws_api_gateway_rest_api" "example" {
  name        = "DnlDiscordBot"
  description = "Api Gateway for Dungeons and Llamas Discord bot"
}

resource "aws_api_gateway_domain_name" "discord_bot" {
  domain_name = "dnl-discord-bot.${var.environment}.aws.fomillercloud.com"

  regional_certificate_arn = data.aws_acm_certificate.fomiller_cloud.arn

  endpoint_configuration {
    types = ["REGIONAL"] # Use "EDGE" if needed
  }
}

resource "aws_api_gateway_deployment" "example" {
  rest_api_id = aws_api_gateway_rest_api.example.id

  depends_on = [
    aws_api_gateway_integration.discord_bot_lambda,
    aws_api_gateway_integration.discord_bot_lambda_root,
    aws_api_gateway_domain_name.discord_bot
  ]
}

resource "aws_api_gateway_base_path_mapping" "discord_bot" {
  api_id      = aws_api_gateway_rest_api.example.id
  stage_name  = aws_api_gateway_deployment.example.stage_name
  domain_name = aws_api_gateway_domain_name.discord_bot.domain_name
}

# define paths
resource "aws_api_gateway_resource" "proxy" {
  rest_api_id = aws_api_gateway_rest_api.example.id
  parent_id   = aws_api_gateway_rest_api.example.root_resource_id
  path_part   = "{proxy+}"
}

# define methods allowed
resource "aws_api_gateway_method" "discord_bot_proxy" {
  rest_api_id   = aws_api_gateway_rest_api.example.id
  resource_id   = aws_api_gateway_resource.proxy.id
  http_method   = "ANY"
  authorization = "NONE"
}

# handle root methods
resource "aws_api_gateway_method" "discord_bot_proxy_root" {
  rest_api_id   = aws_api_gateway_rest_api.example.id
  resource_id   = aws_api_gateway_rest_api.example.root_resource_id
  http_method   = "ANY"
  authorization = "NONE"
}

# define what gateway is attached to in this case lambda
resource "aws_api_gateway_integration" "discord_bot_lambda" {
  rest_api_id = aws_api_gateway_rest_api.example.id
  resource_id = aws_api_gateway_method.discord_bot_proxy.resource_id
  http_method = aws_api_gateway_method.discord_bot_proxy.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = var.lambda_invoke_arn_discord_bot
}

# handle root
resource "aws_api_gateway_integration" "discord_bot_lambda_root" {
  rest_api_id = aws_api_gateway_rest_api.example.id
  resource_id = aws_api_gateway_method.discord_bot_proxy_root.resource_id
  http_method = aws_api_gateway_method.discord_bot_proxy_root.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = var.lambda_invoke_arn_discord_bot
}

resource "aws_lambda_permission" "apigw" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_name_discord_bot
  principal     = "apigateway.amazonaws.com"

  # The /*/* portion grants access from any method on any resource
  # within the API Gateway "REST API".
  source_arn = "${aws_api_gateway_rest_api.example.execution_arn}/*/*"
}
