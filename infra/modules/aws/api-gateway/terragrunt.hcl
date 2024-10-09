include "root" {
	path = find_in_parent_folders()
}

dependency "lambda" {
    config_path = "../lambda"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        lambda_invoke_arn_discord_bot = "my-invoke-arn"
        lambda_name_discord_bot = "fomiller-discord-bot"
    }
}
inputs = {
    lambda_invoke_arn_discord_bot = dependency.lambda.outputs.lambda_invoke_arn_discord_bot
    lambda_name_discord_bot = dependency.lambda.outputs.lambda_name_discord_bot
}
