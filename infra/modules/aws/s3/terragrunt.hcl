include "root" {
	path = find_in_parent_folders()
}

dependency "lambda" {
    config_path = "../lambda"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        lambda_invoke_arn_discord_command_manager = "my-invoke-arn"
        lambda_arn_discord_command_manager = "my-invoke-arn"
        lambda_name_discord_command_manager = "fomiller-discord-command-manager"
    }
}

inputs = {
    lambda_arn_discord_command_manager = dependency.lambda.outputs.lambda_arn_discord_command_manager
}
