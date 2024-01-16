#!/bin/bash
# wrapper_script.sh

# First argument is the cgroup name
CGROUP_NAME=$1

# Second argument is the executable
EXECUTABLE=$2

# Add the current shell process to the specified cgroup
echo $$ > /sys/fs/cgroup/net_cls/$CGROUP_NAME/tasks

# Shift the first two arguments (cgroup and executable)
# so that $@ contains only the remaining arguments
shift 2

# Execute the intended command with all remaining arguments
exec $EXECUTABLE "$@"
