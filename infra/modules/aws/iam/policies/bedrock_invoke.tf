resource "aws_iam_policy" "bedrock_invoke" {
  name        = "LambdaBedrockInvokePolicy"
  description = "Policy for Lambda to invoke AWS Bedrock foundational models"

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Sid    = "AllowBedrockInvocation",
        Effect = "Allow",
        Action = [
          "bedrock:InvokeModel",
          "bedrock:ListFoundationModels"
        ],
        Resource = "*"
      }
    ]
  })
}
