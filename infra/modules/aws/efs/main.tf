resource "aws_efs_file_system" "dnl" {
  tags = {
    Name = "${var.namespace}-${var.project_name}-efs"
  }
}

# EFS access point used by lambda file system
resource "aws_efs_access_point" "dnl" {
  file_system_id = aws_efs_file_system.dnl.id

  root_directory {
    path = "/dnl"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "777"
    }
  }

  posix_user {
    gid = 1000
    uid = 1000
  }
}

resource "aws_efs_mount_target" "dnl" {
  for_each        = toset(data.aws_subnets.private.ids)
  file_system_id  = aws_efs_file_system.dnl.id
  subnet_id       = each.value
  security_groups = [var.aws_security_group_id_lambda_basic]
}
