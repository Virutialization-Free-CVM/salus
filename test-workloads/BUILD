# SPDX-FileCopyrightText: 2023 Rivos Inc.
#
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_library", "rustfmt_test")
load("//:objcopy.bzl", "objcopy_to_bin")

objcopy_to_bin(
    name = "tellus_raw",
    src = ":tellus",
)

objcopy_to_bin(
    name = "guestvm_raw",
    src = ":guestvm",
)

objcopy_to_bin(
    name = "tellus_wasmrt_raw",
    src = ":tellus_wasmrt",
)

objcopy_to_bin(
    name = "wasmrt_guest_raw",
    src = ":wasmrt_guest",
)

sh_binary(
    name = "create_guest_image",
    srcs = ["create_guest_image/create_guest_image.sh"],
)

# from salus/test-workloads/src/consts.rs
NUM_TELLUS_IMAGE_PAGES = 512

PAGE_SIZE_4K = 4096

max_tellus_size = NUM_TELLUS_IMAGE_PAGES * PAGE_SIZE_4K

genrule(
    name = "tellus_guestvm_rule",
    srcs = [
        ":tellus_raw",
        ":guestvm_raw",
    ],
    outs = ["tellus_guestvm"],
    cmd = "$(location :create_guest_image) $(SRCS) $@ " + str(max_tellus_size),
    tools = [":create_guest_image"],
)

genrule(
    name = "tellus_wasmrt_rule",
    srcs = [
        ":tellus_wasmrt_raw",
        ":wasmrt_guest_raw",
    ],
    outs = ["tellus_wasmrt_guest"],
    cmd = "$(location :create_guest_image) $(SRCS) $@ " + str(max_tellus_size),
    tools = [":create_guest_image"],
)

rust_binary(
    name = "tellus",
    srcs = [
        "src/bin/tellus.rs",
        "src/consts.rs",
    ],
    crate_root = "src/bin/tellus.rs",
    data = glob([
        "src/**/*.S",
    ]),
    linker_script = "tellus.lds",
    proc_macro_deps = ["@salus-index//:hex-literal"],
    rustc_flags = [
        "--codegen=link-arg=-nostartfiles",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "//sync",
        "//test-system",
        "//test-workloads",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_binary(
    name = "tellus_wasmrt",
    srcs = [
        "src/bin/tellus_wasmrt.rs",
    ],
    crate_root = "src/bin/tellus_wasmrt.rs",
    data = glob([
        "src/**/*.S",
    ]),
    linker_script = "tellus.lds",
    rustc_flags = [
        "--codegen=link-arg=-nostartfiles",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//s-mode-utils",
        "@liberum_parent//liberum-tellus-wasmrt",
    ],
)

rust_binary(
    name = "guestvm",
    srcs = [
        "src/bin/guestvm.rs",
        "src/consts.rs",
    ],
    crate_root = "src/bin/guestvm.rs",
    data = glob([
        "src/**/*.S",
        "src/**/*.der",
    ]),
    linker_script = "guest.lds",
    proc_macro_deps = ["@salus-index//:hex-literal"],
    rustc_flags = [
        "--codegen=link-arg=-nostartfiles",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "//test-system",
        "//test-workloads",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_binary(
    name = "wasmrt_guest",
    srcs = [
        "src/bin/wasmrt_guest.rs",
        "src/consts.rs",
    ],
    crate_root = "src/bin/wasmrt_guest.rs",
    data = glob([
        "src/**/*.S",
    ]),
    linker_script = "guest.lds",
    rustc_flags = [
        "--codegen=link-arg=-nostartfiles",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//test-workloads",
        "//s-mode-utils",
        "@liberum_parent//liberum-wasmrt-guest",
    ],
)

rust_library(
    name = "test-workloads",
    srcs = glob(["src/*.rs"]),
    data = glob([
        "src/**/*.S",
        "src/**/*.der",
    ]),
    proc_macro_deps = ["@salus-index//:hex-literal"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_clippy(
    name = "clippy",
    deps = [
        "guestvm",
        "tellus",
        "tellus_wasmrt",
        "test-workloads",
        "wasmrt_guest",
    ],
)

rustfmt_test(
    name = "rustfmt",
    targets = [
        "test-workloads",
        "tellus",
        "tellus_wasmrt",
        "guestvm",
        "wasmrt_guest",
        "create_guest_image",
    ],
)
