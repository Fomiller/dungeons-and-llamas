resource "aws_s3_bucket" "dungeons_and_discord" {
  bucket = "${var.namespace}-${var.environment}-dungeons-and-discord"

  object_lock_enabled = false

  tags = {
    Owner       = "Forrest Miller"
    Email       = "forrestmillerj@gmail.com"
    Environment = var.environment
  }
}


