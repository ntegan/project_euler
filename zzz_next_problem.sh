#!/bin/bash
#!/bin/bash

digits=(
    "" one two three four five six seven eight nine
    ten eleven twelve thirteen fourteen fifteen sixteen seventeen eightteen nineteen
)
tens=("" "" twenty thirty forty fifty sixty seventy eighty ninety)
units=("" thousand million billion trillion)

number2words() {
    local -i number=$((10#$1))
    local -i u=0
    local words=()
    local group

    while ((number > 0)); do
        group=$(hundreds2words $((number % 1000)) )
        [[ -n "$group" ]] && group="$group ${units[u]}"

        words=("$group" "${words[@]}")

        ((u++))
        ((number = number / 1000))
    done
    echo "${words[*]}"
}

hundreds2words() {
    local -i num=$((10#$1))
    if ((num < 20)); then
        echo "${digits[num]}"
    elif ((num < 100)); then
        echo "${tens[num / 10]} ${digits[num % 10]}"
    else
        echo "${digits[num / 100]} hundred $("$FUNCNAME" $((num % 100)) )"
    fi
}

with_commas() {
    # sed -r ':a;s/^([0-9]+)([0-9]{3})/\1,\2/;ta' <<<"$1"
    # or, with just bash
    while [[ $1 =~ ^([0-9]+)([0-9]{3})(.*) ]]; do
        set -- "${BASH_REMATCH[1]},${BASH_REMATCH[2]}${BASH_REMATCH[3]}"
    done
    echo "$1"
}

num_to_words() {
    arg=$1
    [[ $arg == *[^0-9]* ]] && result="NaN" || result=$(number2words "$arg")
    #printf "%s\t%s\n" "$(with_commas "$arg")" "$result"
    echo "$result"
}






url="https://projecteuler.net/minimal="
probnum=0
lastprob=0
last=""
next_prob_dir=""

for i in {a..f}
do
    for j in {a..z}
    do
        probnum=$((probnum + 1))
        file="$(compgen -G "$i$j*")"
        if [[ "$file" ]]
        then
            lastprob=$probnum
            last="$file"
        else
            [[ "$next_prob_dir" == "" ]] && \
                next_prob_dir="$i$j""_$(num_to_words $((lastprob + 1)))"
        fi
    done
done












## No idea what any of that is above, but now have
## $last == the most recent problem in the directory

## and $next_prob_dir where to put the next/new problem

[[ ! "$last" ]] && echo uh oh && exit
[[ ! "$next_prob_dir" ]] && echo uh oh && exit

next_prob_dir="$(echo $next_prob_dir)"

cp -r $last $next_prob_dir
wget -o /dev/null -O "$next_prob_dir""/info.html" "$url""$((lastprob + 1))"


# sed 's/\]/\t''"ag_seven"'',\n\]/g' Cargo.toml 
#echo DBSERVERNAME     xxx | sed -rne 's/(dbservername)\s+\w+/\1 yyy/gip'
#DBSERVERNAME yyy



# add to workspace
sed -i 's/^\]/    "'$next_prob_dir'",\n\]/g' Cargo.toml 
sed  -i 's/name = ".*"/name = "'$next_prob_dir'"/g' $next_prob_dir/Cargo.toml 
