# DO NOT SUBMIT: manually edited
#
LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := vm_memory
MODULE_SRCS := \
	$(LOCAL_DIR)/src/lib.rs \

MODULE_RUST_EDITION := 2021
MODULE_RUSTFLAGS += \
	--cfg 'feature="default"' \

MODULE_LIBRARY_DEPS := \
	$(call FIND_CRATE,libc) \
	$(call FIND_CRATE,thiserror) \

include make/library.mk
