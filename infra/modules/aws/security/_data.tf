data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

data "aws_vpc" "fomiller" {
  filter {
    name   = "tag:Name"
    values = ["${var.namespace}-vpc"]
  }
}
