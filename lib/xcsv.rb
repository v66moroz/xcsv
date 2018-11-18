require 'helix_runtime'
require 'xcsv/native'
require 'xcsv/version'

class XCSV
  include Enumerable

  def self.open(path)
    xcsv = new(path)
    xcsv.open
    begin
      yield xcsv
    ensure
      xcsv.close
    end
  end

  def each
    while (rec = self.next) do
      yield rec
    end
  end
end
