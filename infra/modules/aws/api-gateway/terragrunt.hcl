include "root" {
	path = find_in_parent_folders()
}

dependency "s3" {
    config_path = "../s3"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        s3_bucket_name_dnl = "mock-dnl-bucket-name"
        s3_bucket_arn_dnl = "arn:aws:s3:::fomiller-MOCK-dungeons-and-llamas"
        s3_bucket_object_key_dnl_api_favicon = "/mock/favicon.ico"
    }
}
dependency "lambda" {
    config_path = "../lambda"
    mock_outputs_merge_strategy_with_state = "shallow"
    mock_outputs_allowed_terraform_commands = ["validate", "plan", "apply", "destroy"]
    mock_outputs = {
        lambda_invoke_arn_discord_bot = "my-invoke-arn"
        lambda_invoke_arn_dnl_api = "my-invoke-arn"
        lambda_name_discord_bot = "fomiller-discord-bot"
        lambda_name_dnl_api = "fomiller-dnl-api"
    }
}
inputs = {
    lambda_invoke_arn_discord_bot = dependency.lambda.outputs.lambda_invoke_arn_discord_bot
    lambda_invoke_arn_dnl_api = dependency.lambda.outputs.lambda_invoke_arn_dnl_api
    lambda_name_discord_bot = dependency.lambda.outputs.lambda_name_discord_bot
    lambda_name_dnl_api = dependency.lambda.outputs.lambda_name_dnl_api
    s3_bucket_name_dnl = dependency.s3.outputs.s3_bucket_name_dnl
    s3_bucket_arn_dnl = dependency.s3.outputs.s3_bucket_arn_dnl
    s3_bucket_object_key_dnl_api_favicon = dependency.s3.outputs.s3_bucket_object_key_dnl_api_favicon
}
