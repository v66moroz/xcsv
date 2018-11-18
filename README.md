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
XCSV.open("foo.csv") do |csv_reader|
  csv_reader.each do |rec|
    rec #=> [col1, col2, col3, ...]
  end
end

XCSV.open("foo.csv") do |csv_reader|
  csv_reader.take(10).to_a #=> [[col1, ...], [col1, ...], ...]
end

# while loop
XCSV.open("bar.csv") do |csv_reader|
  while (rec = csv_reader.next) do
    rec #=> [col1, col2, col3, ...]
  end
end

# Both forms will gunzip if file name ends with .gz
XCSV.open("foo_bar.csv.gz") do |csv_reader|
  while (rec = csv_reader.next) do
    rec #=> [col1, col2, col3, ...]
  end
end
```

### Benchmarks

#### Code

```ruby
# FastestCSV
FastestCSV.foreach('sample.csv') do |rec|
end

# XCSV
XCSV.open('sample.csv') do |csv_reader|
  while (rec = csv_reader.next) do
  end
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