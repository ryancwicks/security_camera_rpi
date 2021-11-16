#!/bin/bash -e

# Set your timezone here.
tz="America/Toronto"

container_name="hardware_control"
common_docker_run_args=(
        # This avoids errors with broken seccomp on Raspberry Pi OS.
        --security-opt=seccomp:unconfined

        # docker's default log driver won't rotate logs properly, and will throw
        # away logs when you destroy and recreate the container. Using journald
        # solves these problems.
        # https://docs.docker.com/config/containers/logging/configure/
        --log-driver=journald
        --log-opt="tag=hardware_control"
        --privilidged
        --env=RUST_BACKTRACE=1
        --env=TZ=":${tz}"
)


case "$1" in 
run)
    exec docker run \
        "${common_docker_run_args[@]}" \
        --network=host \
        "${container_name}" \
        
        ;;
start|stop|logs|rm)
        exec docker "$@" "${container_name}"
        ;;
build)
        exec docker "$@" -t "${container_name}" .
        ;;
*)
        exec docker run \
                --interactive=true \
                --tty \
                --rm \
                "${common_docker_run_args[@]}" \
                "$@"
        ;;
esac

