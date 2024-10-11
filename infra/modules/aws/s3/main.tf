resource "aws_s3_bucket" "dnl" {
  bucket = "${var.namespace}-${var.environment}-${var.project_name}"

  object_lock_enabled = false

  versioning {
    enabled = true
  }

  tags = {
    Owner       = "Forrest Miller"
    Email       = "forrestmillerj@gmail.com"
    Environment = var.environment
  }
}

resource "aws_s3_bucket_object" "commands" {
  bucket = aws_s3_bucket.dnl.id
  key    = "data/commands.json"
  source = local.commands_path

  etag = filemd5(local.commands_path)
}
