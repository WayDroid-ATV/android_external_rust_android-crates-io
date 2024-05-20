#!/bin/sh

set -e
set -x

ANDROID_BUILD_TOP=$(realpath $(dirname $0)/../../..)
pushd $ANDROID_BUILD_TOP

bash -c "source build/envsetup.sh && lunch aosp_cf_x86_64_phone-trunk_staging-userdebug && m cargo_embargo bpfmt"

PATH=$ANDROID_BUILD_TOP/out/host/linux-x86/bin:$PATH

for CONFIG in $(find external/rust/crates -name cargo_embargo.json) ; do
    CRATE_DIR=$(dirname $CONFIG)
    if [ ! -f $CRATE_DIR/Android.bp ] ; then
        continue
    fi
    repo start cargo-embargo-all $CRATE_DIR
    pushd $CRATE_DIR
    ANDROID_BUILD_TOP=$ANDROID_BUILD_TOP $ANDROID_BUILD_TOP/out/host/linux-x86/bin/cargo_embargo generate cargo_embargo.json
    if ! git diff --exit-code Android.bp ; then
        git add Android.bp
        git commit -m "Update Android.bp by running cargo_embargo"$'\n'$'\n'"Test: ran cargo_embargo"
    fi
    rm -rf cargo.out cargo.metadata target.tmp rules.mk Cargo.lock Android.bp.orig
    git restore .
    popd
done

popd

# repo upload -t --br=cargo-embargo-all --re=srhines@google.com