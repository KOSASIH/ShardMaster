# test_env_provisioner.rb
require 'aws-sdk'

class TestEnvProvisioner
  def provision_test_env
    # Create an AWS EC2 instance using the AWS SDK
    ec2 = Aws::EC2::Client.new
    instance = ec2.run_instances({
      image_id: 'ami-abc123',
      instance_type: 't2.micro',
      min_count: 1,
      max_count: 1
    })

    # Configure the instance with the required test environment
    instance_id = instance.instances[0].instance_id
    ec2.wait_until(:instance_status_ok, instance_ids: [instance_id])
    ec2.create_tags({
      resources: [instance_id],
      tags: [
        { key: 'Name', value: 'Test Environment' },
        { key: 'Environment', value: 'Testing' }
      ]
    })
  end
end

test_env_provisioner = TestEnvProvisioner.new
test_env_provisioner.provision_test_env
