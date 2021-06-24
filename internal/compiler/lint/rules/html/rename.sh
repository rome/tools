for file in *.rjson; do
    mv "$file" "$(basename "$file" .rjson).toml"
done


