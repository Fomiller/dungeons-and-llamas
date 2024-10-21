include "root" {
  path = find_in_parent_folders()
}

dependency "roles" {
    config_path = "../iam/roles"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        iam_role_arn_lambda_discord_bot = "arn:aws:iam::123456789012:role/MOCK-FomillerLambda"
        iam_role_arn_lambda_discord_command_manager = "arn:aws:iam::123456789012:role/MOCK-FomillerLambda"
        iam_role_arn_lambda_dnl_api = "arn:aws:iam::123456789012:role/MOCK-FomillerLambdaApi"
    }
}
inputs = {
    iam_role_arn_lambda_discord_bot = dependency.roles.outputs.iam_role_arn_lambda_discord_bot
    iam_role_arn_lambda_discord_command_manager = dependency.roles.outputs.iam_role_arn_lambda_discord_command_manager
    iam_role_arn_lambda_dnl_api = dependency.roles.outputs.iam_role_arn_lambda_dnl_api
}

