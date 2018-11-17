require 'helix_runtime'
require 'xcsv/native'

class XCSV
  include Enumerable

  def each
    while (r = self.next) do
      yield r
    end
  end
end
