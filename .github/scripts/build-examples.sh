#!/usr/bin/env bash

_status=0

for example in $(ls examples | sed s/\.rs$//); do
  output=$(cargo build --target=thumbv6m-none-eabi --example=$example --color=always 2>&1)
  result=$?

  if [[ $result == 0 ]]; then
    echo "::group::✅ $example ok"
  else
    echo "::group::💥 $example fail"
    _status=1
  fi
  echo "$output"
  echo "::endgroup::"
done

exit $_status
