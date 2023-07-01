#!/bin/bash

function build() {
    source .env
    if [[ -z $RUNSLATE_PUBLISH_ALIAS ]]; then
        RUNSLATE_PUBLISH_ALIAS="${HOME}/.cargo/bin/"
    fi

    cargo build --release
    cp target/release/runslate "${RUNSLATE_PUBLISH_ALIAS}"
}

function update_version() {
    if [ "$1" -lt 1 ] || [ "$1" -gt 3 ]; then
        echo 'Invalid version.'
        return 1
    fi

    # exit if `version` more than one line.
    version_count=$(grep -cE '^version = .+?"$' Cargo.toml)
    if [ "$version_count" -ne 1 ]; then
        echo Parse version error: multiple version description.
        exit 1
    else
        # version = "0.1.0"
        old_version_line=$(grep -E '^version = .+?"$' Cargo.toml)

        # 0.1.0
        # gnu grep grep -oP '(?<=version = ")[0-9\.]+(?="$)'
        # fuck you macOS and BSD series
        old_version=$(echo "$old_version_line" |
            grep -E '^version = .+?"$' Cargo.toml |
            grep -oE '(?:").*(?:")' |
            sed 's/"//g')

        # [0 1 0]
        # versions=($(echo $old_version | tr '.' "\n"))
        IFS='.' read -r -a versions <<<"${old_version}"

        # $1=3; [0 1 1]
        case $1 in
        1)
            ((versions[0]++))
            versions[1]=0
            versions[2]=0
            ;;
        2)
            ((versions[1]++))
            versions[2]=0
            ;;
        3)
            ((versions[2]++))
            ;;
        *)
            exit 1
            ;;
        esac

        # 0.1.1.
        printf -v new_version_tmp '%d.' "${versions[@]}"

        # 0.1.1
        new_version="${new_version_tmp%.}"

        # version = "0.1.1"
        new_version_line=$(echo "${old_version_line}" | sed "s/${old_version}/${new_version}/")

        # sed -i "s/^${old_version_line}$/${new_version_line}/" 'Cargo.toml'
        sed -i '' "s/^${old_version_line}$/${new_version_line}/" 'Cargo.toml'
    fi
}

function help() {
    echo "    -h update main version."
    echo "    -m update feature version."
    echo "    -l update fix version."
}

function main() {
    if [ ! -f Cargo.toml ]; then
        echo Cargo.toml not exist.
        exit 1
    fi

    case $1 in
    '-h')
        update_version 1
        ;;
    '-m')
        update_version 2
        ;;
    '-l')
        update_version 3
        ;;
    'help')
        help
        exit 0
        ;;
    esac

    build
}

main "$@"
