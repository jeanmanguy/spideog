# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross build --target $TARGET --release
    # cross rustc --bin spideog --target $TARGET --release -- -C lto
    
    if [[ $TARGET == *"pc-windows"* ]]; then
        cp target/$TARGET/release/spideog.exe $stage/
    else 
        cp target/$TARGET/release/spideog $stage/
    fi

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
