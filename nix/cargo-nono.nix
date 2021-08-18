{ lib, rustPlatform, applyPatches, fetchFromGitHub }:

rustPlatform.buildRustPackage rec {
  pname = "cargo-nono";
  version = "0.1.9";

  src = applyPatches rec { inherit (src) name; src = fetchFromGitHub {
    owner = "hobofan";
    repo = pname;
    rev = "${version}";
    sha256 = "07iaxpr0bhmikpl5zg646n5zzh60zbxfp1c1n4k2j2bzzfmxz93q";
  }; patches = [ (builtins.toFile "cargo-nono-lock-version.patch" /*diff*/''
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -44,7 +44,7 @@ source = "registry+https://github.com/rust-lang/crates.io-index"

 [[package]]
 name = "cargo-nono"
-version = "0.1.8"
+version = "0.1.9"
 dependencies = [
  "assert_cmd 0.11.1 (registry+https://github.com/rust-lang/crates.io-index)",
  "cargo_metadata 0.8.2 (registry+https://github.com/rust-lang/crates.io-index)",
  '') ]; };

  cargoSha256 = "00cmqzp9mwwvwkg49knjwkwb0znkmnqmikznhnc2n87iy4xrfl38";

  patches = [
    (builtins.toFile "cargo-nono-cargo_metadata-command.patch" /*diff*/''
--- a/src/main.rs
+++ b/src/main.rs
@@ -12,7 +12,7 @@ use crate::check_source::*;
 use crate::ext::*;
 use crate::util::*;
 
-use cargo_metadata::{Metadata, Package};
+use cargo_metadata::{CargoOpt, Metadata, MetadataCommand, Package};
 
 pub static SUCCESS: Emoji = Emoji("✅  ", "SUCCESS");
 pub static FAILURE: Emoji = Emoji("❌  ", "FAILURE");
@@ -133,8 +133,8 @@ fn main() {
 
     let matches = app.clone().get_matches();
     if let Some(matches) = matches.subcommand_matches("check") {
-        let metadata_full = metadata_run(Some("--all-features".to_owned())).unwrap();
-        let metadata = metadata_run(None).unwrap();
+        let metadata_full = MetadataCommand::new().features(CargoOpt::AllFeatures).exec().unwrap();
+        let metadata = MetadataCommand::new().exec().unwrap();
 
         let target_workspace_member =
             main_ws_member_from_args(&metadata, matches.value_of("package"));
--- a/src/util.rs
+++ b/src/util.rs
@@ -1,25 +1,8 @@
 use std::env;
-use std::process::Command;
-use std::str::from_utf8;
 use cargo_metadata::{Dependency, Metadata, Package, PackageId};
 
 use crate::ext::{Feature, FeatureCause};
 
-pub fn metadata_run(additional_args: Option<String>) -> Result<Metadata, ()> {
-    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
-    let mut cmd = Command::new(cargo);
-    cmd.arg("metadata");
-    cmd.args(&["--format-version", "1"]);
-    if let Some(additional_args) = additional_args {
-        cmd.arg(&additional_args);
-    }
-
-    let output = cmd.output().unwrap();
-    let stdout = from_utf8(&output.stdout).unwrap();
-    let meta = serde_json::from_str(stdout).unwrap();
-    Ok(meta)
-}
-
 pub fn features_from_args(
     package_id: String,
     no_default: bool,
    '')
    (builtins.toFile "cargo-nono-tests-align-locks.nix" /*diff*/''
--- a/tests/dependency_default_std/Cargo.lock
+++ b/tests/dependency_default_std/Cargo.lock
@@ -4,13 +4,13 @@
 name = "dependency_default_std"
 version = "0.1.0"
 dependencies = [
- "serde 1.0.80 (registry+https://github.com/rust-lang/crates.io-index)",
+ "serde 1.0.116 (registry+https://github.com/rust-lang/crates.io-index)",
 ]
 
 [[package]]
 name = "serde"
-version = "1.0.80"
+version = "1.0.116"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 
 [metadata]
-"checksum serde 1.0.80 (registry+https://github.com/rust-lang/crates.io-index)" = "15c141fc7027dd265a47c090bf864cf62b42c4d228bbcf4e51a0c9e2b0d3f7ef"
+"checksum serde 1.0.116 (registry+https://github.com/rust-lang/crates.io-index)" = "96fe57af81d28386a513cbc6858332abc6117cfdb5999647c6444b8f43a370a5"
    '')
    (builtins.toFile "cargo-nono-tests-isatty-false.patch" /*diff*/''
--- a/tests/crate_itself.rs
+++ b/tests/crate_itself.rs
@@ -28,6 +28,6 @@ fn it_prints_checkmark() {
         .stdout;
     let output = String::from_utf8(output).unwrap();

-    let expected_cause = "crate_itself_fixed_no_std: ✅";
+    let expected_cause = "crate_itself_fixed_no_std: SUCCESS";
     assert!(output.contains(expected_cause));
 }
--- a/tests/crate_itself_not_test_no_std.rs
+++ b/tests/crate_itself_not_test_no_std.rs
@@ -28,6 +28,6 @@ fn it_prints_checkmark() {
         .stdout;
     let output = String::from_utf8(output).unwrap();

-    let expected_cause = "crate_itself_not_test_no_std: ✅";
+    let expected_cause = "crate_itself_not_test_no_std: SUCCESS";
     assert!(output.contains(expected_cause));
 }
    '')
  ];

  meta = with lib; {
    description = "A Cargo subcommand that checks your crate for (possible) no_std compatibility";
    homepage = "https://github.com/hobofan/cargo-nono";
    license = [ licenses.mit licenses.asl20 ];
    platforms = platforms.unix;
    maintainers = with maintainers; [ bb010g ];
  };
}
