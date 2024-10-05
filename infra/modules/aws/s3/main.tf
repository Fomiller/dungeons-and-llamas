resource "aws_s3_bucket" "dungeons_and_discord" {
  bucket = "${var.namespace}-${var.environment}-dungeons-and-discord"

  object_lock_enabled = false

  tags = {
    Owner       = "Forrest Miller"
    Email       = "forrestmillerj@gmail.com"
    Environment = var.environment
  }
}

resource "aws_s3_bucket_object" "commands" {
  bucket = aws_s3_bucket.dungeons_and_discord.id
  key    = "data/commands.json"
  source = local.commands_path

  etag = filemd5(local.commands_path)
}
