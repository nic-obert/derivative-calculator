#!/bin/sh

echo $'\nRust:'
find . -type f -name "*.rs" ! -wholename "**/target/*" | xargs wc -l | sort -nr
echo 
