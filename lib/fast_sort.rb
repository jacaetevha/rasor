require "fast_sort/version"

module FastSort
  class Error < StandardError; end

  def self.sort(array)
    array.sort
  end

  class << self
    # @since 0.0.1
    # @api private
    alias sort_ruby sort
  end
end

require "helix_runtime"

begin
  require "fast_sort/native"
rescue LoadError
  warn "Unable to load fast_sort/native. Please run `rake build`"
end
