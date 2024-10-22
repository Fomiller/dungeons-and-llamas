# HTTP API Gateway
resource "aws_apigatewayv2_api" "dnl_api" {
  name          = "${var.namespace}-${var.app_prefix}-api"
  protocol_type = "HTTP"
}

# Create a Custom Domain Name for API Gateway
resource "aws_apigatewayv2_domain_name" "dnl_api" {
  domain_name = "dnl-api.dev.aws.fomillercloud.com" # Your custom domain
  domain_name_configuration {
    certificate_arn = data.aws_acm_certificate.fomiller_cloud.arn # ACM certificate for HTTPS
    endpoint_type   = "REGIONAL"                                  # Use REGIONAL for API Gateway
    security_policy = "TLS_1_2"
  }
}

# Deploy API Gateway
resource "aws_apigatewayv2_stage" "dnl_api" {
  api_id      = aws_apigatewayv2_api.dnl_api.id
  name        = "$default"
  auto_deploy = true
}

# API Gateway Custom Domain Mapping to Stage
resource "aws_apigatewayv2_api_mapping" "dnl_api" {
  api_id      = aws_apigatewayv2_api.dnl_api.id
  domain_name = aws_apigatewayv2_domain_name.dnl_api.domain_name
  stage       = aws_apigatewayv2_stage.dnl_api.name # $default stage
}

# Lambda Integration for HTTP API
resource "aws_apigatewayv2_integration" "dnl_api" {
  api_id             = aws_apigatewayv2_api.dnl_api.id
  integration_type   = "AWS_PROXY"
  integration_uri    = var.lambda_invoke_arn_dnl_api
  integration_method = "POST"

  # Integration Credentials: Attach API Gateway permission to invoke Lambda
  payload_format_version = "2.0"
}

# API Gateway S3 Integration for favicon.ico
resource "aws_apigatewayv2_integration" "dnl_api_favicon_s3_integration" {
  api_id             = aws_apigatewayv2_api.dnl_api.id
  integration_type   = "HTTP_PROXY"
  integration_uri    = "https://${var.s3_bucket_name_dnl}.s3.amazonaws.com/${var.s3_bucket_object_key_dnl_api_favicon}"
  integration_method = "GET"

  # Keeping this here to come back to
  # credentials_arn = aws_iam_role.apigateway_s3_role.arn
}

# Route for favicon.ico
resource "aws_apigatewayv2_route" "dnl_api_favicon_route" {
  api_id    = aws_apigatewayv2_api.dnl_api.id
  route_key = "GET /favicon.ico"
  target    = "integrations/${aws_apigatewayv2_integration.dnl_api_favicon_s3_integration.id}"
}

# Default route ($default) - Catch-all
resource "aws_apigatewayv2_route" "dnl_api" {
  api_id    = aws_apigatewayv2_api.dnl_api.id
  route_key = "$default"

  target = "integrations/${aws_apigatewayv2_integration.dnl_api.id}"
}

# Permission for API Gateway to invoke Lambda
resource "aws_lambda_permission" "dnl_api" {
  statement_id  = "AllowAPIGatewayInvokeDnlApi"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_name_dnl_api
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.dnl_api.execution_arn}/*"
}

# Keeping this here to come back to
# # Create an IAM role for API Gateway to access S3
# resource "aws_iam_role" "apigateway_s3_role" {
#   name = "apigateway-s3-role"
#
#   assume_role_policy = jsonencode({
#     Version = "2012-10-17"
#     Statement = [{
#       Action = "sts:AssumeRole"
#       Effect = "Allow"
#       Principal = {
#         Service = "apigateway.amazonaws.com"
#       }
#     }]
#   })
# }
#
# resource "aws_iam_role_policy_attachment" "dnl_api_s3_full_access" {
#   policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
#   role       = aws_iam_role.apigateway_s3_role.name
# }
