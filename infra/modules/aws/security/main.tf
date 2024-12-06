resource "aws_security_group" "lambda_basic_sg" {
  name        = "${var.namespace}-${var.app_prefix}-lambda-basic-sg"
  description = "Basic security group for Lambda function"
  vpc_id      = data.aws_vpc.fomiller.id

  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "efs_sg" {
  name        = "${var.namespace}-${var.app_prefix}-efs-sg"
  description = "Allow access EFS"
  vpc_id      = data.aws_vpc.fomiller.id

  ingress {
    description     = "Allow NFS access from Lambda SG"
    from_port       = 2049
    to_port         = 2049
    protocol        = "tcp"
    security_groups = [aws_security_group.lambda_basic_sg.id]
  }

  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
