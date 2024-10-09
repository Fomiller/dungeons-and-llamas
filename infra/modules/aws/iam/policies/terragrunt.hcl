include "root" {
  path = find_in_parent_folders()
}

dependency "roles" {
    config_path = "../roles"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        iam_role_arn_lambda_discord_bot = "arn:aws:iam::123456789012:role/MOCK-FomillerLambda"
        iam_role_name_lambda_discord_bot = "FomillerLambdaDungeonsAndLlamasDiscordBot"
        iam_role_arn_lambda_discord_command_manager = "arn:aws:iam::123456789012:role/MOCK-FomillerLambda"
        iam_role_name_lambda_discord_command_manager = "FomillerLambdaDungeonsAndLlamasDiscordBotCommandManager"
    }
}
inputs = {
    iam_role_arn_lambda_discord_bot = dependency.roles.outputs.iam_role_arn_lambda_discord_bot
    iam_role_name_lambda_discord_bot = dependency.roles.outputs.iam_role_name_lambda_discord_bot
    iam_role_arn_lambda_discord_command_manager = dependency.roles.outputs.iam_role_arn_lambda_discord_command_manager
    iam_role_name_lambda_discord_command_manager = dependency.roles.outputs.iam_role_name_lambda_discord_command_manager
}

