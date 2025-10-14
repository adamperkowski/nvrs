{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:

let
  p = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = p.name;
  inherit (p) version;

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  buildFeatures = [ "cli" ];
  cargoBuildFlags = [ "--bin" "nvrs" ];

  # Skip tests that rely on network access.
  # We're also not running cli tokio tests because they don't implement skipping functionality.
  checkPhase = ''
    runHook preCheck

    cargo test -- \
      --skip 'api::aur::request_test' \
      --skip 'api::crates_io::request_test' \
      --skip 'api::gitea::request_test' \
      --skip 'api::github::request_test' \
      --skip 'api::gitlab::request_test' \
      --skip 'api::regex::request_test'

    runHook postCheck
  '';

  meta = {
    inherit (p) description homepage;
    changelog = ./CHANGELOG.md;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ adamperkowski ];
    mainProgram = p.name;
  };
}
