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

fetch_aoc_inputs() {
    local year=$1
    local current_year=$(date +%Y)
    echo "Fetching Advent of Code inputs..."
    if [ "$year" -ne "$current_year" ]; then
        echo "Fetching inputs for all days in $year..."
        for day in {1..25}; do
            cargo aoc input -d $day -y $year || exit 1
            echo "Day $day inputs fetched successfully!"
        done
    else
      
        local current_day=$(date +%d)
        local max_day=$((current_day < 25 ? current_day : 25))
        echo "Fetching inputs for all days until $max_day in $year..."
        for day in $(seq 1 $max_day); do
            cargo aoc input -d $day -y $year || exit 1
        done
    fi
    echo "Advent of Code inputs have been fetched successfully!"
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
    echo "Creating folders and files..."
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
    
    fetch_aoc_inputs $year
    echo "Advent of Code $year has been initialized successfully!"
    echo "Happy coding!"
else
    echo "Folder already exists! Happy coding!"
fi