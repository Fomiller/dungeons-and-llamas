resource "aws_iam_role_policy_attachment" "lambda_dnl_api" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role_name_lambda_dnl_api
}

resource "aws_iam_role_policy_attachment" "lambda_dnl_api_bedrock" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonBedrockFullAccess"
  role       = var.iam_role_name_lambda_dnl_api
}

resource "aws_iam_role_policy_attachment" "lambda_dnl_api_vpc_access" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = var.iam_role_name_lambda_dnl_api
}

resource "aws_iam_role_policy_attachment" "lambda_dnl_api_dynamodb" {
  policy_arn = aws_iam_policy.dnl_dynamodb_rw.arn
  role       = var.iam_role_name_lambda_dnl_api
}
