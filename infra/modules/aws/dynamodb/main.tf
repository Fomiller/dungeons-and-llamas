resource "aws_dynamodb_table" "dnl_game_state" {
  name         = "${var.namespace}-${var.app_prefix}-game-state"
  hash_key     = "UserId"
  billing_mode = "PAY_PER_REQUEST"

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"

  attribute {
    name = "UserId"
    type = "S"
  }
}

