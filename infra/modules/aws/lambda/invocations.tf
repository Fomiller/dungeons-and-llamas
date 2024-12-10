resource "aws_lambda_invocation" "db_manager" {
  function_name = aws_lambda_function.db_manager.function_name

  triggers = {
    redeployment = filebase64sha256(local.filename["db_manager"])
  }

  input = jsonencode({
    command = "test"
  })
  depends_on = [aws_lambda_function.db_manager]
}
