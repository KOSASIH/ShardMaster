# test_env_provisioner.tf
provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "test_env" {
  ami           = "ami-abc123"
  instance_type = "t2.micro"
  vpc_security_group_ids = [aws_security_group.test_env_sg.id]
  key_name               = "my_key_pair"
}

resource "aws_security_group" "test_env_sg" {
  name        = "test_env_sg"
  description = "Security group for test environment"
  vpc_id      = "vpc-12345678"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
