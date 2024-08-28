#!/bin/bash

# Check if a file name is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <filename>"
  exit 1
fi

# Check if the file exists
if [ ! -f "$1" ]; then
  echo "File not found!"
  exit 1
fi

# Original file name
FILE=$1

# Function to measure time and compress
measure_time() {
  local tool=$1
  local extension=$2
  local command=$3

  echo "Compressing with $tool..."
  start_time=$(date +%s%N)
  eval "$command"
  status=$?
  end_time=$(date +%s%N)
  elapsed=$((end_time - start_time))
  
  if [ $status -eq 0 ]; then
    echo "$tool compression successful: $FILE.$extension"
  else
    echo "$tool compression failed!"
  fi

  echo "$tool: $elapsed nanoseconds" >> timings.txt
}

# Clear timings file
> timings.txt

# Compress the file using bzip2
measure_time "bzip2" "bz2" "bzip2 -k \"$FILE\""

# Compress the file using lzma
measure_time "lzma" "lzma" "lzma -k \"$FILE\""

# Compress the file using gzip
measure_time "gzip" "gz" "gzip -k \"$FILE\""

# Compress the file using zstd
measure_time "zstd" "zst" "zstd -k \"$FILE\""

# Compress the file using lz4
measure_time "lz4" "lz4" "lz4 -k \"$FILE\""

# Compress the file using xz
measure_time "xz" "xz" "xz -k \"$FILE\""

# Print the timings
echo "Compression timings (in nanoseconds):"
cat timings.txt

# Clean up
rm timings.txt
