resource "aws_api_gateway_domain_name" "dnl_api" {
  domain_name = "dnl-api.${var.environment}.aws.fomillercloud.com"

  regional_certificate_arn = data.aws_acm_certificate.fomiller_cloud.arn

  endpoint_configuration {
    types = ["REGIONAL"] # Use "EDGE" if needed
  }
}

resource "aws_api_gateway_base_path_mapping" "dnl_api" {
  api_id = aws_api_gateway_rest_api.dnl_api.id
  # stage_name  = aws_api_gateway_deployment.dnl_api.stage_name
  domain_name = aws_api_gateway_domain_name.dnl_api.domain_name
  stage_name  = ""
}

resource "aws_api_gateway_deployment" "dnl_api" {
  depends_on = [
    aws_api_gateway_integration.dnl_api_lambda,
    aws_api_gateway_integration.dnl_api_lambda_root,
    aws_api_gateway_domain_name.dnl_api
  ]

  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  stage_name  = ""
}

resource "aws_api_gateway_rest_api" "dnl_api" {
  name        = "DungeonsAndLlamas"
  description = "Api Gateway for Dungeons and Llamas API"
}

# define paths
resource "aws_api_gateway_resource" "dnl_api" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  parent_id   = aws_api_gateway_rest_api.dnl_api.root_resource_id
  path_part   = "{proxy+}"
}

# define methods allowed
resource "aws_api_gateway_method" "dnl_api_proxy" {
  rest_api_id   = aws_api_gateway_rest_api.dnl_api.id
  resource_id   = aws_api_gateway_resource.dnl_api.id
  http_method   = "ANY"
  authorization = "NONE"
}

# handle root methods
resource "aws_api_gateway_method" "dnl_api_proxy_root" {
  rest_api_id   = aws_api_gateway_rest_api.dnl_api.id
  resource_id   = aws_api_gateway_rest_api.dnl_api.root_resource_id
  http_method   = "ANY"
  authorization = "NONE"
}

# define what gateway is attached to in this case lambda
resource "aws_api_gateway_integration" "dnl_api_lambda" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  resource_id = aws_api_gateway_method.dnl_api_proxy.resource_id
  http_method = aws_api_gateway_method.dnl_api_proxy.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = var.lambda_invoke_arn_dnl_api
}

# handle root
resource "aws_api_gateway_integration" "dnl_api_lambda_root" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  resource_id = aws_api_gateway_method.dnl_api_proxy_root.resource_id
  http_method = aws_api_gateway_method.dnl_api_proxy_root.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = var.lambda_invoke_arn_dnl_api
}


resource "aws_lambda_permission" "dnl_api" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_name_dnl_api
  principal     = "apigateway.amazonaws.com"

  # The /*/* portion grants access from any method on any resource
  # within the API Gateway "REST API".
  source_arn = "${aws_api_gateway_rest_api.dnl_api.execution_arn}/*/*"
}

# CORS
# OPTIONS method for CORS preflight
# resource "aws_api_gateway_method" "dnl_api_options_method" {
#   rest_api_id   = aws_api_gateway_rest_api.dnl_api.id
#   resource_id   = aws_api_gateway_resource.dnl_api.id
#   http_method   = "OPTIONS"
#   authorization = "NONE"
# }

# Method response for OPTIONS to define CORS headers
resource "aws_api_gateway_method_response" "options_response" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  resource_id = aws_api_gateway_resource.dnl_api.id
  http_method = aws_api_gateway_method.dnl_api_proxy_root.http_method
  status_code = "200"

  response_parameters = {
    "method.response.header.Access-Control-Allow-Origin"  = true
    "method.response.header.Access-Control-Allow-Methods" = true
    "method.response.header.Access-Control-Allow-Headers" = true
  }

  // seems to help with consistent plans
  depends_on = [aws_api_gateway_method.dnl_api_proxy_root]
}

resource "aws_api_gateway_integration_response" "dnl_api_options_integration_response" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  resource_id = aws_api_gateway_resource.dnl_api.id
  http_method = aws_api_gateway_method.dnl_api_proxy_root.http_method
  status_code = "200"

  response_parameters = {
    "method.response.header.Access-Control-Allow-Origin"  = "'*'"
    "method.response.header.Access-Control-Allow-Methods" = "'GET,POST,OPTIONS'"
    "method.response.header.Access-Control-Allow-Headers" = "'Content-Type,Authorization'"
  }

  response_templates = {
    "application/json" = ""
  }

  // seems to help with consistent plans
  depends_on = [aws_api_gateway_method.dnl_api_proxy_root]
}
