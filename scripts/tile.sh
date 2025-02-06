cd output

for file in *.png; do
  convert +append "$file" "$file" "$file"
done

for file in *.png; do
  convert -append "$file" "$file" "$file"
done

cd -
