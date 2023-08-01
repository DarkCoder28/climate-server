#!/Windows/System32/WindowsPowerShell/v1.0/powershell.exe

echo "Resetting build to MASTER"

git fetch origin
git reset --hard origin/main
git pull

echo "Building Rust Project"

cargo build -r

echo "Building dockerfile image tar for Portainer"

mkdir docker
cp ./target/x86_64-unknown-linux-musl/release/climate-server ./docker/
cp ./Dockerfile ./docker/
tar -cvf image.tar -C ./docker *

echo "Done!"
echo "Your tarball for Portainer to build the image is located at: ./image.tar"