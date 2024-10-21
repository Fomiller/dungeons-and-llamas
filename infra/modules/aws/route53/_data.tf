data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

# Data Source to Find the Hosted Zone
data "aws_route53_zone" "fomiller_cloud" {
  name = "${var.environment}.aws.fomillercloud.com."
}
