include "root" {
	path = find_in_parent_folders()
}

dependency "lambda" {
    config_path = "../lambda"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        lambda_name_dnl_api = "fomiller-dnl-api"
        lambda_name_discord_bot = "fomiller-dnl-discord-bot"
        lambda_name_discord_command_manager = "fomiller-dnl-discord-command-manager"
    }
}
inputs = {
    lambda_name_discord_bot = dependency.lambda.outputs.lambda_name_discord_bot
    lambda_name_discord_command_manager = dependency.lambda.outputs.lambda_name_discord_command_manager
    lambda_name_dnl_api = dependency.lambda.outputs.lambda_name_dnl_api
}
