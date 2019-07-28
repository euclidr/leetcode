# Read from the file file.txt and print its transposed content to stdout.
awk '{cols = NF; for (i=1; i<= NF; i=i+1) line[i] = line[i]$i" "} END { for (i=1; i<= NF; i=i+1) { gsub(/[ ]+$/, "", line[i]); print line[i]} }' file.txt