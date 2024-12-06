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
        iam_role_arn_lambda_llm_handler = "arn:aws:iam::123456789012:role/MOCK-FomillerLambdaLlmHandler"
    }
}

dependency "efs" {
    config_path = "../efs"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        aws_efs_access_point_arn_dnl = "arn:aws:elasticfilesystem:us-east-1:123456789012:accesspoint/MOCK"
    }
}

dependency "security" {
    config_path = "../security"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        aws_security_group_id_lambda_basic = "sg-MOCK"
    }
}

inputs = {
    iam_role_arn_lambda_discord_bot = dependency.roles.outputs.iam_role_arn_lambda_discord_bot
    iam_role_arn_lambda_discord_command_manager = dependency.roles.outputs.iam_role_arn_lambda_discord_command_manager
    iam_role_arn_lambda_dnl_api = dependency.roles.outputs.iam_role_arn_lambda_dnl_api
    iam_role_arn_lambda_llm_handler = dependency.roles.outputs.iam_role_arn_lambda_llm_handler
    aws_efs_access_point_arn_dnl = dependency.efs.outputs.aws_efs_access_point_arn_dnl
    aws_security_group_id_lambda_basic = dependency.security.outputs.aws_security_group_id_lambda_basic
}

