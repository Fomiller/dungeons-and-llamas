resource "aws_api_gateway_rest_api" "dnl_api" {
  name        = "Dungeons and Llamas"
  description = "Api Gateway for Duneons and Llamas API"
}

# define paths
resource "aws_api_gateway_resource" "dnl_api" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  parent_id   = aws_api_gateway_rest_api.dnl_api.root_resource_id
  path_part   = "{proxy+}"
}

# define methods allowed
resource "aws_api_gateway_method" "dnl_api" {
  rest_api_id   = aws_api_gateway_rest_api.dnl_api.id
  resource_id   = aws_api_gateway_resource.dnl_api.id
  http_method   = "ANY"
  authorization = "NONE"
}

# # handle root methods
# resource "aws_api_gateway_method" "dnl_api_proxy_root" {
#   rest_api_id   = aws_api_gateway_rest_api.dnl_api.id
#   resource_id   = aws_api_gateway_rest_api.dnl_api.root_resource_id
#   http_method   = "ANY"
#   authorization = "NONE"
# }

# define what gateway is attached to in this case lambda
resource "aws_api_gateway_integration" "dnl_api" {
  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
  resource_id = aws_api_gateway_method.dnl_api.resource_id
  http_method = aws_api_gateway_method.dnl_api.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = var.lambda_invoke_arn_dnl_api
}

# # handle root
# resource "aws_api_gateway_integration" "lambda_root" {
#   rest_api_id = aws_api_gateway_rest_api.dnl_api.id
#   resource_id = aws_api_gateway_method.proxy_root.resource_id
#   http_method = aws_api_gateway_method.proxy_root.http_method
#
#   integration_http_method = "POST"
#   type                    = "AWS_PROXY"
#   uri                     = var.lambda_invoke_arn_discord_bot
# }


resource "aws_api_gateway_deployment" "dnl_api" {
  depends_on = [
    aws_api_gateway_integration.dnl_api,
    # aws_api_gateway_integration.lambda_root,
  ]

  rest_api_id = aws_api_gateway_rest_api.dnl_api.id
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
