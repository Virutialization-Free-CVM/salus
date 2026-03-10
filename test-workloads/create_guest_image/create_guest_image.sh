#!/bin/bash
# SPDX-FileCopyrightText: 2026 The Salus Contributors
#
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

if [[ $# -ne 4 ]]; then
    echo "usage: $0 <tellus> <guest> <output> <max_tellus_size>" >&2
    exit 1
fi

tellus_path=$1
guest_path=$2
output_path=$3
max_tellus_size=$4

tellus_size=$(stat -c %s "${tellus_path}")
if (( tellus_size > max_tellus_size )); then
    echo "Tellus longer than Max Tellus size" >&2
    exit 1
fi

rm -f "${output_path}"
cp "${tellus_path}" "${output_path}"
chmod u+w "${output_path}"
truncate -s "${max_tellus_size}" "${output_path}"
dd if="${guest_path}" of="${output_path}" bs=1 seek="${max_tellus_size}" conv=notrunc status=none
