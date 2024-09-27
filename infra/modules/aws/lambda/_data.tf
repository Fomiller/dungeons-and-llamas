data "aws_region" "current" {}
data "aws_caller_identity" "current" {}

data "aws_lambda_function" "dnd_bot_exists" {
  count         = fileexists("${path.module}/bin/hello/bootstrap.zip") ? 0 : 1
  function_name = "${var.namespace}-${var.app_prefix}-bot"
}

