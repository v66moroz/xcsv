require 'bundler/setup'
require 'helix_runtime/build_task'
require 'rspec/core/rake_task'

HelixRuntime::BuildTask.new do |t|
end

RSpec::Core::RakeTask.new(:spec) do |t|
  t.verbose = false
end

task :default => :test
task :test => :build
