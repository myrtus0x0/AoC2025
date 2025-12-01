#!/bin/bash

if [[ $1 =~ [0-9]{1,2} ]]; then
  mkdir -p "day_$1"
  echo "Directory for day $1 created!"
  echo "Changing to the newly created directory..."
  cp -r template/ ./day_$1/
  cd ./day_$1/
  find . -type f -name "Cargo.toml" -exec sed -i '' "s/day/day_$1/g" {} +
  find . -type f -name "Cargo.lock" -exec sed -i '' "s/day/day_$1/g" {} +

else
  echo "Directory not created, invalid input!"
fi
