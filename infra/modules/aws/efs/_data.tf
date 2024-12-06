data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

data "aws_subnets" "private" {
  filter {
    name   = "tag:Tier"
    values = ["private"]
  }
}
