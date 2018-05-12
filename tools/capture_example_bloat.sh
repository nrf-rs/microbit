#!/bin/bash

filename="bloat_log_"`date -Iminutes`".txt"

for i in `find examples -name "*.rs"`; do
        name=$(echo $i | sed -e "s,examples/,,g" -e "s,\.rs,,g")
        echo "Processing example $name"
        echo >>$filename
        echo "Bloat for example $name" >>$filename
        cargo bloat --release --example $name >>$filename
done

echo "Captures bloat for all examples into $filename"
