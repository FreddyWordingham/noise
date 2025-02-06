cd output

for sample in *-samples.png; do
    base="${sample%-samples.png}"
    gradient="${base}-gradient.png"
    if [ -f "$gradient" ]; then
        convert +append "$sample" "$gradient" "${base}-combined.png"
    fi
done

cd -
