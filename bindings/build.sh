#   Copyright 2023 The Tari Project
#   SPDX-License-Identifier: BSD-3-Clause

cargo test --workspace --exclude integration_tests export_bindings --features ts
shx mv ../dan_layer/bindings/src/types/* ./src/types/
shx rm -rf ../dan_layer/bindings/
DIRECTORY_PATH="./src/types" # replace with your directory path
HELPERS_PATH="./src/helpers" # replace with your directory path
INDEX_FILE="./index.ts"

# Remove the index file if it exists
if [ -f "$INDEX_FILE" ]; then
  rm "$INDEX_FILE"
fi

# Add the license header
echo "//   Copyright 2023 The Tari Project" >> $INDEX_FILE
echo "//   SPDX-License-Identifier: BSD-3-Clause" >> $INDEX_FILE
echo "" >> $INDEX_FILE

# Generate the index file
for file in $(find $DIRECTORY_PATH -name "*.ts"); do
  FILE_NAME=$(basename $file)
  if [ "$FILE_NAME" != "index.ts" ]; then
    MODULE_NAME="${FILE_NAME%.*}"
    echo "export * from '$DIRECTORY_PATH/$MODULE_NAME';" >> $INDEX_FILE
  fi
done

# Add helpers
for file in $(find $HELPERS_PATH -name "*.ts"); do
  FILE_NAME=$(basename $file)
  if [ "$FILE_NAME" != "index.ts" ]; then
    MODULE_NAME="${FILE_NAME%.*}"
    echo "export * from '$HELPERS_PATH/$MODULE_NAME';" >> $INDEX_FILE
  fi
done

npx prettier --write "./**/*.{ts,tsx,css,json}" --log-level=warn
