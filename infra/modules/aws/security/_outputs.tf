output "aws_security_group_id_lambda_basic" {
  value = aws_security_group_id.lambda_basic_sg.id
}

output "aws_security_group_id_efs" {
  value = aws_security_group_id.efs_sg.id
}
