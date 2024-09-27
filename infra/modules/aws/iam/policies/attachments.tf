resource "aws_iam_role_policy_attachment" "lambda_dnd_bot" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role_name_lambda_dnd_bot
}

