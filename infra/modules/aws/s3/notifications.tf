// If I ever need more granularity use eventbridge notifications
resource "aws_s3_bucket_notification" "dnl_commands" {
  bucket = aws_s3_bucket.dnl.id

  lambda_function {
    lambda_function_arn = var.lambda_arn_discord_command_manager
    events              = ["s3:ObjectCreated:*"]
    filter_prefix       = "data/"
    filter_suffix       = ".json"
  }
}

resource "aws_lambda_permission" "allow_bucket" {
  statement_id  = "AllowExecutionFromS3Bucket"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_arn_discord_command_manager
  principal     = "s3.amazonaws.com"
  source_arn    = aws_s3_bucket.dnl.arn
}
