resource "aws_iam_role_policy_attachment" "lambda_db_manager" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role_name_lambda_db_manager
}

resource "aws_iam_role_policy_attachment" "lambda_db_manager_vpc_access" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = var.iam_role_name_lambda_db_manager
}
