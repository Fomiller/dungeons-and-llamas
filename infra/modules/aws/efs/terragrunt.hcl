skip = true
include "root" { path = find_in_parent_folders() }

dependency "security" {
    config_path = "../security"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        aws_security_group_id_lambda_basic = "sg-MOCK"
    }
}

inputs = {
    aws_security_group_id_lambda_basic = dependency.security.outputs.aws_security_group_id_lambda_basic
}
