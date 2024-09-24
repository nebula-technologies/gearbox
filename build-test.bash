#!/bin/bash

export RUST_FEATURE_FLAGS="storage-all log-tracing time time-serde"
export RUST_FEATURE_FLAGS="with_serde collections collections-const-hash-map collections-hash-map collections-simple-linked-list collections-vec-deque common common-all common-try-default common-boxed-future common-ips common-process error error-all error-tracer error-tracer-macros error-type-registry log log-tracing log-tracing-bunyan log-tracing-deeplog log-tracing-syslog log-tracing-macros log-tracing-macros-syslog log-tracing-macros-common net net-hostname path path-dirs rails rails-ext rails-tracing storage storage-all storage-web storage-io storage-yaml-ext storage-json-ext template time time-serde"
export RUST_TARGETS="x86_64-unknown-linux-gnu"

# Function to compile with specified features
compile_features() {
    feature_string=$(IFS=,; echo "$1")
    printf "($2 of $3) - Compiling with features: $feature_string\n" | tee -a compile_status.log
    echo cargo build --release --target=${target} --jobs=${CARGO_JOBS:-10} --no-default-features --features "$feature_string"
    cargo build --release --target=${target} --jobs=${CARGO_JOBS:-10} --no-default-features --features "$feature_string"
    local status=$?
    if [ $status -eq 0 ]; then
        echo " - Success..." | tee -a compile_status.log
        echo "$feature_string: SUCCESS" >> compile_status.log
    else
        echo " - Failed..." | tee -a compile_status.log
        echo "$feature_string: FAILED" >> compile_status.log
    fi
}

