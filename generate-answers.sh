for i in $(seq -f "%02g" 1 25); do

	f="./src/day$i.rs"
	[[ -f $f ]] && continue

	# rust
	cat << EOF > "$f"
pub fn main() {
    let contents = include_str!("../input/day-$i-input.txt");
}
EOF

done
