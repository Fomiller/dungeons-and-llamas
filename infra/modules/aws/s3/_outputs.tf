output "s3_bucket_name_dnl" {
  value = aws_s3_bucket.dnl.bucket
}

output "s3_bucket_arn_dnl" {
  value = aws_s3_bucket.dnl.arn
}

# output "s3_bucket_object_key_dnl_api_favicon" {
#   value = aws_s3_bucket_object.dnl_api_favicon.key
# }
