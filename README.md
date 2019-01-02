# azure-ost-cli
A CLI front-end for azure-ost-core

For information on AzureOST, see https://github.com/CerulanLumina/azureost-core-rs.

### Compiling
In order to acquire a binary, clone or download the repository onto your
computer, and enter the folder.

Ex:
```sh
git clone https://github.com/CerulanLumina/azure-ost-cli.git
cd azure-ost-cl
```
Now, run `cargo build --release`, and your binary should be in the
`target/release/` directory.

If you'd like to have MP3 support, acquire a copy of libmp3lame and compile with
the `lamemp3` feature enabled.

On Linux distributions using `apt`:
```sh
sudo apt install libmp3lame-dev
cargo build --release --features "lamemp3"
```
On Windows, LAME libraries can be acquired from 
[RareWares](http://www.rarewares.org/mp3-lame-libraries.php). Place the included
libmp3lame.lib in the deps folder of release (`target/release/deps`), or
somewhere on your PATH. Rename it to `libmp3lame.a`. Now you can run
`cargo build --release --features "lamemp3"`. When the build process is complete,
place a copy of `libmp3lame.dll` in the executable's path, or somewhere on your
system PATH.
