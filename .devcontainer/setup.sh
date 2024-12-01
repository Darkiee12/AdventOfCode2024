get_valid_year() {
    current_year=$(date +%Y)
    while true; do
        read -p "Enter a valid year (between 2015 and $current_year): " year
        if [[ $year =~ ^[0-9]{4}$ ]] && [ "$year" -ge 2015 ] && [ "$year" -le "$current_year" ]; then
            echo "$year"
            break
        else
            echo "Invalid input. Please enter a year between 2015 and $current_year."
        fi
    done
}

year=$(get_valid_year)
workspace="aoc$year"

if [ ! -d "$workspace" ]; then
    echo "Initializing Advent of Code $year"
    
    cargo init ${workspace} || exit 1
    cd "${workspace}" || exit 1

    cargo install cargo-aoc || exit 1
    cargo add aoc-runner aoc-runner-derive || exit 1
    clear

    for i in {1..25}; do
        touch src/day$i.rs
        echo "pub mod day$i;" >> src/lib.rs
    done

    echo "aoc_runner_derive::aoc_main! { lib = $workspace }" > src/main.rs
    echo "aoc_runner_derive::aoc_lib! { year = $year }" >> src/lib.rs

    echo "Please enter your Advent of Code session cookie:"
    read -sp "Cookie: " cookie
    echo
    cargo aoc credentials "$cookie" || exit 1
    cargo aoc input || exit 1

    echo "Happy coding!"
else
    echo "Folder already exists! Happy coding!"
fi
