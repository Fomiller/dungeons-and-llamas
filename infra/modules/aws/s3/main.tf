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

resource "aws_s3_bucket_ownership_controls" "example" {
  bucket = aws_s3_bucket.dnl.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}

resource "aws_s3_bucket_object" "commands" {
  bucket = aws_s3_bucket.dnl.id
  key    = local.commands.key
  source = local.commands.local_path

  etag = filemd5(local.commands.local_path)
}

resource "aws_s3_bucket_object" "dnl_api_favicon" {
  bucket = aws_s3_bucket.dnl.id
  key    = local.favicon.key
  source = local.favicon.local_path

  etag = filemd5(local.favicon.local_path)
}

resource "aws_s3_bucket_policy" "dnl_api_gateway" {
  bucket = aws_s3_bucket.dnl.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "arn:aws:s3:::${aws_s3_bucket.dnl.bucket}/data/api/favicon.ico"
      }
    ]
  })
}
