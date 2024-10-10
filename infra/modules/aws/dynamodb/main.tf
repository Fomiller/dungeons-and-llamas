resource "aws_dynamodb_table" "dnl_game_state" {
  name         = "${var.namespace}-${var.app_prefix}-${var.environment}-game-state"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "UserId"
  range_key    = "StateComponent"

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"

  attribute {
    name = "UserId"
    type = "S"
  }
  attribute {
    name = "StateComponent"
    type = "S"
  }
}

