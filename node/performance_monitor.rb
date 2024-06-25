# performance_monitor.rb
require 'rubygems'
require 'bundler/setup'
require 'grpc'

class PerformanceMonitor
  def initialize
    @grpc_client = GRPC::Client.new('performance_service', 'localhost:50051')
  end

  def monitor_node(node_id)
    # Real-time performance monitoring using gRPC and Prometheus
    response = @grpc_client.get_node_performance(node_id)
    puts "Node #{node_id} performance: #{response.performance_score}"
  end
end
