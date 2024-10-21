# Create a Custom Domain Name for API Gateway
resource "aws_apigatewayv2_domain_name" "custom_domain" {
  domain_name = "dnl-api.dev.aws.fomillercloud.com" # Your custom domain
  domain_name_configuration {
    certificate_arn = data.aws_acm_certificate.fomiller_cloud.arn # ACM certificate for HTTPS
    endpoint_type   = "REGIONAL"                                  # Use REGIONAL for API Gateway
    security_policy = "TLS_1_2"
  }
}

# API Gateway Custom Domain Mapping to Stage
resource "aws_apigatewayv2_api_mapping" "custom_domain_mapping" {
  api_id      = aws_apigatewayv2_api.http_api.id
  domain_name = aws_apigatewayv2_domain_name.custom_domain.domain_name
  stage       = aws_apigatewayv2_stage.api_stage.name # $default stage
}
# HTTP API Gateway
resource "aws_apigatewayv2_api" "http_api" {
  name          = "my_http_api"
  protocol_type = "HTTP"
}

# Lambda Integration for HTTP API
resource "aws_apigatewayv2_integration" "lambda_integration" {
  api_id             = aws_apigatewayv2_api.http_api.id
  integration_type   = "AWS_PROXY"
  integration_uri    = var.lambda_invoke_arn_dnl_api
  integration_method = "POST"

  # Integration Credentials: Attach API Gateway permission to invoke Lambda
  payload_format_version = "2.0"
}

# Default route ($default) - Catch-all
resource "aws_apigatewayv2_route" "default_route" {
  api_id    = aws_apigatewayv2_api.http_api.id
  route_key = "$default"

  target = "integrations/${aws_apigatewayv2_integration.lambda_integration.id}"
}

# Deploy API Gateway
resource "aws_apigatewayv2_stage" "api_stage" {
  api_id      = aws_apigatewayv2_api.http_api.id
  name        = "$default"
  auto_deploy = true
}

# Permission for API Gateway to invoke Lambda
resource "aws_lambda_permission" "api_gateway_permission" {
  statement_id  = "AllowAPIGatewayInvoke2"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_name_dnl_api
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.http_api.execution_arn}/*"
}
