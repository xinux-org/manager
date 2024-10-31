# https://zero-to-nix.com/concepts/reproducibility
{ stdenv, fetchGit }:
stdenv.mkDerivation {
  name = "my-package";
  src = fetchgit {
    url = "https://github.com/DeterminateSystems/riff";
    sha256 = "sha256-7mnx7J0AacL2P2mNuNNB+kKE7VR8nniVG+PSrwpZixE=";
  };
}
