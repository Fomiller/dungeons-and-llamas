include "root" { path = find_in_parent_folders() }

dependency "api_gateway" {
    config_path = "../api-gateway"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        aws_api_gateway_domain_name_regional_zone_id_discord_bot = "67890"
        aws_api_gateway_domain_name_regional_zone_id_dnl_api = "12345"
        aws_api_gateway_domain_name_regional_domain_name_dnl_api = "dnl-api.MOCK.fomillercloud.com"
        aws_api_gateway_domain_name_regional_domain_name_discord_bot = "dnl-discord-bot.MOCK.fomillercloud.com"
    }
}

inputs = {
    aws_api_gateway_domain_name_regional_zone_id_discord_bot = dependency.api_gateway.outputs.aws_api_gateway_domain_name_regional_zone_id_discord_bot
    aws_api_gateway_domain_name_regional_zone_id_dnl_api = dependency.api_gateway.outputs.aws_api_gateway_domain_name_regional_zone_id_dnl_api
    aws_api_gateway_domain_name_regional_domain_name_discord_bot = dependency.api_gateway.outputs.aws_api_gateway_domain_name_regional_domain_name_discord_bot
    aws_api_gateway_domain_name_regional_domain_name_dnl_api = dependency.api_gateway.outputs.aws_api_gateway_domain_name_regional_domain_name_dnl_api
}
