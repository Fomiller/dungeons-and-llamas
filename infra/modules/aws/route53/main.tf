# Step 5: Create a DNS Record in Route 53
resource "aws_route53_record" "discord_bot" {
  zone_id = data.aws_route53_zone.fomiller_cloud.id # Use the data source to get the hosted zone ID
  name    = "dnl-discord-bot"                       # Just the subdomain part
  type    = "A"

  alias {
    name                   = var.aws_api_gateway_domain_name_regional_domain_name_discord_bot
    zone_id                = var.aws_api_gateway_domain_name_regional_zone_id_discord_bot
    evaluate_target_health = false
  }
}

# Step 5: Create a DNS Record in Route 53
resource "aws_route53_record" "dnl_api" {
  zone_id = data.aws_route53_zone.fomiller_cloud.id # Use the data source to get the hosted zone ID
  name    = "dnl-api"                               # Just the subdomain part
  type    = "A"

  alias {
    name                   = var.aws_api_gateway_domain_name_regional_domain_name_dnl_api
    zone_id                = var.aws_api_gateway_domain_name_regional_zone_id_dnl_api
    evaluate_target_health = false
  }
}

