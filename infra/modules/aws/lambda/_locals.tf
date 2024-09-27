locals {
  source_code_hash = {
    dnd_bot = fileexists("${path.module}/bin/hello/bootstrap.zip") ? filebase64sha256("${path.module}/bin/hello/bootstrap.zip") : data.aws_lambda_function.dnd_bot_exists[0].source_code_hash
  }
}
