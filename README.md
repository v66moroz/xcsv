Fast CSV reader based on [Rust CSV crate](https://docs.rs/csv/1.0.2/csv/)

### Installation

1. Install [Rust](https://www.rust-lang.org/)

   `curl https://sh.rustup.rs -sSf | sh`

   Don't miss this message:

   ```
   Rust is installed now. Great!

   To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH 
   environment variable. Next time you log in this will be done automatically.

   To configure your current shell run source $HOME/.cargo/env
   ```

2. `gem install xcsv`

### Usage

```ruby
require 'xcsv'

# Enumerable
csv_reader = XSV.new("foo.csv")
csv_reader.each do |rec|
  rec #=> [col1, col2, col3, ...]
end

csv_reader = XSV.new("foo.csv")
csv_reader.take(10).to_a #=> [[col1, ...], [col1, ...], ...]

# while loop
csv_reader = XSV.new("bar.csv") 
while (rec = csv_reader.next) do
  rec #=> [col1, col2, col3, ...]
end

# Both forms will gunzip if file name ends with .gz
csv_reader = XSV.new("foo_bar.csv.gz") 
while (rec = csv_reader.next) do
  rec #=> [col1, col2, col3, ...]
end
```

### Benchmarks

#### Code

```ruby
# FastestCSV
FastestCSV.foreach('sample.csv') do |rec|
end

# XCSV
csv_reader = XCSV.new('sample.csv')
while (rec = csv_reader.next) do
end

# CSV
CSV.foreach('sample.csv') do |rec|
end
```

#### Parameters

|Records|File size|CPU|
|---|---|---|
|1M|742M (426M .gz)|i7-6600U @ 2.60GHz|

#### Elapsed time (secs):

|FastestCSV|XCSV|XCSV (.gz)|CSV|
|---|---|---|---|
|10.1|12.4|20.7|50.5|

**Note**: FastestCSV doesn't decode embedded newlines