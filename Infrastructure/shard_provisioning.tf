# shard_provisioning.tf
provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "shard" {
  count = 3
  ami           = "ami-abc123"
  instance_type = "t2.micro"
  vpc_security_group_ids = [aws_security_group.shard_sg.id]
}

resource "aws_security_group" "shard_sg" {
  name        = "shard-sg"
  description = "Security group for shards"
  vpc_id      = "vpc-abc123"

  ingress {
    from_port   = 8080
    to_port     = 8080
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
