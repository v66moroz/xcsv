require 'helix_runtime'
require 'xcsv/native'

class XCSV
  include Enumerable

  def each
    while (rec = self.next) do
      yield rec
    end
  end
end
