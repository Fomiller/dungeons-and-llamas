resource "aws_iam_role_policy_attachment" "lambda_dnl_discord_bot" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role_name_lambda_dnl_discord_bot
}

resource "aws_iam_role_policy_attachment" "lambda_command_manager_basic" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role_name_lambda_command_manager
}

resource "aws_iam_role_policy_attachment" "lambda_command_manager_s3" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
  role       = var.iam_role_name_lambda_command_manager
}

