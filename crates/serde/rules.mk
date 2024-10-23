# DO NOT SUBMIT: manually edited
#
LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := serde
MODULE_SRCS := \
	$(LOCAL_DIR)/src/lib.rs \

MODULE_RUST_EDITION := 2015
MODULE_RUSTFLAGS += \
	--cfg 'feature="alloc"' \
	--cfg 'feature="default"' \
	--cfg 'feature="derive"' \
	--cfg 'feature="serde_derive"' \

ifeq ($(call TOBOOL,$(TRUSTY_USERSPACE)),true)

MODULE_RUSTFLAGS += \
	--cfg 'feature="std"' \

endif

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	$(call FIND_CRATE,serde_derive) \

include make/library.mk
