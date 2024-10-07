include "root" {
	path = find_in_parent_folders()
}

dependency "lambda" {
    config_path = "../lambda"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        lambda_name_hello = "fomiller-project-hello"
        lambda_name_command_manager = "fomiller-project-command-manager"
    }
}
inputs = {
    lambda_name_hello = dependency.lambda.outputs.lambda_name_hello
    lambda_name_command_manager = dependency.lambda.outputs.lambda_name_command_manager
}
