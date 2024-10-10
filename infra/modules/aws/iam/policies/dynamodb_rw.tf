data "aws_iam_policy_document" "dynamodb_rw" {
  statement {
    sid    = "DyanamoDBReadWtie"
    effect = "Allow"
    actions = [
      "dynamodb:BatchGetItem",
      "dynamodb:BatchWriteItem",
      "dynamodb:ConditionCheckItem",
      "dynamodb:PutItem",
      "dynamodb:DescribeTable",
      "dynamodb:DeleteItem",
      "dynamodb:GetItem",
      "dynamodb:Scan",
      "dynamodb:Query",
      "dynamodb:UpdateItem"
    ]
    resources = [
      "arn:aws:dynamodb:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:table/${var.namespace}-${var.app_prefix}*"
    ]
  }
}
resource "aws_iam_policy" "dnl_dynamodb_rw" {
  name   = "${local.policy_prefix}DynamoDBReadWrite"
  policy = data.aws_iam_policy_document.dynamodb_rw.json
}


