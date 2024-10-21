// relies on aws-infrastructure repo being deployed
data "aws_acm_certificate" "fomiller_cloud" {
  domain   = "${var.environment}.aws.fomillercloud.com"
  statuses = ["ISSUED"]
}