# Generate all combinations of features
generate_combinations() {
    local -n combis=$1  # Name reference to the result array
    local num_combinations=$((1 << ${#RUST_FEATURE_FLAGS[@]}))  # 2^n combinations

    for ((i=1; i<num_combinations; i++)); do
        local combo=()
        for ((j=0; j<${#RUST_FEATURE_FLAGS[@]}; j++)); do
            if ((i & (1 << j))); then
                combo+=("${RUST_FEATURE_FLAGS[j]}")
            fi
        done
        combis+=("$(IFS=,; echo "${combo[*]}")")
    done
}


# Build function to handle short or long combinations
build() {
    case "$1" in
        --short)
            # Add each feature individually and all together
            for feature in "${RUST_FEATURE_FLAGS[@]}"; do
                combinations+=("$feature")
            done
            combinations+=("${RUST_FEATURE_FLAGS[*]}")
            ;;
        --long)
            # Use the generate_combinations function to fill the array
            generate_combinations combinations
            ;;
        *)
            echo "Usage: $0 --long | --short"
            exit 1
            ;;
    esac

    # Prepare log and build log directories
    echo "Compilation Status on $(date)" > compile_status.log
    echo "-------------------------------------" >> compile_status.log
    mkdir -p build_logs

    # Compile for each combination stored in combinations array
    total=${#combinations[@]}

    echo "Feature flag combinations:"
    for combo in "${combinations[@]}"; do
        printf "%s, " "$combo"
    done
    echo

    printf "\n"

    count=1
    for combo in "${combinations[@]}"; do
        compile_features "$combo" $count $total
        ((count++))
    done

    # Print status report
    echo "Compilation results:"
    cat compile_status.log
}

# Utility functions for logging
print_err() { echo " ❌ $@"; }
print_cmd() { echo " ✨ $@"; }
print_info() { echo " ⅈ $@"; }
print_list_item() { echo "   ⇒ $@"; }
print_list() { print_info "${1}"; for i in "${@:2}"; do print_list_item "${i}"; done; }
print() { echo " ✅ $@"; }

# Run a command and handle errors
run() {
    print_cmd "$@";
    "$@" > /tmp/log 2> /tmp/log_err || (print_err "$@ ... failed" && cat /tmp/log_err && exit 1);
    print "$@ ... success";
}

# Main script logic
main() {
    # Set artifacts and distribution directories
    mkdir -p ${ARTIFACTS_DIR}
    mkdir -p ${DIST_DIR}
    RUST_TARGETS="${RUST_TARGETS:-$(rustup target list | grep "(installed)" | sed 's/ (installed)//g')}"

    print_list "Following Toolchains will be used:" ${RUST_TARGETS}

    for target in ${RUST_TARGETS}; do
        case $target in
            "aarch64-"*) target_arch="aarch64";;
            "arm-"*) target_arch="arm";;
            "armebv7r-"*) target_arch="armebv7r";;
            "armv5te-"*) target_arch="armv5te";;
            "armv7-"*) target_arch="armv7";;
            "armv7a-"*) target_arch="armv7a";;
            "armv7r-"*) target_arch="armv7r";;
            "asmjs-"*) target_arch="asmjs";;
            "i586-"*) target_arch="i586";;
            "i686-"*) target_arch="i686";;
            "mips-"*) target_arch="mips";;
            "mips64-"*) target_arch="mips64";;
            "mips64el-"*) target_arch="mips64el";;
            "mipsel-"*) target_arch="mipsel";;
            "nvptx64-"*) target_arch="nvptx64";;
            "powerpc-"*) target_arch="powerpc";;
            "powerpc64-"*) target_arch="powerpc64";;
            "powerpc64le-"*) target_arch="powerpc64le";;
            "riscv32i-"*) target_arch="riscv32i";;
            "riscv32imac-"*) target_arch="riscv32imac";;
            "riscv32imc-"*) target_arch="riscv32imc";;
            "riscv64gc-"*) target_arch="riscv64gc";;
            "riscv64imac-"*) target_arch="riscv64imac";;
            "s390x-"*) target_arch="s390x";;
            "sparc64-"*) target_arch="sparc64";;
            "sparcv9-"*) target_arch="sparcv9";;
            "thumbv6m-"*) target_arch="thumbv6m";;
            "thumbv7em-"*) target_arch="thumbv7em";;
            "thumbv7m-"*) target_arch="thumbv7m";;
            "thumbv7neon-"*) target_arch="thumbv7neon";;
            "thumbv8m.base-"*) target_arch="thumbv8m.base";;
            "thumbv8m.main-"*) target_arch="thumbv8m.main";;
            "wasm32-"*) target_arch="wasm32";;
            "x86_64-"*) target_arch="x86_64";;
            *) echo "Unable to determine architecture"; exit 1;;
        esac
        print_info "Architecture: ${target_arch}"

        case $target in
            *"-freebsd"*) target_os="freebsd";;
            *"-illumos"*) target_os="illumos";;
            *"-netbsd"*) target_os="netbsd";;
            *"-redox"*) target_os="redox";;
            *"-linux"*) target_os="linux";;
            *"-fortanix"*) target_os="fortanix";;
            *"-windows"*) target_os="windows";;
            *"-sun"*) target_os="sun";;
            *"-pc"*) target_os="pc";;
            *"-none"*) target_os="none";;
            *"-unknown"*) target_os="unknown";;
            *"-apple"*) target_os="apple";;
            *) target_os="";;
        esac
        print_info "Operating System: ${target_os}"

        case $target in
            *"-wasi") target_abi="wasi";;
            *"-solaris") target_abi="solaris";;
            *"-softfloat") target_abi="softfloat";;
            *"-sgx") target_abi="sgx";;
            *"-musleabihf") target_abi="musleabihf";;
            *"-musleabi") target_abi="musleabi";;
            *"-muslabi64") target_abi="muslabi64";;
            *"-musl") target_abi="musl";;
            *"-msvc") target_abi="msvc";;
            *"-ios-sim") target_abi="ios-sim";;
            *"-ios") target_abi="ios";;
            *"-gnux32") target_abi="gnux32";;
            *"-gnueabihf") target_abi="gnueabihf";;
            *"-gnueabi") target_abi="gnueabi";;
            *"-gnuabi64") target_abi="gnuabi64";;
            *"-gnu") target_abi="gnu";;
            *"-fuchsia") target_abi="fuchsia";;
            *"-freebsd") target_abi="freebsd";;
            *"-emscripten") target_abi="emscripten";;
            *"-elf") target_abi="elf";;
            *"-eabihf") target_abi="eabihf";;
            *"-eabi") target_abi="eabi";;
            *"-darwin") target_abi="darwin";;
            *"-cuda") target_abi="cuda";;
            *"-androideabi") target_abi="androideabi";;
            *"-android") target_abi="android";;
            *) target_abi="";;
        esac
        print_info "Application Binary Interface: ${target_abi}"

        rustup target add ${target}
        if [ -n "$RUST_FEATURE_FLAGS" ]; then
            # Build with the specified features
            export target=${target};
            export RUST_FEATURE_FLAGS=($RUST_FEATURE_FLAGS)
            build --short;
        else
            # Build without specifying features
            cargo build --release --target=${target} --jobs=${CARGO_JOBS:-10};
        fi


    done
}

# Invoke the main logic
main "$@"
