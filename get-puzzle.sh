today=$(date '+%d' | sed 's/^0//')
day=${1:-$today}

# TODO: fix this
# w3m -header "Cookie: $AOC_TOKEN" "https://adventofcode.com/2024/day/$day" |
# 	tee "day-$day.txt"

source .env
curl -sL "https://adventofcode.com/2024/day/$day/input" -H "Cookie: session=$AOC_TOKEN" |
	tee "./input/day-$day-input.txt"
