#!/bin/bash

name=`echo $1 | sed -r 's/([a-z0-9])([A-Z])/\1_\L\2/g'`
path="src/bin/${name}.rs"

touch "${path}"
echo "file created at ${path}"
code "${path}"
