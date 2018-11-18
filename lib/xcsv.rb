require 'helix_runtime'
require 'xcsv/native'
require 'xcsv/version'

class XCSV
  include Enumerable

  def each
    while (rec = self.next) do
      yield rec
    end
  end
end
