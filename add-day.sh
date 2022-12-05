#!/bin/bash

daynumber=$(printf "%02d" $1)

# copy the template day
cp -n src/solutions/day00.rs src/solutions/day$daynumber.rs
touch src/solutions/inputs/day$daynumber.txt

# add line to the mod.rs file if it doesn't exist
if ! grep -Fxq "pub mod day$daynumber;" src/solutions/mod.rs
then
  echo "pub mod day$daynumber;" >> src/solutions/mod.rs
else
  echo "Entry for 'day$daynumber' already existed in mod.rs"
fi

# add line to the main.rs file for the day
if ! grep -Fxq "    $1 => solve!(day$daynumber)," src/main.rs
then
  sed -i "/\/\/ new days go here/c\    $1 => solve!(day$daynumber),\n    \/\/ new days go here" src/main.rs
else
  echo "Entry for 'day$daynumber' already existed in main.rs"
fi

# replace the input file in the new day
sed -i "s/inputs\/day01.txt/inputs\/day$daynumber.txt/g" src/solutions/day$daynumber.rs